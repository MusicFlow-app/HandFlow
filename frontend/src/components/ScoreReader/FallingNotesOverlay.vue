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

    <!-- Connecting lines from notes to targets -->
    <svg class="connection-lines" v-if="approachingEvents.length > 0">
      <line
        v-for="event in approachingEvents"
        :key="`line-${event.id}`"
        class="connection-line"
        :class="`connection-line--${event.hand}`"
        :x1="getLineCoords(event).x1"
        :y1="getLineCoords(event).y1"
        :x2="getLineCoords(event).x2"
        :y2="getLineCoords(event).y2"
        :style="{ opacity: getLineOpacity(event) }"
      />
    </svg>

    <!-- Target glow indicators on handpan positions -->
    <div
      v-for="event in approachingEvents"
      :key="`glow-${event.id}`"
      class="target-glow"
      :class="`target-glow--${event.hand}`"
      :style="getTargetGlowStyle(event)"
    ></div>

    <!-- Synthesia-style falling note bars with tone field bottom -->
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
      <!-- Realistic tone field shape at the bottom (hit zone) -->
      <div
        class="note-bar__tone-field"
        :class="[
          `note-bar__tone-field--${event.hand}`,
          { 'note-bar__tone-field--flash': event.isHit }
        ]"
        :style="getToneFieldStyle(event)"
      >
        <!-- Nipple in center like real handpan -->
        <div class="note-bar__nipple"></div>
      </div>
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

const emit = defineEmits(['note-hit', 'note-approaching']);

const overlayRef = ref(null);
const hitNotes = ref(new Set());

// Pixels per millisecond (fall speed)
const pixelsPerMs = computed(() => props.fallHeight / props.leadTime);

// Sort handpan notes by pitch for rank-based scaling (same as useHandpanDisplay.js)
const sortedHandpanNotes = computed(() => {
  if (!props.handpanNotes || props.handpanNotes.length === 0) return [];

  // Map notes with their pitches and original indices
  const notesWithPitches = props.handpanNotes.map((note, index) => {
    let pitch = note.calculated_pitch;
    if (!pitch && note.note) {
      // Fallback: calculate from note name
      pitch = noteToPitchValue(note.note || note.calculated_note || '');
    }
    return {
      originalIndex: index,
      pitch: pitch || 0,
      note: note
    };
  });

  // Sort by pitch (lowest to highest)
  notesWithPitches.sort((a, b) => a.pitch - b.pitch);
  return notesWithPitches;
});

