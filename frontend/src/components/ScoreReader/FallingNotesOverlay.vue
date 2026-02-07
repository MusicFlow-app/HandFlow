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
        { 'note-bar--past': event.isPast },
        { 'note-bar--ding': event.handpanNoteIndex === 0 }
      ]"
      :style="getNoteBarStyle(event)"
    >
      <!-- Realistic tone field shape at the bottom (hit zone) -->
      <div
        class="note-bar__tone-field"
        :class="[
          `note-bar__tone-field--${event.hand}`,
          { 'note-bar__tone-field--flash': event.isHit },
          { 'note-bar__tone-field--ding': event.handpanNoteIndex === 0 }
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

// Get scale factor from notePositions (passed from ScoreReader, matches handpan exactly)
const getScaleForNoteIndex = (handpanNoteIndex) => {
  const notePos = props.notePositions[handpanNoteIndex];
  if (notePos && notePos.scale !== undefined) {
    return notePos.scale;
  }
  return 1.0;
};

// Bar width based on scale from handpan
const getBarWidth = (handpanNoteIndex) => {
  const notePos = props.notePositions[handpanNoteIndex];
  // Ding uses a wider bar to match its 110px width
  if (notePos?.isDing) {
    return 30; // Wider bar for ding (proportional to 110px width)
  }
  const scale = getScaleForNoteIndex(handpanNoteIndex);
  const baseWidth = 20;
  return baseWidth * scale;
};

// Tone field dimensions - match handpan exactly
const getToneFieldSize = (handpanNoteIndex) => {
  const notePos = props.notePositions[handpanNoteIndex];
  // Ding is elliptical (110x90 on handpan, matching handpan3d.css .ding-note)
  if (notePos?.isDing) {
    return { width: 110, height: 90 };
  }
  // Tone fields are 65x52 base, scaled by the handpan's scale factor
  const scale = getScaleForNoteIndex(handpanNoteIndex);
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

  // Y position: tone field (bottom of bar) aligns with target at hit time
  const timeOffset = event.absoluteTime - props.currentTime;
  // bottomY = position of tone field (0 at hit time, negative when approaching)
  const bottomY = targetPos.y - (timeOffset * pixelsPerMs.value);

  const currentX = targetPos.x;

  // Fade out notes that are too far up
  let opacity = 1;
  const distanceFromTop = bottomY + props.fallHeight;
  if (distanceFromTop < 50) {
    opacity = Math.max(0, distanceFromTop / 50);
  }

  return {
    '--bar-width': `${barWidth}px`,
    '--bar-height': `${barHeight}px`,
    '--tone-field-width': `${toneFieldSize.width}px`,
    '--tone-field-height': `${toneFieldSize.height}px`,
    '--tx': `${currentX}px`,
    // With bottom-based CSS positioning, use bottomY (tone field position)
    '--ty': `${bottomY}px`,
    width: `var(--bar-width)`,
    height: `var(--bar-height)`,
    // Transform positions the tone field (bar bottom) at the target
    transform: `translate(calc(-50% + var(--tx)), var(--ty))`,
    opacity,
    zIndex: event.isActive ? 100 : 50
  };
};

