<template>
  <div class="score-reader" v-if="isHandpanReady">
    <!-- Score info and playback controls -->
    <div class="score-reader__controls">
      <div class="score-info">
        <button class="back-button" @click="emit('close')" title="Back to library">
          <PhArrowLeft :size="20" weight="bold" />
        </button>
        <h2 class="score-info__title">
          {{ scoreFile?.name || scoreMetadata?.work_title || selectedScale?.name || 'Score Reader' }}
        </h2>
        <div class="score-info__meta">
          <span class="scale-badge">{{ selectedScale?.name }}</span>
          <span class="ding-badge">Ding: {{ selectedDing }}</span>
          <span class="notes-badge">{{ displayedNotes.length + 1 }} notes</span>
          <span v-if="scoreMetadata?.tempo || scoreFile?.metadata?.tempo" class="score-info__tempo">
            <PhMetronome :size="16" />
            {{ scoreMetadata?.tempo || scoreFile?.metadata?.tempo || 120 }} BPM
          </span>
        </div>
      </div>

      <PlaybackControls
        :is-playing="playback.isPlaying.value"
        :current-time="playback.currentTime.value"
        :duration="playback.duration.value"
        :speed="playback.speed.value"
        :loop-enabled="playback.loopEnabled.value"
        :loop-start="playback.loopStart.value"
        :loop-end="playback.loopEnd.value"
        :speed-options="playback.speedOptions"
        :measures-ahead="measuresAhead"
        :min-measures="MIN_MEASURES"
        :max-measures="MAX_MEASURES"
        @toggle-play="playback.togglePlay"
        @stop="playback.stop"
        @seek="playback.seek"
        @set-speed="playback.setSpeed"
        @toggle-loop="playback.toggleLoop"
        @zoom-in="handleZoomIn"
        @zoom-out="handleZoomOut"
      />
    </div>

    <!-- Handpan display with falling notes overlay -->
    <div class="score-reader__stage" ref="stageRef">
      <!-- Falling notes overlay (covers stage, notes fall toward center) -->
      <FallingNotesOverlay
        :events="scheduler.scheduledEvents.value"
        :current-time="playback.currentTime.value"
        :lead-time="leadTime"
        :trail-time="TRAIL_TIME"
        :lead-in-ms="LEAD_IN_MS"
        :note-positions="notePositions"
        :handpan-notes="allHandpanNotes"
        :handpan-center="handpanCenter"
        :fall-height="fallHeight"
        :tempo="scheduler.tempo.value"
        :time-signature="scheduler.timeSignature.value"
        :score-duration="scheduler.scoreDuration.value"
        @note-hit="handleNoteHit"
      />

      <!-- Embedded handpan visualization -->
      <div class="handpan-container top-view" ref="handpanRef">
        <div class="handpan-instrument">
          <div class="handpan-shell handpan-top-shell">
            <div class="metal-texture"></div>

            <!-- Ding note -->
            <div
              class="ding-note note"
              :class="{ 'note-hit': activeNoteIndex === 0 }"
              @click="playNoteAtIndex(0)"
            >
              <span class="note-name">{{ dingNoteDisplay }}</span>
            </div>

            <!-- Top tone fields (from selected handpan) -->
            <div
              v-for="(note, index) in topNotes"
              :key="`top-${index}`"
              class="note-wrapper"
              :class="getHitClass(index + 1)"
              :style="getNoteWrapperStyle(index + 1, topNotes.length, 'top', note)"
            >
              <div
                class="note tone-field"
                :style="getNoteInnerStyle(index + 1, topNotes.length, 'top', note)"
                @click="playNoteAtIndex(index + 1)"
              ></div>
              <span class="note-name">{{ note.note || note.calculated_note }}</span>
            </div>

            <!-- Inner tone fields -->
            <div
              v-for="(note, index) in innerNotes"
              :key="`inner-${index}`"
              class="note-wrapper"
              :class="getHitClass(topNotes.length + index + 1)"
              :style="getInnerNoteWrapperStyle(index + 1, innerNotes.length, note)"
            >
              <div
                class="note tone-field inner"
                :style="getInnerNoteInnerStyle(index + 1, innerNotes.length, note)"
                @click="playNoteAtIndex(topNotes.length + index + 1)"
              ></div>
              <span class="note-name">{{ note.note || note.calculated_note }}</span>
            </div>

            <div class="light-reflection reflection-1"></div>
            <div class="light-reflection reflection-2"></div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Show message if handpan not configured -->
  <div v-else class="score-reader score-reader--empty">
    <div class="score-reader__message">
      <PhMusicNotes :size="64" weight="duotone" />
      <h2>No Handpan Selected</h2>
      <p>Please select a scale and configure your handpan first.</p>
      <button class="action-button primary" @click="goToSelection">
        Select Handpan Scale
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { PhMetronome, PhMusicNotes, PhArrowLeft } from '@phosphor-icons/vue';
import { apiUrl } from '@/services/api';

