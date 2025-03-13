`<template>
  <div class="app">
    <piece-info 
      :workTitle="workTitle"
      :composer="composer"
      :arranger="arranger"
    />
    <div class="informations">
      <part-selector 
        :mscxPath="mscxPath"
        :partOptions="partOptions"
        :scaleOptions="scaleOptions"
        @generate="handleGenerate"
      />
      <controls-panel />
    </div>
    <div class="generate-container" id="generate-container"></div>
  </div>
</template>

<script>
import { ref } from 'vue'
import PieceInfo from './components/PieceInfo.vue'
import PartSelector from './components/PartSelector.vue'
import ControlsPanel from './components/ControlsPanel.vue'

export default {
  name: 'App',
  components: {
    PieceInfo,
    PartSelector,
    ControlsPanel
  },
  setup() {
    const workTitle = ref('')
    const composer = ref('')
    const arranger = ref('')
    const mscxPath = ref('')
    const partOptions = ref([])
    const scaleOptions = ref([])

    const handleGenerate = async (formData) => {
      try {
        const response = await fetch('/generate', {
          method: 'POST',
          body: formData
        })
        if (response.ok) {
          const container = document.getElementById('generate-container')
          container.innerHTML = await response.text()
        }
      } catch (error) {
        console.error('Generation failed:', error)
      }
    }

    return {
      workTitle,
      composer,
      arranger,
      mscxPath,
      partOptions,
      scaleOptions,
      handleGenerate
    }
  }
}
</script>

<style>
.app {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.informations {
  display: grid;
  grid-template-columns: 1fr;
  gap: 20px;
  margin-top: 20px;
}

@media (min-width: 768px) {
  .informations {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>`
