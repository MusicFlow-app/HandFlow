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

    <!-- Amber concentric halo indicators for approaching notes -->
    <div
      v-for="event in haloEvents"
      :key="`halo-${event.id}`"
      class="approach-halo"
      :class="[
        `approach-halo--${event.hand}`,
        { 'approach-halo--hit': event.haloProgress >= 1 }
      ]"
      :style="getHaloStyle(event)"
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
        { 'note-bar--landed': event.isLanded },
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
  },
  leadInMs: {
    type: Number,
    default: 3000
  }
});

const emit = defineEmits(['note-hit', 'note-approaching']);

const overlayRef = ref(null);
const hitNotes = ref(new Set());

// Pixels per millisecond (fall speed)
const pixelsPerMs = computed(() => props.fallHeight / props.leadTime);

// Halo timing constants
const HALO_START_TIME = 800; // Start showing halo 800ms before hit
const HALO_START_SCALE = 1.2; // Start at 120% of target size

// Build fixed lane grid from notePositions (stable across all measures)
// This ensures all notes snap to consistent handpan mapping
const laneGrid = computed(() => {
  const grid = {};
  props.notePositions.forEach((pos, index) => {
    grid[index] = {
      x: pos.x,
      y: pos.y,
      rotation: pos.rotation || 0,
      scale: pos.scale || 1.0,
      isDing: pos.isDing || false
    };
  });
  return grid;
});

// Get fixed lane position for a note (from pre-computed grid)
const getLanePosition = (handpanNoteIndex) => {
  const lane = laneGrid.value[handpanNoteIndex];
  if (lane) {
    return lane;
  }
  // Fallback for unmapped notes
  return { x: 0, y: 0, rotation: 0, scale: 1.0, isDing: false };
};

// Get scale factor from fixed lane grid
const getScaleForNoteIndex = (handpanNoteIndex) => {
  return getLanePosition(handpanNoteIndex).scale;
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

// Get position for a handpan note index (uses fixed lane grid)
const getNotePosition = (noteIndex) => {
  return getLanePosition(noteIndex);
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
      // Note is "landed" when it has reached the target (clamped at handpan)
      const timeOffset = event.absoluteTime - props.currentTime;
      const isLanded = timeOffset <= 0;
      return { ...event, isHit, isActive, isPast, isLanded };
    });
});

// Events that get an amber halo (within HALO_START_TIME of hitting)
const haloEvents = computed(() => {
  return props.events
    .filter(event => {
      const timeOffset = event.absoluteTime - props.currentTime;
      // Show halo from HALO_START_TIME before hit until note is past
      return timeOffset > -100 && timeOffset < HALO_START_TIME;
    })
    .map(event => {
      const timeOffset = event.absoluteTime - props.currentTime;
      // haloProgress: 0 = just appeared, 1 = hit moment
      const haloProgress = 1 - Math.max(0, timeOffset / HALO_START_TIME);
      // haloScale: starts at HALO_START_SCALE, shrinks to 1.0 at hit
      const haloScale = HALO_START_SCALE - (haloProgress * (HALO_START_SCALE - 1));
      return { ...event, haloProgress, haloScale };
    });
});

// Style for note bars
const getNoteBarStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);

  // Use handpanNoteIndex for rank-based sizing (same as handpan display)
  const toneFieldSize = getToneFieldSize(noteIndex);
  const barWidth = getBarWidth(noteIndex);

  // Calculate bar height based on remaining duration
  const fullDuration = event.duration || 500;
  const timeOffset = event.absoluteTime - props.currentTime;

  let barHeight;
  if (timeOffset >= 0) {
    // Note hasn't landed yet - show full duration
    barHeight = durationToPixels(fullDuration);
  } else {
    // Note is playing - shrink bar based on remaining time
    const elapsedTime = -timeOffset; // How long since note started
    const remainingDuration = Math.max(0, fullDuration - elapsedTime);
    barHeight = durationToPixels(remainingDuration);
  }

  // NORMALIZED Y POSITION:
  // All notes at the same timeOffset appear at the same visual Y level
  // This ensures visual order matches playback order
  // The tone field offset (targetPos.y) is handled separately in getToneFieldStyle
  const normalizedY = -(timeOffset * pixelsPerMs.value);

  // CLAMP: Never let the note go below y=0 (the reference point)
  const bottomY = Math.min(normalizedY, 0);

  // X position: FIXED to target lane throughout the fall
  const currentX = targetPos.x;

  // Fade out notes that are too far up (bottomY very negative = high up)
  let opacity = 1;
  const distanceFromTarget = Math.abs(bottomY - targetPos.y);
  if (distanceFromTarget > props.fallHeight - 50) {
    opacity = Math.max(0, (props.fallHeight - distanceFromTarget) / 50);
  }

  // Fade out landed notes that are past their duration (let handpan hit effect take over)
  if (event.isLanded && event.isPast) {
    opacity = 0.15;
  } else if (event.isLanded && event.isActive) {
    // Active notes stay visible but slightly dimmed
    opacity = 0.8;
  }

  return {
    '--bar-width': `${barWidth}px`,
    '--bar-height': `${barHeight}px`,
    '--tone-field-width': `${toneFieldSize.width}px`,
    '--tone-field-height': `${toneFieldSize.height}px`,
    '--tx': `${currentX}px`,
    '--ty': `${bottomY}px`,
    width: `var(--bar-width)`,
    height: `var(--bar-height)`,
    transform: `translate(calc(-50% + var(--tx)), var(--ty))`,
    opacity,
    zIndex: event.isActive ? 100 : 50
  };
};

