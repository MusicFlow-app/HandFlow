<script setup>
import UploadZone from '../components/UploadZone.vue'
import TabsLibrary from '../components/TabsLibrary.vue'
import FeaturesApp from '../components/FeaturesApp.vue'
import  HandpanScaleSelection from '../components/HandpanScaleSelection.vue'

import '@/assets/styles/views/home.css'
</script>

<template>
  <div class="home">
    <section class="handpan-scale-section">
      <HandpanScaleSelection />
    </section>
    <TabsLibrary />
    <UploadZone />
    <FeaturesApp />
  </div>
</template>


