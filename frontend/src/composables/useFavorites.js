import { ref, computed, watch } from 'vue'
import Cookies from 'js-cookie'
import useNotification from './useNotification'
import { apiUrl } from '@/services/api'

export function useFavorites() {
  // Initialize notification system
  const notification = useNotification()

  // Initialize favorites from cookies
  const FAVORITES_COOKIE_KEY = 'handflow_tabs_favorites'

  // Safely parse cookie value with error handling
  let initialFavorites = [];
  try {
    const cookieValue = Cookies.get(FAVORITES_COOKIE_KEY);

    if (cookieValue) {
      initialFavorites = JSON.parse(cookieValue);
    }
  } catch (error) {
    // Reset cookie if corrupted
    Cookies.set(FAVORITES_COOKIE_KEY, '[]', { expires: 365, path: '/' });
  }
  
  const favoriteIds = ref(new Set(initialFavorites))

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
    const favoritesArray = Array.from(favoriteIds.value)
    Cookies.set(
      FAVORITES_COOKIE_KEY,
      JSON.stringify(favoritesArray),
      { expires: 365, path: '/' }
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
      const response = await fetch(apiUrl(`/api/tabs/${fileId}/favorite`), {
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
        notification.success(`Tab added to favorites`)
      } else {
        favoriteIds.value.delete(fileId)
        notification.warning(`Tab removed from favorites`)
      }

      // Save to cookies after a successful update
      saveFavoritesToCookies()
      return true

    } catch (error) {
      //console.error('Error toggling favorite:', error)
      // Revert UI state on error
      favoriteStates.value[fileId] = isFavorite(fileId)
      notification.error(`Failed to update favorite status: ${error.message}`)
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