// Style for the tone field at bottom of bar - matches handpan note exactly
// The tone field has an additional Y offset to land at the correct target position
// This offset compensates for the normalized Y position of the note bar
const getToneFieldStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);
  const size = getToneFieldSize(noteIndex);

  // Target Y offset - this makes the tone field land at the correct handpan position
  // The note bar is at normalized Y (same for all notes at same time)
  // The tone field offset positions it at the actual target
  const targetYOffset = targetPos.y;

  // Ding is circular, no rotation needed
  if (targetPos.isDing) {
    return {
      '--rotation': '0deg',
      '--target-y-offset': `${targetYOffset}px`,
      width: `${size.width}px`,
      height: `${size.height}px`,
      borderRadius: '50%',
      transform: `translateX(-50%) translateY(var(--target-y-offset))`
    };
  }

  // Tone fields are elliptical with rotation matching handpan
  return {
    '--rotation': `${targetPos.rotation || 0}deg`,
    '--target-y-offset': `${targetYOffset}px`,
    width: `${size.width}px`,
    height: `${size.height}px`,
    transform: `translateX(-50%) translateY(var(--target-y-offset)) rotate(var(--rotation))`
  };
};

// Amber halo style - concentric shrinking halo around target
// Halo is positioned slightly above the tone field center for earlier visual cue
// Halo now matches tone field shape: elliptical with same rotation
const HALO_Y_OFFSET = -15; // Pixels above tone field center

const getHaloStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);
  const size = getToneFieldSize(noteIndex);

  // Use ellipse dimensions (matching tone field), not just max size
  const currentScale = event.haloScale;
  const haloWidth = size.width * currentScale;
  const haloHeight = size.height * currentScale;

  // Rotation matches the tone field (ding has no rotation)
  const rotation = targetPos.isDing ? 0 : (targetPos.rotation || 0);

  // Opacity: fade in during first 20% of approach, stays visible until hit
  let opacity = 1;
  if (event.haloProgress < 0.2) {
    opacity = event.haloProgress / 0.2;
  }
  // Flash brighter at hit moment
  if (event.haloProgress >= 0.95) {
    opacity = 1.2;
  }

  return {
    '--tx': `${targetPos.x}px`,
    '--ty': `${targetPos.y + HALO_Y_OFFSET}px`,
    '--rotation': `${rotation}deg`,
    '--halo-width': `${haloWidth}px`,
    '--halo-height': `${haloHeight}px`,
    '--halo-opacity': opacity * 0.7,
    '--halo-progress': event.haloProgress,
    transform: `translate(calc(-50% + var(--tx)), calc(50% + var(--ty))) rotate(var(--rotation))`,
    width: `var(--halo-width)`,
    height: `var(--halo-height)`,
    opacity: `var(--halo-opacity)`
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
    // Center-based: top:50% is the reference, ty moves from there
    transform: `translateY(var(--ty))`,
    opacity
  };
};

