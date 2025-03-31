<!-- FeaturesApp.vue -->
<template>
  <div class="upload-features">
    <div class="feature-item">
      <PhCloudArrowUp :size="24" />
      <div class="feature-text">
        <h3>Easy Import</h3>
        <p>Drag & drop your MIDI or MuseScore files directly from your computer</p>
      </div>
    </div>
    <div class="feature-item">
      <PhMagicWand :size="24" />
      <div class="feature-text">
        <h3>Smart Detection</h3>
        <p>Automatic detection of notes, key signature, difficulty and metadata</p>
      </div>
    </div>
    <div class="feature-item">
      <PhDownload :size="24" />
      <div class="feature-text">
        <h3>Quick Export</h3>
        <p>Export your tablatures in MIDI and Handflow formats (json)</p>
      </div>
    </div>
    <div class="feature-item">
      <PhPencil :size="24" />
      <div class="feature-text">
        <h3>Editing</h3>
        <p>Customize your tablatures to match your needs</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { PhMagicWand, PhPencil, PhCloudArrowUp, PhDownload } from '@phosphor-icons/vue'
import '@/assets/styles/components/FeaturesApp/features.css'
</script>
