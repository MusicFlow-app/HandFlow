<template>
  <div class="playback-controls">
    <!-- Main control buttons -->
    <div class="playback-controls__main">
      <!-- Stop button -->
      <button
        class="control-button"
        @click="emit('stop')"
        title="Stop"
      >
        <PhStop :size="20" weight="fill" />
      </button>

      <!-- Play/Pause button -->
      <button
        class="control-button control-button--primary"
        @click="emit('toggle-play')"
        :title="isPlaying ? 'Pause' : 'Play'"
      >
        <PhPause v-if="isPlaying" :size="24" weight="fill" />
        <PhPlay v-else :size="24" weight="fill" />
      </button>
    </div>

    <!-- Progress bar -->
    <div class="playback-controls__progress">
      <span class="time-display">{{ formattedTime }}</span>

      <div
        class="progress-bar"
        ref="progressBar"
        @click="handleProgressClick"
        @mousedown="startDragging"
      >
        <!-- Loop region indicator -->
        <div
          v-if="loopEnabled && loopStart !== undefined && loopEnd !== undefined"
          class="progress-bar__loop-region"
          :style="{
            left: `${(loopStart / duration) * 100}%`,
            width: `${((loopEnd - loopStart) / duration) * 100}%`
          }"
        ></div>

        <!-- Progress fill -->
        <div
          class="progress-bar__fill"
          :style="{ width: `${progress * 100}%` }"
        ></div>

        <!-- Draggable handle -->
        <div
          class="progress-bar__handle"
          :style="{ left: `${progress * 100}%` }"
        ></div>
      </div>

      <span class="time-display">{{ formattedDuration }}</span>
    </div>

    <!-- Options -->
    <div class="playback-controls__options">
      <!-- Speed selector -->
      <div class="speed-selector" ref="speedSelector">
        <button
          class="speed-selector__button"
          @click="toggleSpeedDropdown"
        >
          <PhGauge :size="16" />
          <span>{{ currentSpeedLabel }}</span>
          <PhCaretDown :size="12" />
        </button>

        <div v-if="showSpeedDropdown" class="speed-selector__dropdown">
          <button
            v-for="option in speedOptions"
            :key="option.value"
            class="speed-selector__option"
            :class="{ 'speed-selector__option--active': speed === option.value }"
            @click="selectSpeed(option.value)"
          >
            {{ option.label }}
          </button>
        </div>
      </div>

      <!-- Loop toggle -->
      <button
        class="control-button"
        :class="{ 'control-button--active': loopEnabled }"
        @click="emit('toggle-loop')"
        title="Toggle Loop"
      >
        <PhRepeat :size="20" :weight="loopEnabled ? 'fill' : 'regular'" />
      </button>

      <!-- Zoom controls -->
      <div class="zoom-controls">
        <button
          class="control-button control-button--small"
          @click="emit('zoom-in')"
          title="Zoom In (show less measures, faster notes)"
          :disabled="measuresAhead <= minMeasures"
        >
          <PhMagnifyingGlassPlus :size="18" />
        </button>
        <span class="zoom-label">{{ measuresAhead }}m</span>
        <button
          class="control-button control-button--small"
          @click="emit('zoom-out')"
          title="Zoom Out (show more measures, slower notes)"
          :disabled="measuresAhead >= maxMeasures"
        >
          <PhMagnifyingGlassMinus :size="18" />
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue';
import {
  PhPlay,
  PhPause,
  PhStop,
  PhRepeat,
  PhGauge,
  PhCaretDown,
  PhMagnifyingGlassPlus,
  PhMagnifyingGlassMinus
} from '@phosphor-icons/vue';