// Generate beat markers (starting after lead-in)
const allBeatMarkers = computed(() => {
  if (props.scoreDuration <= 0 || props.tempo <= 0) return [];

  const markers = [];
  const msPerBeat = 60000 / props.tempo;
  const beatsPerMeasure = props.timeSignature.beats || 4;
  const msPerMeasure = msPerBeat * beatsPerMeasure;

  let measureNumber = 1;
  // Start beat markers after the lead-in time
  let time = props.leadInMs;

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

// Track the last processed time to detect seeks/resets
const lastProcessedTime = ref(0);

// Check for note hits - trigger when falling tone field visually overlaps target
const checkForHits = () => {
  const currentTime = props.currentTime;
  const prevTime = lastProcessedTime.value;

  // Detect if we seeked backwards - clear hit notes to allow replay
  if (currentTime < prevTime - 100) {
    hitNotes.value.clear();
  }

  lastProcessedTime.value = currentTime;

  props.events.forEach(event => {
    // Skip if already played
    if (hitNotes.value.has(event.id)) return;

    const timeOffset = event.absoluteTime - currentTime;

    // Fire when the falling tone field center reaches the handpan tone field center
    // Add small delay (-30ms) to ensure visual has caught up with timing
    // timeOffset <= -30 means we're 30ms past the scheduled time
    if (timeOffset <= -30 && timeOffset > -150) {
      hitNotes.value.add(event.id);
      emit('note-hit', event);
    }
  });
};

watch(() => props.currentTime, () => {
  checkForHits();
});

// Clear hit notes when events change (new score loaded)
watch(() => props.events, () => {
  hitNotes.value.clear();
  lastProcessedTime.value = 0;
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

/* Amber concentric halo for approaching notes */
.approach-halo {
  position: absolute;
  bottom: 250px; /* Aligned with handpan center */
  left: 50%;
  border-radius: 50%;
  pointer-events: none;
  z-index: 8;
  /* Amber/gold color for anticipation cue */
  background: radial-gradient(circle,
    transparent 40%,
    rgba(255, 191, 0, 0.15) 50%,
    rgba(255, 170, 0, 0.3) 70%,
    rgba(255, 150, 0, 0.15) 85%,
    transparent 100%);
  box-shadow:
    0 0 20px rgba(255, 180, 0, 0.2),
    inset 0 0 10px rgba(255, 200, 50, 0.1);
  transition: opacity 0.05s ease-out;
}

/* Hand-colored halos (slight tint variation) */
.approach-halo--left {
  background: radial-gradient(circle,
    transparent 40%,
    rgba(200, 180, 80, 0.15) 50%,
    rgba(220, 170, 60, 0.3) 70%,
    rgba(200, 160, 50, 0.15) 85%,
    transparent 100%);
  box-shadow:
    0 0 20px rgba(200, 180, 80, 0.25),
    inset 0 0 10px rgba(220, 200, 100, 0.1);
}

.approach-halo--right {
  background: radial-gradient(circle,
    transparent 40%,
    rgba(255, 180, 80, 0.15) 50%,
    rgba(255, 160, 60, 0.35) 70%,
    rgba(255, 140, 50, 0.15) 85%,
    transparent 100%);
  box-shadow:
    0 0 20px rgba(255, 170, 60, 0.25),
    inset 0 0 10px rgba(255, 190, 100, 0.1);
}

/* Flash effect when halo reaches hit moment */
.approach-halo--hit {
  background: radial-gradient(circle,
    rgba(255, 255, 200, 0.3) 0%,
    rgba(255, 220, 100, 0.5) 40%,
    rgba(255, 180, 50, 0.3) 70%,
    transparent 100%);
  box-shadow:
    0 0 40px rgba(255, 200, 50, 0.6),
    0 0 80px rgba(255, 180, 0, 0.3);
}

/* Note bar */
.note-bar {
  position: absolute;
  bottom: 250px; /* Reference point aligned with handpan center (80px padding + ~250px radius) */
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

/* Landed notes (clamped at handpan, waiting for duration to complete) */
.note-bar--landed {
  /* Subtle glow when note is at target */
  filter: brightness(1.1);
}

.note-bar--landed.note-bar--active {
  /* Active landed notes have enhanced glow */
  filter: brightness(1.2);
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
  bottom: 250px; /* Aligned with handpan center (80px padding + ~250px radius) */
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

/* Light theme halos - slightly darker amber for visibility */
:root[data-theme="light"] .approach-halo {
  background: radial-gradient(circle,
    transparent 40%,
    rgba(220, 160, 0, 0.2) 50%,
    rgba(200, 140, 0, 0.4) 70%,
    rgba(180, 120, 0, 0.2) 85%,
    transparent 100%);
  box-shadow:
    0 0 20px rgba(200, 150, 0, 0.3),
    inset 0 0 10px rgba(220, 180, 50, 0.15);
}

:root[data-theme="light"] .approach-halo--hit {
  background: radial-gradient(circle,
    rgba(255, 240, 180, 0.4) 0%,
    rgba(255, 200, 80, 0.6) 40%,
    rgba(220, 160, 40, 0.4) 70%,
    transparent 100%);
  box-shadow:
    0 0 40px rgba(220, 170, 40, 0.7),
    0 0 80px rgba(200, 150, 0, 0.4);
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

  /* Scale down tone fields on tablet - maintain center rotation and target offset */
  .note-bar__tone-field {
    transform: translateX(-50%) translateY(var(--target-y-offset, 0px)) rotate(var(--rotation, 0deg)) scale(0.77);
    bottom: calc(-0.5 * var(--tone-field-height) * 0.77);
  }

  /* Ding doesn't rotate but still scales */
  .note-bar__tone-field--ding {
    transform: translateX(-50%) translateY(var(--target-y-offset, 0px)) scale(0.77);
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

  /* Scale down tone fields on mobile - maintain center rotation and target offset */
  .note-bar__tone-field {
    transform: translateX(-50%) translateY(var(--target-y-offset, 0px)) rotate(var(--rotation, 0deg)) scale(0.68);
    bottom: calc(-0.5 * var(--tone-field-height) * 0.68);
  }

  /* Ding doesn't rotate but still scales */
  .note-bar__tone-field--ding {
    transform: translateX(-50%) translateY(var(--target-y-offset, 0px)) scale(0.68);
    bottom: calc(-0.5 * var(--tone-field-height) * 0.68);
  }
}
</style>
