<template>
  <div class="falling-notes-overlay" ref="overlayRef">
    <!-- Beat grid lines (measures and quarter beats) -->
    <div
      v-for="beat in visibleBeatMarkers"
      :key="beat.id"
      class="beat-line"
      :class="[
        `beat-line--${beat.type}`,
        { 'beat-line--hit': beat.progress >= 1 }
      ]"
      :style="getBeatLineStyle(beat)"
    >
      <span v-if="beat.type === 'measure'" class="beat-line__label">{{ beat.measureNumber }}</span>
    </div>

    <!-- Synthesia-style falling note bars -->
    <div
      v-for="event in visibleEvents"
      :key="event.id"
      class="note-bar"
      :class="[
        `note-bar--${event.hand}`,
        { 'note-bar--active': event.isActive },
        { 'note-bar--past': event.isPast }
      ]"
      :style="getNoteBarStyle(event)"
    >
      <!-- Hit indicator at the bottom of the bar -->
      <div
        class="note-bar__hit-zone"
        :class="{ 'note-bar__hit-zone--flash': event.isHit }"
      ></div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const props = defineProps({
  events: {
    type: Array,
    default: () => []
  },
  currentTime: {
    type: Number,
    default: 0
  },
  leadTime: {
    type: Number,
    default: 2000
  },
  trailTime: {
    type: Number,
    default: 200
  },
  notePositions: {
    type: Array,
    default: () => []
  },
  handpanNotes: {
    type: Array,
    default: () => []
  },
  handpanCenter: {
    type: Object,
    default: () => ({ x: 0, y: 0 })
  },
  fallHeight: {
    type: Number,
    default: 400
  },
  tempo: {
    type: Number,
    default: 120
  },
  timeSignature: {
    type: Object,
    default: () => ({ beats: 4, beatType: 4 })
  },
  scoreDuration: {
    type: Number,
    default: 0
  }
});

const emit = defineEmits(['note-hit']);

const overlayRef = ref(null);
const hitNotes = ref(new Set());

// Pixels per millisecond (fall speed)
const pixelsPerMs = computed(() => props.fallHeight / props.leadTime);

// Calculate pitch range for width scaling
const pitchRange = computed(() => {
  const pitches = props.events.map(e => e.pitch).filter(p => p > 0);
  if (pitches.length === 0) return { min: 48, max: 72, range: 24 };
  const min = Math.min(...pitches);
  const max = Math.max(...pitches);
  return { min, max, range: max - min || 1 };
});

// Width based on pitch: lower pitch = wider bar
const getPitchWidth = (pitch) => {
  if (!pitch || pitchRange.value.range === 0) return 24;

  const normalized = (pitch - pitchRange.value.min) / pitchRange.value.range;
  // Width range: 36px (lowest) to 16px (highest)
  const maxWidth = 36;
  const minWidth = 16;

  return maxWidth - (normalized * (maxWidth - minWidth));
};

// Convert duration in ms to pixels
const durationToPixels = (durationMs) => {
  return Math.max(12, durationMs * pixelsPerMs.value);
};

// Get position for a handpan note index
const getNotePosition = (noteIndex) => {
  if (noteIndex < 0 || noteIndex >= props.notePositions.length) {
    return { x: 0, y: 0, rotation: 0 };
  }
  return props.notePositions[noteIndex] || { x: 0, y: 0, rotation: 0 };
};

// Filter and enhance visible events
const visibleEvents = computed(() => {
  const windowStart = props.currentTime - props.trailTime;
  const windowEnd = props.currentTime + props.leadTime;

  return props.events
    .filter(event => {
      // Note is visible if any part of it is in the window
      const noteEnd = event.absoluteTime + event.duration;
      return noteEnd >= windowStart && event.absoluteTime <= windowEnd;
    })
    .map(event => {
      const isHit = hitNotes.value.has(event.id);
      // Note is active when current time is within its duration
      const isActive = props.currentTime >= event.absoluteTime &&
                       props.currentTime <= event.absoluteTime + event.duration;
      // Note is past when current time is after its end
      const isPast = props.currentTime > event.absoluteTime + event.duration;

      return { ...event, isHit, isActive, isPast };
    });
});

