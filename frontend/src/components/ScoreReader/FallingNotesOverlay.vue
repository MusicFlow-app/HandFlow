<template>
  <div class="falling-notes-overlay" ref="overlayRef">
    <!-- Falling notes that look like tone fields -->
    <div
      v-for="event in visibleEvents"
      :key="event.id"
      class="falling-tone-field"
      :class="[
        `falling-tone-field--${event.hand}`,
        `falling-tone-field--${event.noteType}`,
        { 'falling-tone-field--hit': event.isHit }
      ]"
      :style="getFallingNoteStyle(event)"
    >
      <div class="falling-tone-field__inner" :style="getNoteRotationStyle(event)"></div>
    </div>
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
  // Handpan center position relative to overlay
  handpanCenter: {
    type: Object,
    default: () => ({ x: 0, y: 0 })
  },
  // Height from which notes start falling (pixels above handpan center)
  fallHeight: {
    type: Number,
    default: 400
  }
});

const emit = defineEmits(['note-hit']);

// Refs
const overlayRef = ref(null);
const hitNotes = ref(new Set());

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

// Get the position data for a specific handpan note index
const getNotePosition = (noteIndex) => {
  if (noteIndex < 0 || noteIndex >= props.notePositions.length) {
    return { x: 0, y: 0, rotation: 0 };
  }
  return props.notePositions[noteIndex] || { x: 0, y: 0, rotation: 0 };
};

// Calculate the style for a falling note
const getFallingNoteStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);

  // Calculate progress: 0 = just appeared (top), 1 = at target (hit)
  const timeOffset = event.absoluteTime - props.currentTime;
  const progress = 1 - (timeOffset / props.leadTime);

  // Clamp progress between 0 and 1 for notes approaching
  // Allow > 1 for notes that have passed (trail)
  const clampedProgress = Math.max(0, progress);

  // Calculate Y position: start at -fallHeight, end at target Y
  // We use the handpan center as reference point
  const startY = -props.fallHeight;
  const endY = targetPos.y;
  const currentY = startY + (endY - startY) * clampedProgress;

  // X position stays constant (directly above target)
  const currentX = targetPos.x;

  // Scale note as it approaches (optional: smaller when far, full size when near)
  const scale = 0.6 + (clampedProgress * 0.4);

  // Opacity: fade in as it appears, fade out after hit
  let opacity = 1;
  if (clampedProgress < 0.1) {
    opacity = clampedProgress / 0.1;
  } else if (progress > 1) {
    // Fade out after passing
    opacity = Math.max(0, 1 - (progress - 1) * 5);
  }

  return {
    '--tx': `${currentX}px`,
    '--ty': `${currentY}px`,
    '--scale': scale,
    '--opacity': opacity,
    transform: `translate(calc(-50% + var(--tx)), calc(-50% + var(--ty))) scale(var(--scale))`,
    opacity: `var(--opacity)`,
    zIndex: Math.floor(progress * 100)
  };
};

// Get the rotation style for the inner note element (matches handpan note rotation)
const getNoteRotationStyle = (event) => {
  const noteIndex = event.handpanNoteIndex >= 0 ? event.handpanNoteIndex : 0;
  const targetPos = getNotePosition(noteIndex);

  return {
    transform: `rotate(${targetPos.rotation || 0}deg)`
  };
};

// Check for notes that should trigger hits
const checkForHits = () => {
  // Hit tolerance: when note is within this range of its target time
  const hitTolerance = 50; // ms

  props.events.forEach(event => {
    const timeOffset = event.absoluteTime - props.currentTime;

    // Check if note is at hit time (within tolerance of reaching target)
    if (Math.abs(timeOffset) <= hitTolerance) {
      if (!hitNotes.value.has(event.id)) {
        hitNotes.value.add(event.id);
        emit('note-hit', event);

        // Remove hit status after animation
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

/* Falling note styled like a tone field */
.falling-tone-field {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 80px;
  height: 80px;
  pointer-events: none;
  will-change: transform, opacity;
}

.falling-tone-field__inner {
  width: 65px;
  height: 52px;
  margin: auto;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);

  /* Elliptical shape like tone field */
  border-radius: 50%;

  /* Steel-like appearance */
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

/* Hit animation */
.falling-tone-field--hit .falling-tone-field__inner {
  animation: falling-note-hit 0.3s ease-out forwards;
}

@keyframes falling-note-hit {
  0% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1);
  }
  50% {
    opacity: 0.8;
    transform: translate(-50%, -50%) scale(1.4);
    box-shadow:
      inset 2px 2px 6px rgba(255, 255, 255, 0.5),
      inset -2px -2px 4px rgba(0, 0, 0, 0.3),
      0 0 30px rgba(255, 255, 255, 0.6);
  }
  100% {
    opacity: 0;
    transform: translate(-50%, -50%) scale(0.8);
  }
}

/* Responsive */
@media (max-width: 768px) {
  .falling-tone-field {
    width: 60px;
    height: 60px;
  }

  .falling-tone-field__inner {
    width: 50px;
    height: 40px;
  }

  .falling-tone-field--grace .falling-tone-field__inner {
    width: 38px;
    height: 30px;
  }
}

@media (max-width: 480px) {
  .falling-tone-field {
    width: 50px;
    height: 50px;
  }

  .falling-tone-field__inner {
    width: 42px;
    height: 34px;
  }
}
</style>
