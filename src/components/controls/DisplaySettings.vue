`<template>
  <div class="controls-section">
    <h4>Display Settings</h4>
    <div class="zoom">
      <label>Handpan Size:</label>
      <button 
        class="decrease" 
        title="Decrease size"
        @click="decreaseSize"
      >-</button>
      <button 
        class="increase" 
        title="Increase size"
        @click="increaseSize"
      >+</button>
    </div>
    <div class="toggle-container">
      <div class="toggle-switch">
        <label for="inlineDisplay">Inline Display:</label>
        <input 
          type="checkbox"
          id="inlineDisplay"
          v-model="inlineDisplay"
          @change="updateDisplay"
        >
        <label class="toggle-label" for="inlineDisplay"></label>
      </div>
      <div class="toggle-switch">
        <label for="showRestColor">Rest Colors:</label>
        <input 
          type="checkbox"
          id="showRestColor"
          v-model="showRestColor"
          @change="updateDisplay"
        >
        <label class="toggle-label" for="showRestColor"></label>
      </div>
      <div class="toggle-switch">
        <label for="showSvg">Handpan View:</label>
        <input 
          type="checkbox"
          id="showSvg"
          v-model="showSvg"
          @change="updateDisplay"
        >
        <label class="toggle-label" for="showSvg"></label>
      </div>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue'

export default {
  name: 'DisplaySettings',
  setup() {
    const inlineDisplay = ref(true)
    const showRestColor = ref(true)
    const showSvg = ref(true)

    const decreaseSize = () => {
      document.dispatchEvent(new CustomEvent('changeHandpanSize', {
        detail: { action: 'decrease' }
      }))
    }

    const increaseSize = () => {
      document.dispatchEvent(new CustomEvent('changeHandpanSize', {
        detail: { action: 'increase' }
      }))
    }

    const updateDisplay = () => {
      document.dispatchEvent(new CustomEvent('displaySettingsChange', {
        detail: {
          inlineDisplay: inlineDisplay.value,
          showRestColor: showRestColor.value,
          showSvg: showSvg.value
        }
      }))
    }

    return {
      inlineDisplay,
      showRestColor,
      showSvg,
      decreaseSize,
      increaseSize,
      updateDisplay
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

.zoom {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 15px;
}

.zoom button {
  width: 30px;
  height: 30px;
  border: none;
  border-radius: 4px;
  background: #4CAF50;
  color: white;
  cursor: pointer;
  font-size: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background-color 0.2s;
}

.zoom button:hover {
  background: #45a049;
}

.toggle-container {
  display: grid;
  gap: 12px;
}

.toggle-switch {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

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