// Style for Synthesia-style note bars
// The bar's BOTTOM edge represents the hit time
const getNoteBarStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);

  // Bar dimensions
  const barHeight = durationToPixels(event.duration || 500);
  const barWidth = getPitchWidth(event.pitch);

  // Calculate Y position for the BOTTOM of the bar
  // When timeOffset = 0, bottom of bar should be at targetPos.y
  const timeOffset = event.absoluteTime - props.currentTime;
  const bottomY = targetPos.y - (timeOffset * pixelsPerMs.value);

  // The bar extends upward from bottomY
  const topY = bottomY - barHeight;

  // X position (centered on target)
  const currentX = targetPos.x;

  // Opacity: fade in as it appears
  let opacity = 1;
  const distanceFromTop = topY + props.fallHeight; // How far the top is from spawn point
  if (distanceFromTop < 50) {
    opacity = Math.max(0, distanceFromTop / 50);
  }

  return {
    '--bar-width': `${barWidth}px`,
    '--bar-height': `${barHeight}px`,
    '--tx': `${currentX}px`,
    '--ty': `${topY}px`,
    width: `var(--bar-width)`,
    height: `var(--bar-height)`,
    transform: `translate(calc(-50% + var(--tx)), var(--ty))`,
    opacity,
    zIndex: event.isActive ? 100 : 50
  };
};

// Beat grid line style
const getBeatLineStyle = (beat) => {
  const timeOffset = beat.absoluteTime - props.currentTime;
  const progress = 1 - (timeOffset / props.leadTime);
  const clampedProgress = Math.max(0, progress);

  const startY = -props.fallHeight;
  const endY = 0;
  const currentY = startY + (endY - startY) * clampedProgress;

  let opacity = 1;
  if (clampedProgress < 0.1) {
    opacity = clampedProgress / 0.1;
  } else if (progress > 1) {
    opacity = Math.max(0, 1 - (progress - 1) * 3);
  }

  return {
    '--ty': `${currentY}px`,
    transform: `translateY(calc(-50% + var(--ty)))`,
    opacity
  };
};

// Generate beat markers
const allBeatMarkers = computed(() => {
  if (props.scoreDuration <= 0 || props.tempo <= 0) return [];

  const markers = [];
  const msPerBeat = 60000 / props.tempo;
  const beatsPerMeasure = props.timeSignature.beats || 4;
  const msPerMeasure = msPerBeat * beatsPerMeasure;

  let measureNumber = 1;
  let time = 0;

  while (time <= props.scoreDuration + msPerMeasure) {
    markers.push({
      id: `measure-${measureNumber}`,
      type: 'measure',
      absoluteTime: time,
      measureNumber
    });

    for (let beat = 1; beat < beatsPerMeasure; beat++) {
      const beatTime = time + (beat * msPerBeat);
      if (beatTime <= props.scoreDuration + msPerMeasure) {
        markers.push({
          id: `beat-${measureNumber}-${beat}`,
          type: 'beat',
          absoluteTime: beatTime,
          measureNumber,
          beatInMeasure: beat
        });
      }
    }

    measureNumber++;
    time += msPerMeasure;
  }

  return markers;
});

const visibleBeatMarkers = computed(() => {
  const windowStart = props.currentTime - props.trailTime;
  const windowEnd = props.currentTime + props.leadTime;

  return allBeatMarkers.value
    .filter(marker => marker.absoluteTime >= windowStart && marker.absoluteTime <= windowEnd)
    .map(marker => {
      const timeOffset = marker.absoluteTime - props.currentTime;
      const progress = 1 - (timeOffset / props.leadTime);
      return { ...marker, progress };
    });
});

