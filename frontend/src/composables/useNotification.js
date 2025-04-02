import { ref, readonly } from 'vue'

// Singleton pattern - shared state across all imports
const notifications = ref([])
const notificationId = ref(0)

export default function useNotification() {
  /**
   * Show a notification
   * @param {Object} options - Notification options
   * @param {string} options.message - Message to display
   * @param {string} options.type - Type of notification (success, warning, error)
   * @param {number} options.duration - Duration in ms (0 for no auto-close)
   * @returns {number} Notification ID that can be used to close it manually
   */
  const notify = (options) => {
    const id = notificationId.value++
    const notification = {
      id,
      message: options.message,
      type: options.type || 'success',
      duration: options.duration !== undefined ? options.duration : 5000,
      timestamp: Date.now()
    }
    
    notifications.value.push(notification)
    
    // Auto-close after duration if duration > 0
    if (notification.duration > 0) {
      setTimeout(() => {
        closeNotification(id)
      }, notification.duration)
    }
    
    return id
  }
  
  /**
   * Show a success notification
   * @param {string} message - Message to display
   * @param {number} duration - Duration in ms (default: 5000, 0 for no auto-close)
   * @returns {number} Notification ID
   */
  const success = (message, duration = 5000) => {
    return notify({ message, type: 'success', duration })
  }
  
  /**
   * Show a warning notification
   * @param {string} message - Message to display
   * @param {number} duration - Duration in ms (default: 5000, 0 for no auto-close)
   * @returns {number} Notification ID
   */
  const warning = (message, duration = 5000) => {
    return notify({ message, type: 'warning', duration })
  }
  
  /**
   * Show an error notification
   * @param {string} message - Message to display
   * @param {number} duration - Duration in ms (default: 5000, 0 for no auto-close)
   * @returns {number} Notification ID
   */
  const error = (message, duration = 5000) => {
    return notify({ message, type: 'error', duration })
  }
  
  /**
   * Close a notification by ID
   * @param {number} id - Notification ID to close
   */
  const closeNotification = (id) => {
    const index = notifications.value.findIndex(n => n.id === id)
    if (index !== -1) {
      notifications.value.splice(index, 1)
    }
  }
  
  /**
   * Close all notifications
   */
  const closeAll = () => {
    notifications.value = []
  }
  
  return {
    // State (readonly to prevent direct mutation)
    notifications: readonly(notifications),
    
    // Methods
    notify,
    success,
    warning,
    error,
    closeNotification,
    closeAll
  }
}
