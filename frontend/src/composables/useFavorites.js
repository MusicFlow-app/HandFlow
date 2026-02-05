import { ref, computed, watch } from 'vue'
import { useCookies } from '@vueuse/integrations/useCookies'
import useNotification from './useNotification'
import { apiUrl } from '@/services/api'

export function useFavorites() {
  // Initialize notification system
  const notification = useNotification()
  
  // Initialize favorites from cookies
  const FAVORITES_COOKIE_KEY = 'handflow_tabs_favorites'
  const cookies = useCookies([FAVORITES_COOKIE_KEY])
  
  // Safely parse cookie value with error handling
  let initialFavorites = [];
  try {
    const cookieValue = cookies.get(FAVORITES_COOKIE_KEY);
    //console.log('DEBUG - Raw cookie value:', cookieValue);
    
    if (cookieValue) {
      // Handle both string and object formats
      if (typeof cookieValue === 'string') {
        initialFavorites = JSON.parse(cookieValue);
      } else if (Array.isArray(cookieValue)) {
        initialFavorites = cookieValue;
      } else {
        //console.warn('DEBUG - Unexpected cookie format:', typeof cookieValue);
      }
    }
  } catch (error) {
    //console.error('DEBUG - Error parsing favorites cookie:', error);
    // Reset cookie if corrupted
    cookies.set(FAVORITES_COOKIE_KEY, '[]', { maxAge: 60 * 60 * 24 * 365, path: '/' });
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
    cookies.set(
      FAVORITES_COOKIE_KEY, 
      JSON.stringify(favoritesArray), 
      { maxAge: 60 * 60 * 24 * 365, path: '/' }
    )
    //console.log('DEBUG - Saved tabs favorites to cookies:', favoritesArray)
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