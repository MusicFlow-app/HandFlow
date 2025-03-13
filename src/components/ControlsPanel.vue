`<template>
  <div id="controls" class="information-container">
    <h3>Controls</h3>
    <div class="controls">
      <display-settings />
      <playback-controls />
      <metronome-controls />
    </div>
  </div>
</template>

<script>
import DisplaySettings from './controls/DisplaySettings.vue'
import PlaybackControls from './controls/PlaybackControls.vue'
import MetronomeControls from './controls/MetronomeControls.vue'

export default {
  name: 'ControlsPanel',
  components: {
    DisplaySettings,
    PlaybackControls,
    MetronomeControls
  }
}
</script>

<style scoped>
.controls {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.information-container {
  background: #fff;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

h3 {
  margin-top: 0;
  color: #333;
  border-bottom: 1px solid #eee;
  padding-bottom: 10px;
}
</style>`