import PlaybackControls from './PlaybackControls.vue';
import FallingNotesOverlay from './FallingNotesOverlay.vue';
import useNoteScheduler from '@/composables/useNoteScheduler';
import useScorePlayback from '@/composables/useScorePlayback';
import useHandpanSelection from '@/composables/useHandpanSelection';
import useHandpanDisplay from '@/composables/useHandpanDisplay';

// Props for external score data
const props = defineProps({
  scoreData: {
    type: Object,
    default: null
  },
  scoreId: {
    type: String,
    default: null
  },
  // File object from TabsLibrary (has metadata already)
  scoreFile: {
    type: Object,
    default: null
  }
});

// Emits
const emit = defineEmits(['close']);

// Constants
const TRAIL_TIME = 300; // 0.3 seconds behind
const LEAD_IN_MS = 3000; // 3 seconds empty gap before first note
const MIN_MEASURES = 2;
const MAX_MEASURES = 16;
const DEFAULT_MEASURES_BASE = 4; // Base measures at 120 BPM

// Reactive zoom state
const measuresAhead = ref(DEFAULT_MEASURES_BASE);

// Calculate lead time based on BPM and measures ahead
// This creates the "zoom" effect - more measures = slower falling notes
const leadTime = computed(() => {
  const bpm = scheduler.tempo.value || 120;
  const beatsPerMeasure = scheduler.timeSignature.value?.beats || 4;
  const msPerBeat = 60000 / bpm;
  const msPerMeasure = msPerBeat * beatsPerMeasure;
  return measuresAhead.value * msPerMeasure;
});

// Calculate BPM-aware default measures
// Faster songs show more measures, slower songs show fewer
// Target: ~8 seconds of lead time as baseline at any BPM
const calculateDefaultMeasures = (bpm) => {
  const beatsPerMeasure = scheduler.timeSignature.value?.beats || 4;
  const msPerBeat = 60000 / bpm;
  const msPerMeasure = msPerBeat * beatsPerMeasure;
  const targetLeadTime = 8000; // 8 seconds as target
  const measures = Math.round(targetLeadTime / msPerMeasure);
  return Math.max(MIN_MEASURES, Math.min(MAX_MEASURES, measures));
};

// Zoom handlers
const handleZoomIn = () => {
  if (measuresAhead.value > MIN_MEASURES) {
    measuresAhead.value = Math.max(MIN_MEASURES, measuresAhead.value - 1);
  }
};

const handleZoomOut = () => {
  if (measuresAhead.value < MAX_MEASURES) {
    measuresAhead.value = Math.min(MAX_MEASURES, measuresAhead.value + 1);
  }
};

// Router
const route = useRoute();
const router = useRouter();

// Refs for DOM elements
const stageRef = ref(null);
const handpanRef = ref(null);

// Get handpan selection state (singleton - shared across app)
const {
  selectedScale,
  selectedDing,
  selectedNoteCount,
  notes,
  currentStage,
  goToStage
} = useHandpanSelection();

// State
const scoreMetadata = ref(null);
const scoreLoaded = ref(false);
const activeNoteIndex = ref(null);
const lastHitHand = ref('right');
const audioCache = ref({});

// Computed position data for the falling notes overlay
const handpanCenter = ref({ x: 0, y: 0 });
const fallHeight = ref(400);

// Composables
const scheduler = useNoteScheduler();
const playback = useScorePlayback();

// Use the same handpan display logic as HandpanDisplay.vue
const {
  isReadyToDisplay,
  selectedNotes,
  topNotes,
  innerNotes,
  dingData,
  ding,
  getNoteStyle,
  playNote,
  cleanup: cleanupHandpanDisplay
} = useHandpanDisplay({
  notes,
  selectedScale,
  selectedDing,
  selectedNoteCount,
  currentStage
});