// Check for note hits - triggers when note time is reached
const checkForHits = () => {
  const hitTolerance = 30; // ms - tighter tolerance for accuracy

  props.events.forEach(event => {
    const timeOffset = event.absoluteTime - props.currentTime;

    // Trigger hit exactly when the bottom of the bar reaches the target
    if (timeOffset <= 0 && timeOffset > -hitTolerance) {
      if (!hitNotes.value.has(event.id)) {
        hitNotes.value.add(event.id);
        emit('note-hit', event);

        // Keep hit visual for the duration of the note
        setTimeout(() => {
          hitNotes.value.delete(event.id);
        }, Math.min(event.duration || 300, 500));
      }
    }
  });
};

watch(() => props.currentTime, () => {
  checkForHits();
});

watch(() => props.events, () => {
  hitNotes.value.clear();
});
</script>

<style scoped>
.falling-notes-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  overflow: hidden;
  z-index: 15;
}

/* Synthesia-style note bar */
.note-bar {
  position: absolute;
  top: 50%;
  left: 50%;
  border-radius: 4px;
  pointer-events: none;
  will-change: transform, opacity;

  /* Default ghost appearance */
  background: linear-gradient(180deg,
    rgba(255, 255, 255, 0.05) 0%,
    rgba(255, 255, 255, 0.15) 90%,
    rgba(255, 255, 255, 0.3) 100%);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

/* Left hand - blue */
.note-bar--left {
  background: linear-gradient(180deg,
    rgba(90, 138, 176, 0.1) 0%,
    rgba(90, 138, 176, 0.25) 85%,
    rgba(90, 138, 176, 0.6) 100%);
  border-color: rgba(90, 138, 176, 0.4);
  box-shadow: 0 0 8px rgba(90, 138, 176, 0.2);
}

/* Right hand - red/warm */
.note-bar--right {
  background: linear-gradient(180deg,
    rgba(176, 106, 90, 0.1) 0%,
    rgba(176, 106, 90, 0.25) 85%,
    rgba(176, 106, 90, 0.6) 100%);
  border-color: rgba(176, 106, 90, 0.4);
  box-shadow: 0 0 8px rgba(176, 106, 90, 0.2);
}

/* Active state - currently playing */
.note-bar--active {
  border-width: 2px;
}

.note-bar--active.note-bar--left {
  background: linear-gradient(180deg,
    rgba(90, 138, 176, 0.3) 0%,
    rgba(90, 138, 176, 0.5) 85%,
    rgba(90, 138, 176, 0.9) 100%);
  border-color: rgba(90, 138, 176, 0.8);
  box-shadow: 0 0 20px rgba(90, 138, 176, 0.5);
}

.note-bar--active.note-bar--right {
  background: linear-gradient(180deg,
    rgba(176, 106, 90, 0.3) 0%,
    rgba(176, 106, 90, 0.5) 85%,
    rgba(176, 106, 90, 0.9) 100%);
  border-color: rgba(176, 106, 90, 0.8);
  box-shadow: 0 0 20px rgba(176, 106, 90, 0.5);
}

/* Past state - already played */
.note-bar--past {
  opacity: 0.3 !important;
}

/* Hit zone at bottom of bar */
.note-bar__hit-zone {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 8px;
  border-radius: 0 0 3px 3px;
  background: rgba(255, 255, 255, 0.3);
  transition: all 0.1s ease;
}

.note-bar--left .note-bar__hit-zone {
  background: rgba(90, 138, 176, 0.5);
}

.note-bar--right .note-bar__hit-zone {
  background: rgba(176, 106, 90, 0.5);
}

/* Flash effect when hit */
.note-bar__hit-zone--flash {
  height: 12px;
  background: rgba(255, 255, 255, 0.9) !important;
  box-shadow: 0 0 20px rgba(255, 255, 255, 0.8);
  animation: hit-flash 0.15s ease-out;
}

@keyframes hit-flash {
  0% {
    transform: scaleX(1.5);
    box-shadow: 0 0 30px rgba(255, 255, 255, 1);
  }
  100% {
    transform: scaleX(1);
    box-shadow: 0 0 20px rgba(255, 255, 255, 0.8);
  }
}

/* Beat grid lines */
.beat-line {
  position: absolute;
  top: 50%;
  left: 5%;
  right: 5%;
  height: 1px;
  pointer-events: none;
  will-change: transform, opacity;
  z-index: 5;
}

.beat-line--measure {
  height: 2px;
  background: linear-gradient(90deg,
    transparent 0%,
    rgba(255, 255, 255, 0.4) 15%,
    rgba(255, 255, 255, 0.5) 50%,
    rgba(255, 255, 255, 0.4) 85%,
    transparent 100%);
  box-shadow: 0 0 8px rgba(255, 255, 255, 0.2);
}

.beat-line--beat {
  height: 1px;
  background: linear-gradient(90deg,
    transparent 0%,
    rgba(255, 255, 255, 0.15) 20%,
    rgba(255, 255, 255, 0.2) 50%,
    rgba(255, 255, 255, 0.15) 80%,
    transparent 100%);
}

.beat-line--hit {
  opacity: 0.3;
}

.beat-line__label {
  position: absolute;
  left: -30px;
  top: 50%;
  transform: translateY(-50%);
  font-size: 11px;
  font-weight: 600;
  color: rgba(255, 255, 255, 0.5);
  font-family: var(--font-mono, monospace);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
}

/* Light theme */
:root[data-theme="light"] .note-bar {
  background: linear-gradient(180deg,
    rgba(0, 0, 0, 0.03) 0%,
    rgba(0, 0, 0, 0.1) 90%,
    rgba(0, 0, 0, 0.2) 100%);
  border-color: rgba(0, 0, 0, 0.15);
}

:root[data-theme="light"] .note-bar--left {
  background: linear-gradient(180deg,
    rgba(70, 118, 156, 0.1) 0%,
    rgba(70, 118, 156, 0.25) 85%,
    rgba(70, 118, 156, 0.6) 100%);
  border-color: rgba(70, 118, 156, 0.4);
  box-shadow: 0 0 8px rgba(70, 118, 156, 0.2);
}

:root[data-theme="light"] .note-bar--right {
  background: linear-gradient(180deg,
    rgba(156, 86, 70, 0.1) 0%,
    rgba(156, 86, 70, 0.25) 85%,
    rgba(156, 86, 70, 0.6) 100%);
  border-color: rgba(156, 86, 70, 0.4);
  box-shadow: 0 0 8px rgba(156, 86, 70, 0.2);
}

:root[data-theme="light"] .beat-line--measure {
  background: linear-gradient(90deg,
    transparent 0%,
    rgba(0, 0, 0, 0.2) 15%,
    rgba(0, 0, 0, 0.3) 50%,
    rgba(0, 0, 0, 0.2) 85%,
    transparent 100%);
  box-shadow: 0 0 8px rgba(0, 0, 0, 0.1);
}

:root[data-theme="light"] .beat-line--beat {
  background: linear-gradient(90deg,
    transparent 0%,
    rgba(0, 0, 0, 0.1) 20%,
    rgba(0, 0, 0, 0.15) 50%,
    rgba(0, 0, 0, 0.1) 80%,
    transparent 100%);
}

:root[data-theme="light"] .beat-line__label {
  color: rgba(0, 0, 0, 0.4);
  text-shadow: none;
}

/* Responsive */
@media (max-width: 768px) {
  .beat-line {
    left: 2%;
    right: 2%;
  }

  .beat-line__label {
    left: -20px;
    font-size: 10px;
  }
}

@media (max-width: 480px) {
  .beat-line__label {
    display: none;
  }
}
</style>
