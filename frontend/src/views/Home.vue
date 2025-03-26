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

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md);
}

.logo {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.logo h1 {
  font-size: var(--font-size-2xl);
  font-weight: 700;
  color: var(--text-color);
  margin: 0;
  line-height: 1;
}

.tagline {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}



.theme-toggle {
  background: var(--surface-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius-md);
  padding: 0.5rem;
  cursor: pointer;
  color: var(--text-primary);
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: var(--shadow-default);
}

.theme-toggle:hover {
  background: var(--surface-hover);
  border-color: var(--border-hover);
  transform: translateY(-1px);
}

.theme-toggle:active {
  background: var(--surface-active);
  transform: translateY(0);
}
</style>