// Check if handpan is configured
const isHandpanReady = computed(() => {
  return isReadyToDisplay.value;
});

// Display value for ding (use ding from useHandpanDisplay)
const dingNoteDisplay = computed(() => {
  return ding.value || selectedDing.value || 'D';
});

// Alias for consistency with template
const displayedNotes = computed(() => selectedNotes.value || []);

// All handpan notes for lane mapping (ding first, then others)
const allHandpanNotes = computed(() => {
  const allNotes = [];

  // Add ding first (index 0)
  if (dingData.value) {
    allNotes.push({ ...dingData.value, laneIndex: 0 });
  } else {
    allNotes.push({
      note: selectedDing.value,
      calculated_note: selectedDing.value,
      note_index: 0,
      laneIndex: 0
    });
  }

  // Add all displayed notes
  displayedNotes.value.forEach((note, index) => {
    allNotes.push({ ...note, laneIndex: index + 1 });
  });

  return allNotes;
});

// Calculate positions for all notes (for falling notes overlay)
// Include scale from getNoteStyle so falling notes match handpan exactly
const notePositions = computed(() => {
  const positions = [];

  // Ding position (center) - ding has no rotation, scale ~1.0 (it's a circle)
  positions.push({ x: 0, y: 0, rotation: 0, scale: 1.0, noteIndex: 0, isDing: true });

  // Top notes positions - get scale from getNoteStyle
  topNotes.value.forEach((note, index) => {
    const pos = calculateTopNotePosition(index + 1, topNotes.value.length);
    const noteStyle = getNoteStyle(index + 1, topNotes.value.length, 1, 'top', note);
    // Extract scale from transform string "rotate(Xdeg) scale(Y)"
    const scaleMatch = noteStyle.note?.transform?.match(/scale\(([\d.]+)\)/);
    const scale = scaleMatch ? parseFloat(scaleMatch[1]) : 1.0;
    positions.push({ ...pos, scale, noteIndex: index + 1 });
  });

  // Inner notes positions - get scale from getNoteStyle
  innerNotes.value.forEach((note, index) => {
    const pos = calculateInnerNotePosition(index + 1, innerNotes.value.length);
    const noteStyle = getNoteStyle(index + 1, innerNotes.value.length, 0.7, 'inner', note);
    const scaleMatch = noteStyle.note?.transform?.match(/scale\(([\d.]+)\)/);
    const scale = scaleMatch ? parseFloat(scaleMatch[1]) : 1.0;
    positions.push({ ...pos, scale, noteIndex: topNotes.value.length + index + 1 });
  });

  return positions;
});

// Calculate top note position (extracted from getNoteWrapperStyle)
const calculateTopNotePosition = (index, total) => {
  const pairs = Math.floor(total / 2);
  const isOdd = (total % 2) !== 0;

  let angleInDegrees;

  if (index === total) {
    angleInDegrees = 0;
  } else {
    let pairNumber;
    let angleStep;

    if (isOdd) {
      pairNumber = Math.ceil(index / 2);
      angleStep = 180 / (pairs + 1);
      const side = (index % 2) === 0 ? 0 : 1;
      angleInDegrees = side === 0 ? 180 + (pairNumber * angleStep) : 180 - (pairNumber * angleStep);
    } else {
      angleStep = 180 / pairs;
      pairNumber = Math.ceil((index - 1) / 2);
      if (index !== 1) {
        const side = (index % 2) === 0 ? 0 : 1;
        angleInDegrees = side === 0 ? 180 + (pairNumber * angleStep) : 180 - (pairNumber * angleStep);
      } else {
        angleInDegrees = 180;
      }
    }
  }

  angleInDegrees = angleInDegrees % 360;
  const angleInRadians = angleInDegrees * (Math.PI / 180);
  const radius = 135;

  const x = Math.sin(angleInRadians) * radius;
  const y = -Math.cos(angleInRadians) * radius;

  // Calculate rotation for the note
  const rotationDegrees = angleInDegrees > 180 ? angleInDegrees + 90 : angleInDegrees - 90;

  return { x, y, rotation: rotationDegrees };
};

