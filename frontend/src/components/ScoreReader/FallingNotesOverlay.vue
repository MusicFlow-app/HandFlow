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

    <!-- Falling notes with duration boxes -->
    <div
      v-for="event in visibleEvents"
      :key="event.id"
      class="falling-note-container"
      :class="[
        `falling-note-container--${event.hand}`,
        { 'falling-note-container--hit': event.isHit }
      ]"
      :style="getContainerStyle(event)"
    >
      <!-- Duration box (transparent background showing timing) -->
      <div class="falling-note-duration-box" :style="getDurationBoxStyle(event)"></div>

      <!-- The actual tone field note -->
      <div
        class="falling-tone-field"
        :class="[
          `falling-tone-field--${event.hand}`,
          `falling-tone-field--${event.noteType}`,
          { 'falling-tone-field--hit': event.isHit }
        ]"
        :style="getToneFieldStyle(event)"
      >
        <div class="falling-tone-field__inner" :style="getNoteInnerStyle(event)"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

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
  // How far ahead to show notes (ms) - determines fall distance
  leadTime: {
    type: Number,
    default: 2000
  },
  // How far behind to keep notes visible (ms)
  trailTime: {
    type: Number,
    default: 200
  },
  // Note positions from the handpan - array of { x, y, rotation, noteIndex }
  notePositions: {
    type: Array,
    default: () => []
  },
  // Handpan notes data (for pitch-based scaling)
  handpanNotes: {
    type: Array,
    default: () => []
  },
  // Handpan center position relative to overlay
  handpanCenter: {
    type: Object,
    default: () => ({ x: 0, y: 0 })
  },
  // Height from which notes start falling (pixels above handpan center)
  fallHeight: {
    type: Number,
    default: 400
  },
  // Tempo in BPM for beat grid
  tempo: {
    type: Number,
    default: 120
  },
  // Time signature (beats per measure)
  timeSignature: {
    type: Object,
    default: () => ({ beats: 4, beatType: 4 })
  },
  // Total score duration in ms
  scoreDuration: {
    type: Number,
    default: 0
  }
});

const emit = defineEmits(['note-hit']);

// Refs
const overlayRef = ref(null);
const hitNotes = ref(new Set());

// Constants for duration box sizing
const MIN_BOX_HEIGHT = 8; // Minimum height in pixels (64th note)
const PIXELS_PER_BEAT = 80; // How many pixels per beat at 1x speed

// Calculate pitch range for scaling
const pitchRange = computed(() => {
  const pitches = props.events.map(e => e.pitch).filter(p => p > 0);
  if (pitches.length === 0) return { min: 48, max: 72, range: 24 };

  const min = Math.min(...pitches);
  const max = Math.max(...pitches);
  return { min, max, range: max - min || 1 };
});

// Calculate scale factor based on pitch (lower = larger, higher = smaller)
const getPitchScale = (pitch) => {
  if (!pitch || pitchRange.value.range === 0) return 1;

  // Normalize pitch to 0-1 range (0 = lowest, 1 = highest)
  const normalized = (pitch - pitchRange.value.min) / pitchRange.value.range;

  // Scale range: 1.1 (lowest pitch) to 0.8 (highest pitch)
  const maxScale = 1.1;
  const minScale = 0.8;

  return maxScale - (normalized * (maxScale - minScale));
};

// Convert duration in ms to pixels based on fall speed
const durationToPixels = (durationMs) => {
  // Calculate pixels per ms based on fall height and lead time
  const pixelsPerMs = props.fallHeight / props.leadTime;
  const pixels = durationMs * pixelsPerMs;

  // Ensure minimum height
  return Math.max(MIN_BOX_HEIGHT, pixels);
};

// Filter events to only show those in the visible window
const visibleEvents = computed(() => {
  const windowStart = props.currentTime - props.trailTime;
  const windowEnd = props.currentTime + props.leadTime;

  return props.events
    .filter(event => {
      return event.absoluteTime >= windowStart && event.absoluteTime <= windowEnd;
    })
    .map(event => ({
      ...event,
      isHit: hitNotes.value.has(event.id)
    }));
});