// Style for the tone field at bottom of bar - matches handpan note exactly
const getToneFieldStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);
  const size = getToneFieldSize(noteIndex);

  // Ding is circular, no rotation needed
  if (targetPos.isDing) {
    return {
      '--rotation': '0deg',
      width: `${size.width}px`,
      height: `${size.height}px`,
      borderRadius: '50%',
      transform: `translateX(-50%)`
    };
  }

  // Tone fields are elliptical with rotation matching handpan
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

  // SVG uses bottom-based positioning matching the overlay
  // Reference is 200px from bottom, inverted for SVG (which uses top-based coords)
  // Use calc(100% - 200px + offset) to convert from bottom-based to SVG coords
  return {
    x1: `calc(50% + ${targetPos.x}px)`,
    y1: `calc(100% - 200px - ${bottomY}px)`,
    x2: `calc(50% + ${targetPos.x}px)`,
    y2: `calc(100% - 200px - ${targetPos.y}px)`
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
    // With bottom-based positioning, use +50% for vertical centering
    transform: `translate(calc(-50% + var(--tx)), calc(50% + var(--ty)))`,
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
    // With bottom-based positioning, use +50% for vertical centering
    transform: `translateY(calc(50% + var(--ty)))`,
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

// Check for note hits based on visual position (BPM-independent)
const checkForHits = () => {
  props.events.forEach(event => {
    const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
    const targetPos = getNotePosition(noteIndex);

    // Calculate visual position of the note's tone field
    const timeOffset = event.absoluteTime - props.currentTime;
    const bottomY = targetPos.y - (timeOffset * pixelsPerMs.value);

    // Trigger when note overlaps its target tone field position
    // bottomY equals targetPos.y at hit time (when timeOffset = 0)
    const hitWindow = 5; // pixels before target
    const passedWindow = 50; // pixels past target

    // Check if bottomY is near targetPos.y (the handpan note position)
    if (bottomY >= targetPos.y - hitWindow && bottomY < targetPos.y + passedWindow) {
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
  bottom: 200px; /* Aligned with handpan center */
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
  bottom: 200px; /* Reference point aligned with handpan center at bottom */
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

/* Ding notes have a wider bar */
.note-bar--ding {
  border-radius: 6px 6px 0 0;
}

/* Realistic tone field at bottom of bar - matches handpan3d.css */
/* Width and height set via inline style based on pitch */
.note-bar__tone-field {
  position: absolute;
  /* Position center at bar's bottom edge, so rotation is centered */
  bottom: calc(-0.5 * var(--tone-field-height));
  left: 50%;
  /* Size set via inline style for pitch-based scaling */
  transform-origin: center center;
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

/* Ding tone field - elliptical, matches handpan3d.css .ding-note exactly */
.note-bar__tone-field--ding {
  /* Raised dome effect matching handpan ding */
  background:
    radial-gradient(ellipse 80% 70% at 40% 35%,
      var(--handpan-highlight, #b8c0c8) 0%,
      var(--handpan-steel-light, #8a9299) 30%,
      var(--handpan-steel-mid, #5a6268) 60%,
      var(--handpan-steel-dark, #3a4044) 100%);
  box-shadow:
    0 4px 12px rgba(0, 0, 0, 0.3),
    inset 0 2px 8px rgba(255, 255, 255, 0.15),
    inset 0 -3px 8px rgba(0, 0, 0, 0.2);
}

.note-bar__tone-field--ding .note-bar__nipple {
  /* Ding nipple dome matching handpan - 44x34 relative to 110x90 = ~40% x 38% */
  width: 40%;
  height: 38%;
  background:
    radial-gradient(ellipse 90% 80% at 35% 30%,
      var(--handpan-highlight, #b8c0c8) 0%,
      var(--handpan-steel-light, #8a9299) 40%,
      var(--handpan-steel-mid, #5a6268) 100%);
  box-shadow:
    inset 1px 1px 4px rgba(255, 255, 255, 0.3),
    inset -1px -1px 4px rgba(0, 0, 0, 0.15),
    0 1px 3px rgba(0, 0, 0, 0.2);
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
    /* Animation still uses center-based rotation */
  }
  100% {
    transform: translateX(-50%) rotate(var(--rotation, 0deg)) scale(1);
  }
}

/* Beat grid lines */
.beat-line {
  position: absolute;
  bottom: 200px; /* Aligned with handpan center */
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

  /* Scale down tone fields on tablet - maintain center rotation */
  .note-bar__tone-field {
    transform: translateX(-50%) rotate(var(--rotation, 0deg)) scale(0.77);
    bottom: calc(-0.5 * var(--tone-field-height) * 0.77);
  }

  /* Ding doesn't rotate but still scales */
  .note-bar__tone-field--ding {
    transform: translateX(-50%) scale(0.77);
    bottom: calc(-0.5 * var(--tone-field-height) * 0.77);
  }
}

@media (max-width: 480px) {
  .beat-line__label {
    display: none;
  }

  .connection-lines {
    display: none;
  }

  /* Scale down tone fields on mobile - maintain center rotation */
  .note-bar__tone-field {
    transform: translateX(-50%) rotate(var(--rotation, 0deg)) scale(0.68);
    bottom: calc(-0.5 * var(--tone-field-height) * 0.68);
  }

  /* Ding doesn't rotate but still scales */
  .note-bar__tone-field--ding {
    transform: translateX(-50%) scale(0.68);
    bottom: calc(-0.5 * var(--tone-field-height) * 0.68);
  }
}
</style>