// Convert note name to pitch value (same logic as useHandpanDisplay.js)
const noteToPitchValue = (noteStr) => {
  if (!noteStr) return 0;
  const match = noteStr.match(/([A-G][#b]?)([0-9])/);
  if (!match) return 0;

  const [, note, octave] = match;
  const noteValues = {
    'C': 0, 'C#': 1, 'Db': 1,
    'D': 2, 'D#': 3, 'Eb': 3,
    'E': 4, 'F': 5, 'F#': 6, 'Gb': 6,
    'G': 7, 'G#': 8, 'Ab': 8,
    'A': 9, 'A#': 10, 'Bb': 10,
    'B': 11
  };

  return parseInt(octave) * 12 + noteValues[note];
};

// Get rank-based scale factor for a handpan note index (same logic as useHandpanDisplay.js)
const getScaleFactorForNoteIndex = (handpanNoteIndex) => {
  // Ding (index 0) gets a special larger size to match the handpan display
  // On the handpan, ding is 70x70px while tone fields are 65x52px
  // So ding should be ~1.15x the scale of a regular note
  if (handpanNoteIndex === 0) {
    return 1.15;
  }

  // For other notes, exclude ding from rank calculation (same as useHandpanDisplay.js)
  const sorted = sortedHandpanNotes.value.filter(n => n.originalIndex !== 0);
  if (sorted.length <= 1) return 1.0;

  // Find the rank of this note in the sorted list (excluding ding)
  const rank = sorted.findIndex(n => n.originalIndex === handpanNoteIndex);
  if (rank === -1) return 1.0;

  // Scale range: 1.1 (lowest pitch = rank 0) to 0.8 (highest pitch = rank N-1)
  const maxScale = 1.1;
  const minScale = 0.8;
  const noteCount = sorted.length;
  const step = (maxScale - minScale) / (noteCount - 1);

  const scale = maxScale - (rank * step);
  return scale;
};

// Bar width based on rank scale
const getBarWidth = (handpanNoteIndex) => {
  const scale = getScaleFactorForNoteIndex(handpanNoteIndex);
  const baseWidth = 20;
  return baseWidth * scale;
};

// Tone field dimensions based on rank scale
const getToneFieldSize = (handpanNoteIndex) => {
  const scale = getScaleFactorForNoteIndex(handpanNoteIndex);
  // Base size matches handpan tone fields (65x52px)
  return {
    width: 65 * scale,
    height: 52 * scale
  };
};

// Convert duration in ms to pixels
const durationToPixels = (durationMs) => {
  return Math.max(20, durationMs * pixelsPerMs.value);
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
      const noteEnd = event.absoluteTime + event.duration;
      return noteEnd >= windowStart && event.absoluteTime <= windowEnd;
    })
    .map(event => {
      const isHit = hitNotes.value.has(event.id);
      const isActive = props.currentTime >= event.absoluteTime &&
                       props.currentTime <= event.absoluteTime + event.duration;
      const isPast = props.currentTime > event.absoluteTime + event.duration;
      return { ...event, isHit, isActive, isPast };
    });
});

// Events that are approaching (within 800ms of hitting)
const approachingEvents = computed(() => {
  return props.events
    .filter(event => {
      const timeOffset = event.absoluteTime - props.currentTime;
      return timeOffset > 0 && timeOffset < 800;
    })
    .map(event => {
      const timeOffset = event.absoluteTime - props.currentTime;
      const proximity = 1 - (timeOffset / 800); // 0 = far, 1 = very close
      return { ...event, proximity };
    });
});

// Style for note bars
const getNoteBarStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);

  // Use handpanNoteIndex for rank-based sizing (same as handpan display)
  const toneFieldSize = getToneFieldSize(noteIndex);
  const barWidth = getBarWidth(noteIndex);
  const barHeight = durationToPixels(event.duration || 500);

  // Y position: bottom of bar = hit time
  const timeOffset = event.absoluteTime - props.currentTime;
  const bottomY = targetPos.y - (timeOffset * pixelsPerMs.value);
  const topY = bottomY - barHeight;

  const currentX = targetPos.x;

  let opacity = 1;
  const distanceFromTop = topY + props.fallHeight;
  if (distanceFromTop < 50) {
    opacity = Math.max(0, distanceFromTop / 50);
  }

  return {
    '--bar-width': `${barWidth}px`,
    '--bar-height': `${barHeight}px`,
    '--tone-field-width': `${toneFieldSize.width}px`,
    '--tone-field-height': `${toneFieldSize.height}px`,
    '--tx': `${currentX}px`,
    '--ty': `${topY}px`,
    width: `var(--bar-width)`,
    height: `var(--bar-height)`,
    transform: `translate(calc(-50% + var(--tx)), var(--ty))`,
    opacity,
    zIndex: event.isActive ? 100 : 50
  };
};

// Style for the tone field at bottom of bar (rank-based size like handpan display)
const getToneFieldStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);
  // Use handpanNoteIndex for rank-based sizing (same as handpan display)
  const size = getToneFieldSize(noteIndex);

  return {
    '--rotation': `${targetPos.rotation || 0}deg`,
    width: `${size.width}px`,
    height: `${size.height}px`,
    transform: `translateX(-50%) rotate(var(--rotation))`
  };
};