// Generate all beat markers for the score
const allBeatMarkers = computed(() => {
  if (props.scoreDuration <= 0 || props.tempo <= 0) return [];

  const markers = [];
  const msPerBeat = 60000 / props.tempo;
  const beatsPerMeasure = props.timeSignature.beats || 4;
  const msPerMeasure = msPerBeat * beatsPerMeasure;

  // Generate measure and beat markers
  let measureNumber = 1;
  let time = 0;

  while (time <= props.scoreDuration + msPerMeasure) {
    // Add measure line
    markers.push({
      id: `measure-${measureNumber}`,
      type: 'measure',
      absoluteTime: time,
      measureNumber
    });

    // Add quarter beat lines within the measure
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

// Filter beat markers to visible window
const visibleBeatMarkers = computed(() => {
  const windowStart = props.currentTime - props.trailTime;
  const windowEnd = props.currentTime + props.leadTime;

  return allBeatMarkers.value
    .filter(marker => {
      return marker.absoluteTime >= windowStart && marker.absoluteTime <= windowEnd;
    })
    .map(marker => {
      const timeOffset = marker.absoluteTime - props.currentTime;
      const progress = 1 - (timeOffset / props.leadTime);
      return { ...marker, progress };
    });
});

// Style for beat grid lines
const getBeatLineStyle = (beat) => {
  const timeOffset = beat.absoluteTime - props.currentTime;
  const progress = 1 - (timeOffset / props.leadTime);
  const clampedProgress = Math.max(0, progress);

  // Calculate Y position (same as notes)
  const startY = -props.fallHeight;
  const endY = 0; // Center
  const currentY = startY + (endY - startY) * clampedProgress;

  // Opacity: fade in as it appears, fade out after passing
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

// Get the position data for a specific handpan note index
const getNotePosition = (noteIndex) => {
  if (noteIndex < 0 || noteIndex >= props.notePositions.length) {
    return { x: 0, y: 0, rotation: 0 };
  }
  return props.notePositions[noteIndex] || { x: 0, y: 0, rotation: 0 };
};

// Container style (position and opacity)
const getContainerStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);

  // Calculate progress: 0 = just appeared (top), 1 = at target (hit)
  const timeOffset = event.absoluteTime - props.currentTime;
  const progress = 1 - (timeOffset / props.leadTime);
  const clampedProgress = Math.max(0, progress);

  // Calculate Y position
  const startY = -props.fallHeight;
  const endY = targetPos.y;
  const currentY = startY + (endY - startY) * clampedProgress;

  // X position stays constant (directly above target)
  const currentX = targetPos.x;

  // Opacity: fade in as it appears, fade out after hit
  let opacity = 1;
  if (clampedProgress < 0.1) {
    opacity = clampedProgress / 0.1;
  } else if (progress > 1) {
    opacity = Math.max(0, 1 - (progress - 1) * 5);
  }

  return {
    '--tx': `${currentX}px`,
    '--ty': `${currentY}px`,
    '--opacity': opacity,
    transform: `translate(calc(-50% + var(--tx)), calc(-50% + var(--ty)))`,
    opacity: `var(--opacity)`,
    zIndex: Math.floor(progress * 100)
  };
};

// Duration box style (height based on note duration)
const getDurationBoxStyle = (event) => {
  const boxHeight = durationToPixels(event.duration || 500);

  return {
    height: `${boxHeight}px`
  };
};

// Tone field style (size based on pitch)
const getToneFieldStyle = (event) => {
  const pitchScale = getPitchScale(event.pitch);

  // Approach scale: smaller when far, full size when near
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);
  const timeOffset = event.absoluteTime - props.currentTime;
  const progress = 1 - (timeOffset / props.leadTime);
  const clampedProgress = Math.max(0, Math.min(1, progress));
  const approachScale = 0.6 + (clampedProgress * 0.4);

  // Combined scale
  const totalScale = pitchScale * approachScale;

  return {
    '--pitch-scale': pitchScale,
    '--approach-scale': approachScale,
    '--total-scale': totalScale,
    transform: `scale(var(--total-scale))`
  };
};

// Get the style for the inner note element (rotation)
const getNoteInnerStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);

  return {
    '--rotation': `${targetPos.rotation || 0}deg`,
    transform: `translate(-50%, -50%) rotate(var(--rotation))`
  };
};

