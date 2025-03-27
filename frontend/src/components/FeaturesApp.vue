<!-- FeaturesApp.vue -->
<template>
  <div class="upload-features">
    <div class="feature-item">
      <PhFileArrowUp :size="24" />
      <div class="feature-text">
        <h3>Easy Import</h3>
        <p>Drag & drop your MuseScore files directly from your computer</p>
      </div>
    </div>
    <div class="feature-item">
      <PhMagicWand :size="24" />
      <div class="feature-text">
        <h3>Smart Detection</h3>
        <p>Automatic detection of notes, scales and musical patterns</p>
      </div>
    </div>
    <div class="feature-item">
      <PhFileArrowDown :size="24" />
      <div class="feature-text">
        <h3>Quick Export</h3>
        <p>Export your tablatures in various formats</p>
      </div>
    </div>
    <div class="feature-item">
      <PhGear :size="24" />
      <div class="feature-text">
        <h3>Customization</h3>
        <p>Customize your tablatures to match your needs</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { PhFileArrowUp, PhMagicWand, PhFileArrowDown, PhGear } from '@phosphor-icons/vue'
import '@/assets/styles/components/FeaturesApp/features.css'
</script>