// Get connecting line coordinates
const getLineCoords = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);

  const timeOffset = event.absoluteTime - props.currentTime;
  const bottomY = targetPos.y - (timeOffset * pixelsPerMs.value);

  // SVG is centered at 50%, 50% of overlay
  // Convert to SVG coordinates (center = 50%, 50%)
  return {
    x1: `calc(50% + ${targetPos.x}px)`,
    y1: `calc(50% + ${bottomY}px)`,
    x2: `calc(50% + ${targetPos.x}px)`,
    y2: `calc(50% + ${targetPos.y}px)`
  };
};

// Line opacity based on proximity
const getLineOpacity = (event) => {
  return 0.15 + (event.proximity * 0.35);
};

// Target glow style
const getTargetGlowStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);
  // Use handpanNoteIndex for rank-based sizing (same as handpan display)
  const size = getToneFieldSize(noteIndex);

  return {
    '--tx': `${targetPos.x}px`,
    '--ty': `${targetPos.y}px`,
    '--glow-size': `${Math.max(size.width, size.height) * 1.5}px`,
    '--glow-opacity': event.proximity * 0.6,
    transform: `translate(calc(-50% + var(--tx)), calc(-50% + var(--ty)))`,
    width: `var(--glow-size)`,
    height: `var(--glow-size)`,
    opacity: `var(--glow-opacity)`
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

// Check for note hits
const checkForHits = () => {
  const hitTolerance = 30;

  props.events.forEach(event => {
    const timeOffset = event.absoluteTime - props.currentTime;

    if (timeOffset <= 0 && timeOffset > -hitTolerance) {
      if (!hitNotes.value.has(event.id)) {
        hitNotes.value.add(event.id);
        emit('note-hit', event);

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
}, { immediate: true });
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

/* SVG for connection lines */
.connection-lines {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 10;
}

.connection-line {
  stroke-width: 2;
  stroke-dasharray: 4 4;
  fill: none;
}

.connection-line--left {
  stroke: rgba(90, 138, 176, 0.5);
}

.connection-line--right {
  stroke: rgba(176, 106, 90, 0.5);
}

/* Target glow on handpan */
.target-glow {
  position: absolute;
  top: 50%;
  left: 50%;
  border-radius: 50%;
  pointer-events: none;
  z-index: 8;
}

.target-glow--left {
  background: radial-gradient(circle, rgba(90, 138, 176, 0.4) 0%, transparent 70%);
  box-shadow: 0 0 30px rgba(90, 138, 176, 0.5);
}

.target-glow--right {
  background: radial-gradient(circle, rgba(176, 106, 90, 0.4) 0%, transparent 70%);
  box-shadow: 0 0 30px rgba(176, 106, 90, 0.5);
}

/* Note bar */
.note-bar {
  position: absolute;
  top: 50%;
  left: 50%;
  border-radius: 3px 3px 0 0;
  pointer-events: none;
  will-change: transform, opacity;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-end;

  /* Ghost appearance */
  background: linear-gradient(180deg,
    rgba(255, 255, 255, 0.03) 0%,
    rgba(255, 255, 255, 0.1) 100%);
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-bottom: none;
}

.note-bar--left {
  background: linear-gradient(180deg,
    rgba(90, 138, 176, 0.08) 0%,
    rgba(90, 138, 176, 0.2) 100%);
  border-color: rgba(90, 138, 176, 0.3);
}

.note-bar--right {
  background: linear-gradient(180deg,
    rgba(176, 106, 90, 0.08) 0%,
    rgba(176, 106, 90, 0.2) 100%);
  border-color: rgba(176, 106, 90, 0.3);
}

.note-bar--active {
  border-width: 2px;
}

.note-bar--active.note-bar--left {
  background: linear-gradient(180deg,
    rgba(90, 138, 176, 0.2) 0%,
    rgba(90, 138, 176, 0.4) 100%);
  border-color: rgba(90, 138, 176, 0.6);
  box-shadow: 0 0 15px rgba(90, 138, 176, 0.4);
}

.note-bar--active.note-bar--right {
  background: linear-gradient(180deg,
    rgba(176, 106, 90, 0.2) 0%,
    rgba(176, 106, 90, 0.4) 100%);
  border-color: rgba(176, 106, 90, 0.6);
  box-shadow: 0 0 15px rgba(176, 106, 90, 0.4);
}

.note-bar--past {
  opacity: 0.2 !important;
}

/* Realistic tone field at bottom of bar - matches handpan3d.css */
/* Width and height set via inline style based on pitch */
.note-bar__tone-field {
  position: absolute;
  bottom: 0;
  left: 50%;
  /* Size set via inline style for pitch-based scaling */
  transform-origin: center bottom;
  border-radius: 50%;

  /* Concave dimple effect - same as handpan display */
  background:
    radial-gradient(ellipse 90% 80% at 45% 40%,
      var(--handpan-tone-highlight, #6a7278) 0%,
      var(--handpan-tone-base, #4a5258) 50%,
      var(--handpan-steel-dark, #3a4044) 100%);

  box-shadow:
    inset 2px 2px 6px rgba(0, 0, 0, 0.25),
    inset -1px -1px 4px rgba(255, 255, 255, 0.08),
    0 1px 3px rgba(0, 0, 0, 0.15);
}

/* Nipple in center of tone field - like real handpan */
.note-bar__nipple {
  position: absolute;
  width: 40%;
  height: 38%;
  border-radius: 50%;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);

  background:
    radial-gradient(ellipse 85% 75% at 35% 30%,
      var(--handpan-highlight, #b8c0c8) 0%,
      var(--handpan-steel-light, #8a9299) 50%,
      var(--handpan-steel-mid, #5a6268) 100%);

  box-shadow:
    inset 1px 1px 3px rgba(255, 255, 255, 0.25),
    inset -1px -1px 2px rgba(0, 0, 0, 0.1),
    0 1px 2px rgba(0, 0, 0, 0.15);
}

/* Left hand tone field - blue tint overlay */
.note-bar__tone-field--left {
  background:
    radial-gradient(ellipse 90% 80% at 45% 40%,
      rgba(106, 159, 196, 0.7) 0%,
      rgba(74, 127, 160, 0.6) 50%,
      rgba(58, 95, 128, 0.5) 100%),
    radial-gradient(ellipse 90% 80% at 45% 40%,
      var(--handpan-tone-highlight, #6a7278) 0%,
      var(--handpan-tone-base, #4a5258) 50%,
      var(--handpan-steel-dark, #3a4044) 100%);
  box-shadow:
    inset 2px 2px 6px rgba(0, 0, 0, 0.2),
    inset -1px -1px 4px rgba(180, 220, 255, 0.15),
    0 2px 8px rgba(58, 95, 128, 0.4),
    0 0 15px rgba(90, 138, 176, 0.3);
}

.note-bar__tone-field--left .note-bar__nipple {
  background:
    radial-gradient(ellipse 85% 75% at 35% 30%,
      rgba(158, 197, 232, 0.8) 0%,
      rgba(106, 159, 196, 0.7) 50%,
      rgba(74, 127, 160, 0.6) 100%),
    radial-gradient(ellipse 85% 75% at 35% 30%,
      var(--handpan-highlight, #b8c0c8) 0%,
      var(--handpan-steel-light, #8a9299) 50%,
      var(--handpan-steel-mid, #5a6268) 100%);
}

/* Right hand tone field - warm/red tint overlay */
.note-bar__tone-field--right {
  background:
    radial-gradient(ellipse 90% 80% at 45% 40%,
      rgba(196, 122, 106, 0.7) 0%,
      rgba(160, 90, 74, 0.6) 50%,
      rgba(128, 58, 42, 0.5) 100%),
    radial-gradient(ellipse 90% 80% at 45% 40%,
      var(--handpan-tone-highlight, #6a7278) 0%,
      var(--handpan-tone-base, #4a5258) 50%,
      var(--handpan-steel-dark, #3a4044) 100%);
  box-shadow:
    inset 2px 2px 6px rgba(0, 0, 0, 0.2),
    inset -1px -1px 4px rgba(255, 200, 180, 0.15),
    0 2px 8px rgba(128, 58, 42, 0.4),
    0 0 15px rgba(176, 106, 90, 0.3);
}

.note-bar__tone-field--right .note-bar__nipple {
  background:
    radial-gradient(ellipse 85% 75% at 35% 30%,
      rgba(232, 176, 158, 0.8) 0%,
      rgba(196, 122, 106, 0.7) 50%,
      rgba(160, 90, 74, 0.6) 100%),
    radial-gradient(ellipse 85% 75% at 35% 30%,
      var(--handpan-highlight, #b8c0c8) 0%,
      var(--handpan-steel-light, #8a9299) 50%,
      var(--handpan-steel-mid, #5a6268) 100%);
}

/* Flash effect when hit */
.note-bar__tone-field--flash {
  background:
    radial-gradient(ellipse 90% 80% at 50% 45%,
      rgba(255, 255, 255, 0.9) 0%,
      rgba(255, 255, 255, 0.7) 50%,
      rgba(255, 255, 255, 0.5) 100%) !important;
  box-shadow:
    0 0 30px rgba(255, 255, 255, 0.8),
    0 0 60px rgba(255, 255, 255, 0.4) !important;
  animation: tone-field-flash 0.2s ease-out;
}

.note-bar__tone-field--flash .note-bar__nipple {
  background: radial-gradient(ellipse 85% 75% at 35% 30%,
    rgba(255, 255, 255, 1) 0%,
    rgba(255, 255, 255, 0.9) 50%,
    rgba(255, 255, 255, 0.8) 100%) !important;
}

@keyframes tone-field-flash {
  0% {
    transform: translateX(-50%) rotate(var(--rotation, 0deg)) scale(1.2);
  }
  100% {
    transform: translateX(-50%) rotate(var(--rotation, 0deg)) scale(1);
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
    rgba(0, 0, 0, 0.02) 0%,
    rgba(0, 0, 0, 0.08) 100%);
  border-color: rgba(0, 0, 0, 0.1);
}

:root[data-theme="light"] .note-bar--left {
  background: linear-gradient(180deg,
    rgba(70, 118, 156, 0.05) 0%,
    rgba(70, 118, 156, 0.15) 100%);
  border-color: rgba(70, 118, 156, 0.25);
}

:root[data-theme="light"] .note-bar--right {
  background: linear-gradient(180deg,
    rgba(156, 86, 70, 0.05) 0%,
    rgba(156, 86, 70, 0.15) 100%);
  border-color: rgba(156, 86, 70, 0.25);
}

:root[data-theme="light"] .connection-line--left {
  stroke: rgba(70, 118, 156, 0.4);
}

:root[data-theme="light"] .connection-line--right {
  stroke: rgba(156, 86, 70, 0.4);
}

:root[data-theme="light"] .target-glow--left {
  background: radial-gradient(circle, rgba(70, 118, 156, 0.3) 0%, transparent 70%);
  box-shadow: 0 0 25px rgba(70, 118, 156, 0.4);
}

:root[data-theme="light"] .target-glow--right {
  background: radial-gradient(circle, rgba(156, 86, 70, 0.3) 0%, transparent 70%);
  box-shadow: 0 0 25px rgba(156, 86, 70, 0.4);
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

  /* Scale down tone fields on tablet */
  .note-bar__tone-field {
    transform: translateX(-50%) rotate(var(--rotation, 0deg)) scale(0.77);
  }
}

@media (max-width: 480px) {
  .beat-line__label {
    display: none;
  }

  .connection-lines {
    display: none;
  }

  /* Scale down tone fields on mobile */
  .note-bar__tone-field {
    transform: translateX(-50%) rotate(var(--rotation, 0deg)) scale(0.68);
  }
}
</style>
