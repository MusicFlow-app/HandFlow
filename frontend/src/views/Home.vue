<script setup>
import UploadZone from '../components/UploadZone.vue'
import TabsLibrary from '../components/TabsLibrary.vue'
</script>

<template>
  <div class="home">
    <TabsLibrary />
    <UploadZone />
  </div>
</template>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xl);
  width: 100%;
  max-width: 1200px;
}
</style>