// Check for notes that should trigger hits
const checkForHits = () => {
  const hitTolerance = 50; // ms

  props.events.forEach(event => {
    const timeOffset = event.absoluteTime - props.currentTime;

    if (Math.abs(timeOffset) <= hitTolerance) {
      if (!hitNotes.value.has(event.id)) {
        hitNotes.value.add(event.id);
        emit('note-hit', event);

        setTimeout(() => {
          hitNotes.value.delete(event.id);
        }, 400);
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

/* Container for note + duration box */
.falling-note-container {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 80px;
  display: flex;
  flex-direction: column;
  align-items: center;
  pointer-events: none;
  will-change: transform, opacity;
}

/* Duration box - transparent timing indicator */
.falling-note-duration-box {
  width: 4px;
  min-height: 8px;
  background: rgba(255, 255, 255, 0.15);
  border-radius: 2px;
  margin-bottom: -4px;
  position: relative;
  z-index: 1;
}

/* Left hand duration box */
.falling-note-container--left .falling-note-duration-box {
  background: rgba(90, 138, 176, 0.3);
  box-shadow: 0 0 6px rgba(90, 138, 176, 0.2);
}

/* Right hand duration box */
.falling-note-container--right .falling-note-duration-box {
  background: rgba(176, 106, 90, 0.3);
  box-shadow: 0 0 6px rgba(176, 106, 90, 0.2);
}

/* Falling tone field note */
.falling-tone-field {
  width: 80px;
  height: 80px;
  position: relative;
  flex-shrink: 0;
}

.falling-tone-field__inner {
  width: 65px;
  height: 52px;
  position: absolute;
  top: 50%;
  left: 50%;
  border-radius: 50%;

  background: radial-gradient(ellipse 70% 60% at 40% 35%,
    var(--handpan-highlight, #a8a8a8) 0%,
    var(--handpan-steel-light, #888) 30%,
    var(--handpan-steel-mid, #666) 70%,
    var(--handpan-steel-dark, #444) 100%);

  box-shadow:
    inset 2px 2px 6px rgba(255, 255, 255, 0.3),
    inset -2px -2px 4px rgba(0, 0, 0, 0.3),
    0 4px 12px rgba(0, 0, 0, 0.4);
}

/* Left hand - blue tint */
.falling-tone-field--left .falling-tone-field__inner {
  background: radial-gradient(ellipse 70% 60% at 40% 35%,
    #9ec5e8 0%,
    #6a9fc4 30%,
    #4a7fa0 70%,
    #3a5f80 100%);
  box-shadow:
    inset 2px 2px 6px rgba(180, 220, 255, 0.4),
    inset -2px -2px 4px rgba(0, 0, 0, 0.3),
    0 4px 12px rgba(58, 95, 128, 0.5),
    0 0 20px rgba(90, 138, 176, 0.3);
}

/* Right hand - red/warm tint */
.falling-tone-field--right .falling-tone-field__inner {
  background: radial-gradient(ellipse 70% 60% at 40% 35%,
    #e8b09e 0%,
    #c47a6a 30%,
    #a05a4a 70%,
    #803a2a 100%);
  box-shadow:
    inset 2px 2px 6px rgba(255, 200, 180, 0.4),
    inset -2px -2px 4px rgba(0, 0, 0, 0.3),
    0 4px 12px rgba(128, 58, 42, 0.5),
    0 0 20px rgba(176, 106, 90, 0.3);
}

/* Ghost note - semi-transparent */
.falling-tone-field--ghost .falling-tone-field__inner {
  opacity: 0.5;
  border: 2px dashed var(--handpan-steel-light, #888);
}

/* Grace note - smaller */
.falling-tone-field--grace {
  width: 60px;
  height: 60px;
}

.falling-tone-field--grace .falling-tone-field__inner {
  width: 48px;
  height: 38px;
}

/* Hit animation - preserves rotation */
.falling-tone-field--hit .falling-tone-field__inner {
  animation: falling-note-hit 0.3s ease-out forwards;
}

@keyframes falling-note-hit {
  0% {
    opacity: 1;
    transform: translate(-50%, -50%) rotate(var(--rotation, 0deg)) scale(1);
  }
  50% {
    opacity: 0.8;
    transform: translate(-50%, -50%) rotate(var(--rotation, 0deg)) scale(1.4);
    box-shadow:
      inset 2px 2px 6px rgba(255, 255, 255, 0.5),
      inset -2px -2px 4px rgba(0, 0, 0, 0.3),
      0 0 30px rgba(255, 255, 255, 0.6);
  }
  100% {
    opacity: 0;
    transform: translate(-50%, -50%) rotate(var(--rotation, 0deg)) scale(0.8);
  }
}

/* Hit animation for container (fade the box too) */
.falling-note-container--hit .falling-note-duration-box {
  animation: duration-box-hit 0.3s ease-out forwards;
}

@keyframes duration-box-hit {
  0% { opacity: 1; }
  100% { opacity: 0; }
}

/* Responsive */
@media (max-width: 768px) {
  .falling-note-container {
    width: 60px;
  }

  .falling-tone-field {
    width: 60px;
    height: 60px;
  }

  .falling-tone-field__inner {
    width: 50px;
    height: 40px;
  }

  .falling-note-duration-box {
    width: 3px;
  }
}

@media (max-width: 480px) {
  .falling-note-container {
    width: 50px;
  }

  .falling-tone-field {
    width: 50px;
    height: 50px;
  }

  .falling-tone-field__inner {
    width: 42px;
    height: 34px;
  }

  .falling-note-duration-box {
    width: 2px;
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

/* Measure lines - more prominent */
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

/* Quarter beat lines - subtle */
.beat-line--beat {
  height: 1px;
  background: linear-gradient(90deg,
    transparent 0%,
    rgba(255, 255, 255, 0.15) 20%,
    rgba(255, 255, 255, 0.2) 50%,
    rgba(255, 255, 255, 0.15) 80%,
    transparent 100%);
}

/* Fade out after passing */
.beat-line--hit {
  opacity: 0.3;
}

/* Measure number label */
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

/* Dark theme adjustments */
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
