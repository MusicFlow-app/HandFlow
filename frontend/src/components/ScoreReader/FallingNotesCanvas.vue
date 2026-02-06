<template>
  <div class="falling-notes-canvas" ref="canvasContainer">
    <!-- Note lanes (one per handpan note) -->
    <div class="note-lanes" :style="{ width: `${laneWidth * noteCount}px` }">
      <div
        v-for="(note, index) in handpanNotes"
        :key="`lane-${index}`"
        class="note-lane"
        :style="{ width: `${laneWidth}px` }"
      >
        <span class="note-lane__label">{{ note.note || note.calculated_note }}</span>
      </div>
    </div>

    <!-- Falling notes -->
    <div
      v-for="event in visibleEvents"
      :key="event.id"
      class="falling-note"
      :class="[
        `falling-note--${event.hand}`,
        `falling-note--${event.noteType}`,
        { 'falling-note--hit': event.isHit }
      ]"
      :style="getNoteStyle(event)"
    >
      <span v-if="showLabels" class="falling-note__label">
        {{ getNoteLabel(event) }}
      </span>
    </div>

    <!-- Beat lines (optional visual guides) -->
    <div
      v-for="beat in visibleBeats"
      :key="`beat-${beat.time}`"
      class="beat-line"
      :class="{ 'beat-line--measure': beat.isMeasure }"
      :style="{ top: `${getBeatPosition(beat.time)}%` }"
    ></div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';

const props = defineProps({
  // Array of timed note events from useNoteScheduler
  events: {
    type: Array,
    default: () => []
  },
  // Current playback time in ms
  currentTime: {
    type: Number,
    default: 0
  },
  // How far ahead to show notes (ms)
  leadTime: {
    type: Number,
    default: 2000
  },
  // How far behind to keep notes visible (ms)
  trailTime: {
    type: Number,
    default: 200
  },
  // Handpan notes for lane mapping
  handpanNotes: {
    type: Array,
    default: () => []
  },
  // Width of each lane in pixels
  laneWidth: {
    type: Number,
    default: 50
  },
  // Whether to show note labels
  showLabels: {
    type: Boolean,
    default: true
  },
  // Tempo for beat line calculation
  tempo: {
    type: Number,
    default: 120
  },
  // Time signature
  timeSignature: {
    type: Object,
    default: () => ({ beats: 4, beatType: 4 })
  }
});

const emit = defineEmits(['note-hit']);

// Refs
const canvasContainer = ref(null);
const hitNotes = ref(new Set());

// Computed
const noteCount = computed(() => props.handpanNotes.length || 8);

// Filter events to only show those in the visible window
const visibleEvents = computed(() => {
  const windowStart = props.currentTime - props.trailTime;
  const windowEnd = props.currentTime + props.leadTime;

  return props.events
    .filter(event => {
      // Include events in the visible window
      return event.absoluteTime >= windowStart && event.absoluteTime <= windowEnd;
    })
    .map(event => ({
      ...event,
      isHit: hitNotes.value.has(event.id)
    }));
});

// Calculate beat lines for visual rhythm guides
const visibleBeats = computed(() => {
  const beats = [];
  const msPerBeat = 60000 / props.tempo;
  const beatsPerMeasure = props.timeSignature.beats;

  const windowStart = props.currentTime - props.trailTime;
  const windowEnd = props.currentTime + props.leadTime;

  // Find the first beat in the window
  const firstBeat = Math.ceil(windowStart / msPerBeat);
  const lastBeat = Math.floor(windowEnd / msPerBeat);

  for (let i = firstBeat; i <= lastBeat; i++) {
    const time = i * msPerBeat;
    beats.push({
      time,
      isMeasure: i % beatsPerMeasure === 0
    });
  }

  return beats;
});

// Methods
const getNoteStyle = (event) => {
  // Calculate Y position based on time
  // Notes at currentTime + leadTime are at top (0%)
  // Notes at currentTime are at bottom (100%)
  const timeOffset = event.absoluteTime - props.currentTime;
  const yPercent = ((props.leadTime - timeOffset) / props.leadTime) * 100;

  // Calculate X position based on handpan note index
  const laneIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const xOffset = (laneIndex - (noteCount.value - 1) / 2) * props.laneWidth;

  return {
    top: `${yPercent}%`,
    left: `calc(50% + ${xOffset}px)`,
    transform: 'translateX(-50%)'
  };
};

const getBeatPosition = (beatTime) => {
  const timeOffset = beatTime - props.currentTime;
  return ((props.leadTime - timeOffset) / props.leadTime) * 100;
};

const getNoteLabel = (event) => {
  // Get note name from handpan notes if available
  if (event.handpanNoteIndex >= 0 && props.handpanNotes[event.handpanNoteIndex]) {
    const note = props.handpanNotes[event.handpanNoteIndex];
    return note.note || note.calculated_note || '';
  }
  return '';
};

// Check for notes that should trigger hits
const checkForHits = () => {
  const hitTolerance = 50; // ms

  props.events.forEach(event => {
    // Check if note is at hit time
    if (Math.abs(event.absoluteTime - props.currentTime) <= hitTolerance) {
      if (!hitNotes.value.has(event.id)) {
        hitNotes.value.add(event.id);
        emit('note-hit', event);

        // Remove hit status after animation
        setTimeout(() => {
          hitNotes.value.delete(event.id);
        }, 300);
      }
    }
  });
};

// Watch for time changes to check hits
watch(() => props.currentTime, () => {
  checkForHits();
});

// Clear hit notes when events change (new score loaded)
watch(() => props.events, () => {
  hitNotes.value.clear();
});
</script>
