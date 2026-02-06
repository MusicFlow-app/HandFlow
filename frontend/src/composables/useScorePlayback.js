import { ref, computed, watch, onUnmounted } from 'vue';

/**
 * Composable for managing score playback state
 * Handles play/pause, speed control, seeking, and looping
 */
export default function useScorePlayback(options = {}) {
  // Playback state
  const isPlaying = ref(false);
  const currentTime = ref(0);
  const speed = ref(1.0);
  const loopEnabled = ref(false);

  // Loop markers (in milliseconds)
  const loopStart = ref(0);
  const loopEnd = ref(0);

  // Duration from external source (set via setDuration)
  const duration = ref(options.duration || 0);

  // Animation frame reference
  let animationFrameId = null;
  let lastFrameTime = 0;

  // Available speed options
  const speedOptions = [
    { value: 0.5, label: '0.5x' },
    { value: 0.75, label: '0.75x' },
    { value: 1.0, label: '1x' },
    { value: 1.25, label: '1.25x' },
    { value: 1.5, label: '1.5x' }
  ];

  // Callbacks
  const onTimeUpdateCallbacks = [];
  const onNoteHitCallbacks = [];
  const onLoopCallbacks = [];

  /**
   * Start playback
   */
  const play = () => {
    if (isPlaying.value) return;

    isPlaying.value = true;
    lastFrameTime = performance.now();
    requestNextFrame();
  };

  /**
   * Pause playback
   */
  const pause = () => {
    isPlaying.value = false;
    if (animationFrameId) {
      cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }
  };

  /**
   * Toggle play/pause
   */
  const togglePlay = () => {
    if (isPlaying.value) {
      pause();
    } else {
      play();
    }
  };

  /**
   * Stop playback and reset to beginning
   */
  const stop = () => {
    pause();
    currentTime.value = loopEnabled.value ? loopStart.value : 0;
  };

  /**
   * Seek to a specific time
   * @param {number} time - Time in milliseconds
   */
  const seek = (time) => {
    currentTime.value = Math.max(0, Math.min(time, duration.value));
  };

  /**
   * Seek by percentage (0-1)
   * @param {number} percent - Percentage of total duration
   */
  const seekPercent = (percent) => {
    seek(duration.value * Math.max(0, Math.min(1, percent)));
  };

  /**
   * Set playback speed
   * @param {number} newSpeed - Speed multiplier (e.g., 0.5, 1.0, 1.5)
   */
  const setSpeed = (newSpeed) => {
    const validSpeed = speedOptions.find(opt => opt.value === newSpeed);
    if (validSpeed) {
      speed.value = newSpeed;
    }
  };

  /**
   * Toggle loop mode
   */
  const toggleLoop = () => {
    loopEnabled.value = !loopEnabled.value;
  };

  /**
   * Set loop region
   * @param {number} start - Loop start time in ms
   * @param {number} end - Loop end time in ms
   */
  const setLoopRegion = (start, end) => {
    loopStart.value = Math.max(0, start);
    loopEnd.value = Math.min(end, duration.value);
  };

  /**
   * Clear loop region
   */
  const clearLoopRegion = () => {
    loopStart.value = 0;
    loopEnd.value = duration.value;
  };

  /**
   * Set total duration
   * @param {number} dur - Duration in milliseconds
   */
  const setDuration = (dur) => {
    duration.value = dur;
    if (loopEnd.value === 0 || loopEnd.value > dur) {
      loopEnd.value = dur;
    }
  };

  /**
   * Animation frame update loop
   */
  const update = (timestamp) => {
    if (!isPlaying.value) return;

    // Calculate delta time
    const deltaTime = (timestamp - lastFrameTime) * speed.value;
    lastFrameTime = timestamp;

    // Update current time
    const newTime = currentTime.value + deltaTime;

    // Check for loop
    const effectiveEnd = loopEnabled.value ? loopEnd.value : duration.value;

    if (newTime >= effectiveEnd) {
      if (loopEnabled.value) {
        // Loop back to start
        currentTime.value = loopStart.value;
        onLoopCallbacks.forEach(cb => cb());
      } else {
        // Reached end, stop
        currentTime.value = duration.value;
        pause();
        return;
      }
    } else {
      currentTime.value = newTime;
    }

    // Notify time update listeners
    onTimeUpdateCallbacks.forEach(cb => cb(currentTime.value));

    // Request next frame
    requestNextFrame();
  };

  /**
   * Request next animation frame
   */
  const requestNextFrame = () => {
    animationFrameId = requestAnimationFrame(update);
  };

  /**
   * Register a callback for time updates
   * @param {Function} callback - Function to call with current time
   * @returns {Function} Unsubscribe function
   */
  const onTimeUpdate = (callback) => {
    onTimeUpdateCallbacks.push(callback);
    return () => {
      const index = onTimeUpdateCallbacks.indexOf(callback);
      if (index > -1) onTimeUpdateCallbacks.splice(index, 1);
    };
  };

  /**
   * Register a callback for note hits
   * @param {Function} callback - Function to call when a note hits
   * @returns {Function} Unsubscribe function
   */
  const onNoteHit = (callback) => {
    onNoteHitCallbacks.push(callback);
    return () => {
      const index = onNoteHitCallbacks.indexOf(callback);
      if (index > -1) onNoteHitCallbacks.splice(index, 1);
    };
  };

  /**
   * Register a callback for loop events
   * @param {Function} callback - Function to call when loop restarts
   * @returns {Function} Unsubscribe function
   */
  const onLoop = (callback) => {
    onLoopCallbacks.push(callback);
    return () => {
      const index = onLoopCallbacks.indexOf(callback);
      if (index > -1) onLoopCallbacks.splice(index, 1);
    };
  };

  /**
   * Trigger note hit callbacks
   * @param {Object} noteEvent - The note event that was hit
   */
  const triggerNoteHit = (noteEvent) => {
    onNoteHitCallbacks.forEach(cb => cb(noteEvent));
  };

  // Computed properties
  const progress = computed(() => {
    if (duration.value === 0) return 0;
    return currentTime.value / duration.value;
  });

  const formattedTime = computed(() => {
    return formatTime(currentTime.value);
  });

  const formattedDuration = computed(() => {
    return formatTime(duration.value);
  });

  const isAtEnd = computed(() => {
    return currentTime.value >= duration.value;
  });

  /**
   * Format milliseconds to mm:ss.ms
   * @param {number} ms - Time in milliseconds
   * @returns {string} Formatted time string
   */
  const formatTime = (ms) => {
    const totalSeconds = Math.floor(ms / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    const milliseconds = Math.floor((ms % 1000) / 10);

    return `${minutes}:${seconds.toString().padStart(2, '0')}.${milliseconds.toString().padStart(2, '0')}`;
  };

  /**
   * Cleanup on unmount
   */
  const cleanup = () => {
    pause();
    onTimeUpdateCallbacks.length = 0;
    onNoteHitCallbacks.length = 0;
    onLoopCallbacks.length = 0;
  };

  // Auto cleanup on unmount
  onUnmounted(cleanup);

  return {
    // State
    isPlaying,
    currentTime,
    speed,
    loopEnabled,
    loopStart,
    loopEnd,
    duration,

    // Computed
    progress,
    formattedTime,
    formattedDuration,
    isAtEnd,

    // Speed options
    speedOptions,

    // Control methods
    play,
    pause,
    togglePlay,
    stop,
    seek,
    seekPercent,
    setSpeed,
    toggleLoop,
    setLoopRegion,
    clearLoopRegion,
    setDuration,

    // Event callbacks
    onTimeUpdate,
    onNoteHit,
    onLoop,
    triggerNoteHit,

    // Utility
    formatTime,
    cleanup
  };
}