// Calculate inner note position
const calculateInnerNotePosition = (index, total) => {
  const baseAngle = 360 / total;
  const angleInDegrees = (index - 1) * baseAngle;
  const angleInRadians = angleInDegrees * (Math.PI / 180);
  const radius = 70;

  const x = Math.sin(angleInRadians) * radius;
  const y = -Math.cos(angleInRadians) * radius;

  const rotationDegrees = angleInDegrees > 180 ? angleInDegrees + 90 : angleInDegrees - 90;

  return { x, y, rotation: rotationDegrees };
};

// Note positioning - use getNoteStyle from useHandpanDisplay for consistency
// These wrapper functions match the template's expected signature

const getNoteWrapperStyle = (index, total, position, note) => {
  return getNoteStyle(index, total, 1, position, note).wrapper;
};

const getNoteInnerStyle = (index, total, position, note) => {
  return getNoteStyle(index, total, 1, position, note).note;
};

const getInnerNoteWrapperStyle = (index, total, note) => {
  return getNoteStyle(index, total, 0.7, 'inner', note).wrapper;
};

const getInnerNoteInnerStyle = (index, total, note) => {
  return getNoteStyle(index, total, 0.7, 'inner', note).note;
};

// Get hit animation class
const getHitClass = (index) => {
  if (activeNoteIndex.value === index) {
    return {
      'note-hit': true,
      [`note-hit--${lastHitHand.value}`]: true
    };
  }
  return {};
};

// Audio playback
const playNoteAtIndex = async (index) => {
  const note = allHandpanNotes.value[index];
  if (!note) return;

  const pitch = note.calculated_pitch || 60;
  await playPitch(pitch);
  triggerHitAnimation(index, 'right');
};

const playPitch = async (pitch) => {
  try {
    let audio;
    if (audioCache.value[pitch]) {
      audio = audioCache.value[pitch].cloneNode();
    } else {
      audio = new Audio(apiUrl(`/api/audio/${pitch}`));
      audioCache.value[pitch] = audio;
    }
    await audio.play();
  } catch (err) {
    console.error('Error playing audio:', err);
  }
};

const triggerHitAnimation = (index, hand) => {
  activeNoteIndex.value = index;
  lastHitHand.value = hand;

  setTimeout(() => {
    if (activeNoteIndex.value === index) {
      activeNoteIndex.value = null;
    }
  }, 400);
};

// Handle note hit from falling notes
const handleNoteHit = (event) => {
  // Play the note audio
  if (event.pitch) {
    playPitch(event.pitch);
  }

  // Trigger visual hit on handpan
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  triggerHitAnimation(noteIndex, event.hand || 'right');

  // Notify playback system
  playback.triggerNoteHit(event);
};

// Load score data
const loadScore = async (scoreId) => {
  if (!scoreId) return;

  try {
    // Step 1: Get tab details (metadata + parts list)
    const tabResponse = await fetch(apiUrl(`/api/tabs/${scoreId}`));
    if (!tabResponse.ok) throw new Error('Failed to load tab details');

    const tabData = await tabResponse.json();
    scoreMetadata.value = tabData.metadata;

    // Step 2: Get the first part's measures (contains the actual notes)
    if (!tabData.parts || tabData.parts.length === 0) {
      console.warn('No parts in this score');
      return;
    }

    const partId = tabData.parts[0].id;
    const measuresResponse = await fetch(apiUrl(`/api/tabs/${scoreId}/parts/${partId}/measures`));
    if (!measuresResponse.ok) throw new Error('Failed to load measures');

    const measures = await measuresResponse.json();

    // Build score_data structure expected by scheduler
    const scoreData = {
      parts: [{
        id: partId,
        name: tabData.parts[0].name,
        measures: measures
      }]
    };

    // Schedule the score with selected handpan notes
    scheduler.scheduleScore(scoreData, allHandpanNotes.value, {
      bpm: tabData.metadata?.tempo || 120
    });

    // Set playback duration
    playback.setDuration(scheduler.scoreDuration.value);
    scoreLoaded.value = true;

    // Set BPM-aware default zoom
    const bpm = tabData.metadata?.tempo || 120;
    measuresAhead.value = calculateDefaultMeasures(bpm);

    console.log(`Loaded score: ${tabData.metadata?.work_title}, ${measures.length} measures, ${scheduler.eventCount.value} events, zoom: ${measuresAhead.value}m`);
  } catch (err) {
    console.error('Error loading score:', err);
  }
};

