`<template>
  <div class="controls-section">
    <h4>Metronome</h4>
    <div class="metronome-main">
      <button 
        :class="['metronome-button', { active: isPlaying }]"
        @click="toggleMetronome"
      >
        {{ isPlaying ? 'Stop' : 'Start' }}
      </button>
    </div>
    <div class="metronome-settings">
      <div class="time-signature">
        <label for="beatsPerBar">Time Signature:</label>
        <select 
          id="beatsPerBar"
          v-model="beatsPerBar"
          @change="updateTimeSignature"
        >
          <option value="2">2/4</option>
          <option value="3">3/4</option>
          <option value="4">4/4</option>
          <option value="6">6/8</option>
          <option value="8">8/8</option>
        </select>
      </div>
      <div class="volume-control">
        <label for="metronomeVolume">Volume:</label>
        <input 
          type="range" 
          id="metronomeVolume"
          v-model="volume"
          min="0" 
          max="100"
          @input="updateVolume"
        >
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, onUnmounted } from 'vue'
import { useMetronomeStore } from '@/stores/metronome'

export default {
  name: 'MetronomeControls',
  setup() {
    const store = useMetronomeStore()
    const isPlaying = ref(false)
    const beatsPerBar = ref(4)
    const volume = ref(80)

    const toggleMetronome = () => {
      isPlaying.value = !isPlaying.value
      store.toggleMetronome()
    }

    const updateTimeSignature = () => {
      store.setTimeSignature(beatsPerBar.value)
    }

    const updateVolume = () => {
      store.setVolume(volume.value)
    }

    onMounted(() => {
      store.initializeMetronome()
    })

    onUnmounted(() => {
      store.cleanup()
    })

    return {
      isPlaying,
      beatsPerBar,
      volume,
      toggleMetronome,
      updateTimeSignature,
      updateVolume
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

.metronome-button {
  background: #4CAF50;
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
  transition: background-color 0.2s;
}

.metronome-button:hover {
  background: #45a049;
}

.metronome-button.active {
  background: #f44336;
}

.metronome-settings {
  margin-top: 15px;
  display: grid;
  gap: 15px;
}

.time-signature select {
  padding: 5px;
  border-radius: 4px;
  border: 1px solid #ddd;
}

.volume-control input[type="range"] {
  width: 100%;
  margin: 10px 0;
}
</style>`
