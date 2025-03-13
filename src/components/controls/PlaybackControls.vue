`<template>
  <div class="controls-section">
    <h4>Playback Controls</h4>
    <div class="playback-controls">
      <button 
        :class="['play-button', { playing: isPlaying }]"
        @click="togglePlayback"
      >
        {{ isPlaying ? '⏸ Pause' : '▶ Play' }}
      </button>
      <button 
        class="reset-button"
        @click="resetScroll"
      >
        ↪ Reset
      </button>
      <div class="toggle-switch">
        <label for="togglePlayInScale">In-Scale Notes Only:</label>
        <input 
          type="checkbox"
          id="togglePlayInScale"
          v-model="playInScaleOnly"
          @change="updatePlayInScale"
        >
        <label class="toggle-label" for="togglePlayInScale"></label>
      </div>
    </div>
    <div class="tempo-control">
      <label for="scrollRateBpm">Tempo:</label>
      <input 
        type="range"
        id="scrollRateBpm"
        v-model="bpm"
        min="0"
        max="240"
        @input="updateBpm"
      >
      <span class="value-display">{{ bpm }} BPM</span>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import { useMetronomeStore } from '@/stores/metronome'

export default {
  name: 'PlaybackControls',
  setup() {
    const metronomeStore = useMetronomeStore()
    const isPlaying = ref(false)
    const bpm = ref(120)
    const playInScaleOnly = ref(false)

    const togglePlayback = () => {
      isPlaying.value = !isPlaying.value
      document.dispatchEvent(new CustomEvent('togglePlayback', {
        detail: { isPlaying: isPlaying.value }
      }))
    }

    const resetScroll = () => {
      document.dispatchEvent(new CustomEvent('resetScroll'))
    }

    const updateBpm = () => {
      metronomeStore.setBpm(bpm.value)
      document.dispatchEvent(new CustomEvent('bpmChange', {
        detail: { bpm: bpm.value }
      }))
    }

    const updatePlayInScale = () => {
      document.dispatchEvent(new CustomEvent('playInScaleChange', {
        detail: { playInScaleOnly: playInScaleOnly.value }
      }))
    }

    onMounted(() => {
      // Sync with metronome BPM if it exists
      bpm.value = metronomeStore.bpm
    })

    return {
      isPlaying,
      bpm,
      playInScaleOnly,
      togglePlayback,
      resetScroll,
      updateBpm,
      updatePlayInScale
    }
  }
}
</script>

<style scoped>
.controls-section {
  background: #f8f9fa;
  border-radius: 8px;
  padding: 15px;
}

.playback-controls {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
  align-items: center;
  margin-bottom: 15px;
}

.play-button, .reset-button {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
  transition: all 0.2s;
}

.play-button {
  background: #4CAF50;
  color: white;
}

.play-button.playing {
  background: #f44336;
}

.reset-button {
  background: #607d8b;
  color: white;
}

.reset-button:hover {
  background: #546e7a;
}

.tempo-control {
  display: grid;
  gap: 10px;
}

.tempo-control input[type="range"] {
  width: 100%;
}

.value-display {
  font-weight: 600;
  color: #4CAF50;
}

.toggle-switch {
  display: flex;
  align-items: center;
  gap: 8px;
}

/* Toggle switch styles */
.toggle-switch input[type="checkbox"] {
  display: none;
}

.toggle-label {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 20px;
  background: #ccc;
  border-radius: 20px;
  cursor: pointer;
  transition: 0.3s;
}

.toggle-label::after {
  content: '';
  position: absolute;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: white;
  top: 1px;
  left: 1px;
  transition: 0.3s;
}

input:checked + .toggle-label {
  background: #4CAF50;
}

input:checked + .toggle-label::after {
  left: 21px;
}
</style>`