// Navigate to handpan selection
const goToSelection = () => {
  goToStage(2);
  router.push('/');
};

// Preload audio for all handpan notes
const preloadAudio = () => {
  allHandpanNotes.value.forEach(note => {
    if (note.calculated_pitch) {
      const audio = new Audio(apiUrl(`/api/audio/${note.calculated_pitch}`));
      audio.preload = 'auto';
      audioCache.value[note.calculated_pitch] = audio;
    }
  });
};

// Update layout measurements
const updateLayoutMeasurements = () => {
  if (stageRef.value) {
    // Calculate fall height - handpan is at bottom (30px padding + ~180px radius)
    // Use most of the remaining height for the piano roll
    fallHeight.value = Math.max(400, stageRef.value.clientHeight - 250);
  }
};

// Watch for handpan changes to reload audio
watch(allHandpanNotes, () => {
  preloadAudio();
}, { deep: true });

// Initialize
onMounted(() => {
  const scoreId = props.scoreId || route.params.scoreId;

  if (isHandpanReady.value) {
    preloadAudio();

    // Wait for DOM to be ready then measure
    nextTick(() => {
      updateLayoutMeasurements();
    });

    if (props.scoreData) {
      // Use provided score data
      scoreMetadata.value = props.scoreData.metadata;
      const bpm = props.scoreData.metadata?.tempo || 120;
      scheduler.scheduleScore(props.scoreData.score_data, allHandpanNotes.value, {
        bpm
      });
      playback.setDuration(scheduler.scoreDuration.value);
      scoreLoaded.value = true;
      // Set BPM-aware default zoom
      measuresAhead.value = calculateDefaultMeasures(bpm);
    } else if (scoreId) {
      // Fetch from API
      loadScore(scoreId);
    }
  }

  // Listen for resize
  window.addEventListener('resize', updateLayoutMeasurements);
});

// Cleanup
onUnmounted(() => {
  playback.cleanup();
  scheduler.clearSchedule();
  cleanupHandpanDisplay();
  window.removeEventListener('resize', updateLayoutMeasurements);

  Object.values(audioCache.value).forEach(audio => {
    try {
      audio.pause();
      audio.src = '';
    } catch (e) {
      // Ignore cleanup errors
    }
  });
});
</script>

<style>
@import '@/assets/styles/components/ScoreReader/base.css';
@import '@/assets/styles/components/ScoreReader/falling-notes.css';
@import '@/assets/styles/components/HandpanDisplay/handpan3d.css';

/* Override base layout for overlay approach */
.score-reader {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  background: var(--surface-primary);
  overflow: hidden;
}

/* Stage takes most of the space */
.score-reader__stage {
  flex: 1;
  position: relative;
  display: flex;
  justify-content: center;
  align-items: flex-end; /* Handpan at bottom */
  padding-bottom: 30px; /* Minimal space below handpan */
  overflow: hidden;
  background: linear-gradient(180deg,
    var(--surface-secondary) 0%,
    var(--surface-primary) 60%,
    var(--surface-primary) 100%);
}

/* Handpan at bottom of stage */
.score-reader__stage .handpan-container {
  position: relative;
  z-index: 10;
  margin-top: auto; /* Push to bottom */
}

/* Empty state */
.score-reader--empty {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: var(--surface-primary);
}

.score-reader__message {
  text-align: center;
  padding: var(--spacing-xl);
  color: var(--text-secondary);
}

.score-reader__message h2 {
  margin: var(--spacing-md) 0 var(--spacing-sm);
  color: var(--text-primary);
}

.score-reader__message p {
  margin-bottom: var(--spacing-lg);
}

/* Badges in header */
.scale-badge,
.ding-badge,
.notes-badge {
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-sm);
  background: var(--handpan-steel-mid);
  font-size: var(--font-size-sm);
}

.ding-badge {
  background: var(--handpan-tone-base);
}

/* Back button */
.back-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 50%;
  background: var(--handpan-steel-mid);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  margin-right: var(--spacing-sm);
}

.back-button:hover {
  background: var(--handpan-steel-light);
  transform: scale(1.05);
}

.back-button:active {
  transform: scale(0.95);
}

.score-info {
  display: flex;
  align-items: center;
}
</style>