const props = defineProps({
  isPlaying: {
    type: Boolean,
    default: false
  },
  currentTime: {
    type: Number,
    default: 0
  },
  duration: {
    type: Number,
    default: 0
  },
  speed: {
    type: Number,
    default: 1.0
  },
  loopEnabled: {
    type: Boolean,
    default: false
  },
  loopStart: {
    type: Number,
    default: 0
  },
  loopEnd: {
    type: Number,
    default: 0
  },
  speedOptions: {
    type: Array,
    default: () => [
      { value: 0.5, label: '0.5x' },
      { value: 0.75, label: '0.75x' },
      { value: 1.0, label: '1x' },
      { value: 1.25, label: '1.25x' },
      { value: 1.5, label: '1.5x' }
    ]
  },
  measuresAhead: {
    type: Number,
    default: 4
  },
  minMeasures: {
    type: Number,
    default: 2
  },
  maxMeasures: {
    type: Number,
    default: 16
  }
});

const emit = defineEmits([
  'toggle-play',
  'stop',
  'seek',
  'set-speed',
  'toggle-loop',
  'zoom-in',
  'zoom-out'
]);

// Refs
const progressBar = ref(null);
const speedSelector = ref(null);
const showSpeedDropdown = ref(false);
const isDragging = ref(false);

// Computed
const progress = computed(() => {
  if (props.duration === 0) return 0;
  return props.currentTime / props.duration;
});

const formattedTime = computed(() => formatTime(props.currentTime));
const formattedDuration = computed(() => formatTime(props.duration));

const currentSpeedLabel = computed(() => {
  const option = props.speedOptions.find(opt => opt.value === props.speed);
  return option ? option.label : '1x';
});

// Methods
const formatTime = (ms) => {
  const totalSeconds = Math.floor(ms / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}:${seconds.toString().padStart(2, '0')}`;
};

const handleProgressClick = (event) => {
  if (!progressBar.value) return;

  const rect = progressBar.value.getBoundingClientRect();
  const percent = (event.clientX - rect.left) / rect.width;
  const time = props.duration * Math.max(0, Math.min(1, percent));

  emit('seek', time);
};

const startDragging = (event) => {
  isDragging.value = true;
  document.addEventListener('mousemove', handleDrag);
  document.addEventListener('mouseup', stopDragging);
};

const handleDrag = (event) => {
  if (!isDragging.value || !progressBar.value) return;

  const rect = progressBar.value.getBoundingClientRect();
  const percent = (event.clientX - rect.left) / rect.width;
  const time = props.duration * Math.max(0, Math.min(1, percent));

  emit('seek', time);
};

const stopDragging = () => {
  isDragging.value = false;
  document.removeEventListener('mousemove', handleDrag);
  document.removeEventListener('mouseup', stopDragging);
};

const toggleSpeedDropdown = () => {
  showSpeedDropdown.value = !showSpeedDropdown.value;
};

const selectSpeed = (speed) => {
  emit('set-speed', speed);
  showSpeedDropdown.value = false;
};

const handleClickOutside = (event) => {
  if (speedSelector.value && !speedSelector.value.contains(event.target)) {
    showSpeedDropdown.value = false;
  }
};

// Keyboard shortcuts
const handleKeydown = (event) => {
  // Space to toggle play
  if (event.code === 'Space' && event.target.tagName !== 'INPUT') {
    event.preventDefault();
    emit('toggle-play');
  }

  // L to toggle loop
  if (event.code === 'KeyL') {
    emit('toggle-loop');
  }

  // Arrow keys for seeking
  if (event.code === 'ArrowLeft') {
    emit('seek', Math.max(0, props.currentTime - 5000));
  }
  if (event.code === 'ArrowRight') {
    emit('seek', Math.min(props.duration, props.currentTime + 5000));
  }

  // +/- for zoom
  if (event.code === 'Equal' || event.code === 'NumpadAdd') {
    emit('zoom-in');
  }
  if (event.code === 'Minus' || event.code === 'NumpadSubtract') {
    emit('zoom-out');
  }
};

// Lifecycle
onMounted(() => {
  document.addEventListener('click', handleClickOutside);
  document.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside);
  document.removeEventListener('keydown', handleKeydown);
  document.removeEventListener('mousemove', handleDrag);
  document.removeEventListener('mouseup', stopDragging);
});
</script>
