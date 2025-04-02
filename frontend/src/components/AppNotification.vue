<template>
  <!-- Notification Container -->
  <div class="notification-container">
    <!-- Individual Notifications -->
    <transition-group name="notification-fade">
      <div 
        v-for="notification in notifications" 
        :key="notification.id"
        class="notification-message" 
        :class="notification.type"
      >
        <div class="notification-icon">
          <PhCheckCircle v-if="notification.type === 'success'" :size="24" weight="fill" />
          <PhWarning v-else-if="notification.type === 'warning'" :size="24" weight="fill" />
          <PhX v-else-if="notification.type === 'error'" :size="24" weight="fill" />
        </div>
        <div class="notification-content">{{ notification.message }}</div>
      </div>
    </transition-group>
  </div>
</template>

<script setup>
import { onUnmounted } from 'vue'
import { PhCheckCircle, PhWarning, PhX } from '@phosphor-icons/vue'
import useNotification from '@/composables/useNotification'

// Get notification methods and state from the composable
const { notifications, closeNotification, closeAll } = useNotification()

// Clean up all notifications when component is unmounted
onUnmounted(() => {
  closeAll()
})
</script>

<style scoped>
@import '@/assets/styles/components/AppNotification/base.css';
</style>
