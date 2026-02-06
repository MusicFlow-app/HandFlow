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
        @toggle-play="playback.togglePlay"
        @stop="playback.stop"
        @seek="playback.seek"
        @set-speed="playback.setSpeed"
        @toggle-loop="playback.toggleLoop"
      />
    </div>

    <!-- Falling notes canvas -->
    <div class="score-reader__canvas">
      <FallingNotesCanvas
        :events="scheduler.scheduledEvents.value"
        :current-time="playback.currentTime.value"
        :lead-time="LEAD_TIME"
        :trail-time="TRAIL_TIME"
        :handpan-notes="allHandpanNotes"
        :lane-width="laneWidth"
        :show-labels="showNoteLabels"
        :tempo="scheduler.tempo.value"
        :time-signature="scheduler.timeSignature.value"
        @note-hit="handleNoteHit"
      />
    </div>

    <!-- Handpan display -->
    <div class="score-reader__handpan">
      <div class="score-reader__hit-line"></div>

      <!-- Embedded handpan visualization -->
      <div class="handpan-container top-view">
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
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { PhMetronome, PhMusicNotes, PhArrowLeft } from '@phosphor-icons/vue';
import { apiUrl } from '@/services/api';

import PlaybackControls from './PlaybackControls.vue';
import FallingNotesCanvas from './FallingNotesCanvas.vue';
import useNoteScheduler from '@/composables/useNoteScheduler';
import useScorePlayback from '@/composables/useScorePlayback';
import useHandpanSelection from '@/composables/useHandpanSelection';

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
const LEAD_TIME = 2000; // 2 seconds ahead
const TRAIL_TIME = 200; // 0.2 seconds behind

// Router
const route = useRoute();
const router = useRouter();

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
const showNoteLabels = ref(true);
const audioCache = ref({});

// Composables
const scheduler = useNoteScheduler();
const playback = useScorePlayback();

// Check if handpan is configured
const isHandpanReady = computed(() => {
  return selectedScale.value && selectedDing.value && notes.value && notes.value.length > 0;
});

// Get the ding note data
const dingData = computed(() => {
  if (!notes.value || !Array.isArray(notes.value)) return null;
  return notes.value.find(note => note.id === 0 || note.note_index === 0);
});

// Display value for ding
const dingNoteDisplay = computed(() => {
  if (dingData.value) {
    return dingData.value.note || dingData.value.calculated_note || selectedDing.value;
  }
  return selectedDing.value || 'D';
});

// Filter notes by position (excluding ding)
const displayedNotes = computed(() => {
  if (!notes.value || !Array.isArray(notes.value)) return [];

  // Filter out ding and limit by selectedNoteCount
  let filtered = notes.value.filter(note =>
    note.id !== 0 && note.note_index !== 0
  );

  // Limit by note count if specified
  if (selectedNoteCount.value && selectedNoteCount.value > 0) {
    filtered = filtered.slice(0, selectedNoteCount.value);
  }

  return filtered;
});

// Top notes (position === 'Top' or undefined)
const topNotes = computed(() => {
  return displayedNotes.value.filter(note =>
    note.position === 'Top' || !note.position
  );
});

// Inner notes
const innerNotes = computed(() => {
  return displayedNotes.value.filter(note => note.position === 'Inner');
});

// Bottom notes
const bottomNotes = computed(() => {
  return displayedNotes.value.filter(note => note.position === 'Bottom');
});

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

// Responsive lane width
const laneWidth = computed(() => {
  if (typeof window !== 'undefined') {
    if (window.innerWidth < 480) return 32;
    if (window.innerWidth < 768) return 40;
  }
  return 50;
});

// Note positioning (adapted from useHandpanDisplay)
const getNoteWrapperStyle = (index, total, position, note) => {
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

  return {
    '--tx': `${x}px`,
    '--ty': `${y}px`,
    transform: `translate(var(--tx), var(--ty))`,
    position: 'absolute',
    zIndex: '5'
  };
};

const getNoteInnerStyle = (index, total, position, note) => {
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
  const rotationDegrees = angleInDegrees > 180 ? angleInDegrees + 90 : angleInDegrees - 90;

  return {
    transform: `rotate(${rotationDegrees}deg)`,
    transformOrigin: 'center center'
  };
};

// Inner note positioning
const getInnerNoteWrapperStyle = (index, total, note) => {
  const baseAngle = 360 / total;
  const angleInDegrees = (index - 1) * baseAngle;
  const angleInRadians = angleInDegrees * (Math.PI / 180);
  const radius = 70;

  const x = Math.sin(angleInRadians) * radius;
  const y = -Math.cos(angleInRadians) * radius;

  return {
    '--tx': `${x}px`,
    '--ty': `${y}px`,
    transform: `translate(var(--tx), var(--ty))`,
    position: 'absolute',
    zIndex: '5'
  };
};

const getInnerNoteInnerStyle = (index, total, note) => {
  const baseAngle = 360 / total;
  const angleInDegrees = (index - 1) * baseAngle;
  const rotationDegrees = angleInDegrees > 180 ? angleInDegrees + 90 : angleInDegrees - 90;

  return {
    transform: `rotate(${rotationDegrees}deg)`,
    transformOrigin: 'center center'
  };
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

    console.log(`Loaded score: ${tabData.metadata?.work_title}, ${measures.length} measures, ${scheduler.eventCount.value} events`);
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

// Watch for handpan changes to reload audio
watch(allHandpanNotes, () => {
  preloadAudio();
}, { deep: true });

// Initialize
onMounted(() => {
  const scoreId = props.scoreId || route.params.scoreId;

  if (isHandpanReady.value) {
    preloadAudio();

    if (props.scoreData) {
      // Use provided score data
      scoreMetadata.value = props.scoreData.metadata;
      scheduler.scheduleScore(props.scoreData.score_data, allHandpanNotes.value, {
        bpm: props.scoreData.metadata?.tempo || 120
      });
      playback.setDuration(scheduler.scoreDuration.value);
      scoreLoaded.value = true;
    } else if (scoreId) {
      // Fetch from API
      loadScore(scoreId);
    }
  }
});

// Cleanup
onUnmounted(() => {
  playback.cleanup();
  scheduler.clearSchedule();

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
