import { ref, computed, watch } from 'vue'
import Cookies from 'js-cookie'

export function useFavorites() {
  // Initialize favorites from cookies
  const FAVORITES_COOKIE_KEY = 'handflow_favorites'
  const favoriteIds = ref(new Set(
    JSON.parse(Cookies.get(FAVORITES_COOKIE_KEY) || '[]')
  ))

  // Reactive state for immediate UI updates
  const favoriteStates = ref({})

  // Computed property for favorite files
  const favoriteFiles = computed(() => {
    return Array.from(favoriteIds.value)
  })

  // Check if a file is favorited
  const isFavorite = (fileId) => {
    return favoriteIds.value.has(fileId)
  }

  // Save favorites to cookies
  const saveFavoritesToCookies = () => {
    Cookies.set(
      FAVORITES_COOKIE_KEY, 
      JSON.stringify(Array.from(favoriteIds.value)), 
      { expires: 365 }
    )
  }

  // Toggle favorite status
  const toggleFavorite = async (fileId) => {
    try {
      // Optimistically update UI state
      const isCurrentlyFavorited = isFavorite(fileId)
      const newState = !isCurrentlyFavorited

      // Update local state immediately
      favoriteStates.value[fileId] = newState
      
      // Make API request
      const response = await fetch(`/api/tabs/${fileId}/favorite`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          increment: newState // true when adding, false when removing
        })
      })

      if (!response.ok) {
        throw new Error('Failed to update favorite status')
      }

      // Update Set and cookies only after successful API call
      if (newState) {
        favoriteIds.value.add(fileId)
      } else {
        favoriteIds.value.delete(fileId)
      }

      saveFavoritesToCookies()
      return true

    } catch (error) {
      console.error('Error toggling favorite:', error)
      // Revert UI state on error
      favoriteStates.value[fileId] = isFavorite(fileId)
      return false
    }
  }

  // Initialize favoriteStates from favoriteIds
  watch(favoriteIds, (ids) => {
    const states = {}
    ids.forEach(id => {
      states[id] = true
    })
    favoriteStates.value = states
  }, { immediate: true })

  return {
    favoriteIds,
    favoriteStates,
    favoriteFiles,
    isFavorite,
    toggleFavorite
  }
}