<script setup>
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue'
import { useTabsLibrary } from '@/composables/useTabsLibrary'
import { useFavorites } from '@/composables/useFavorites'
import { PhInfinity, PhMusicNotes, PhBarbell, PhShootingStar, PhStar, PhStarHalf, PhArrowRight, PhCaretLeft, PhCaretRight, PhSortAscending, PhSortDescending, PhCaretCircleUpDown, PhArrowsDownUp, PhMagnifyingGlass, PhHeart } from '@phosphor-icons/vue'
import '@/assets/styles/components/tabs-library.css'
import '@/assets/styles/components/sorting.css'

const { 
  allFiles,
  loading, 
  error, 
  fetchRecentFiles,
  currentPage,
  totalPages,
  totalItems,
  perPage,
  filterByCategory,
  filterByDifficulty,
  selectedCategory: activeCategory,
  selectedDifficulty: activeDifficulty,
  sortBy,
  sortOrder,
  toggleSort,
  searchQuery,
  setSearchQuery,
  filteredFiles
} = useTabsLibrary()

const showOnlyFavorites = ref(false)
const { favoriteStates, isFavorite, toggleFavorite, favoriteFiles } = useFavorites()

// Get all favorite files first
const allFavoriteFiles = computed(() => {
  return allFiles.value.filter(file => isFavorite(file.id))
})

// Apply filters to favorite files
const filteredFavoriteFiles = computed(() => {
  let filtered = [...allFavoriteFiles.value]

  // Apply search filter
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(file => 
      file.name.toLowerCase().includes(query) ||
      file.category?.toLowerCase().includes(query) ||
      file.difficulty?.toLowerCase().includes(query)
    )
  }

  // Apply category filter
  if (activeCategory.value !== 'all') {
    filtered = filtered.filter(file => file.category === activeCategory.value)
  }

  // Apply difficulty filter
  if (activeDifficulty.value !== 'all') {
    filtered = filtered.filter(file => file.difficulty === activeDifficulty.value)
  }

  return filtered
})

// Replace displayedFiles with filtered and paginated files
const displayedFiles = computed(() => {
  const files = showOnlyFavorites.value ? filteredFavoriteFiles.value : filteredFiles.value
  const start = (currentPage.value - 1) * perPage.value
  const end = start + perPage.value
  
  // Update total pages
  totalItems.value = files.length
  const newTotalPages = Math.ceil(files.length / perPage.value) || 1
  totalPages.value = newTotalPages
  
  // Debug logging
  console.log('Pagination Debug:', {
    filesLength: files.length,
    perPage: perPage.value,
    totalPages: totalPages.value,
    currentPage: currentPage.value,
    start,
    end
  })
  
  // Ensure current page is valid
  if (currentPage.value > totalPages.value) {
    currentPage.value = 1
  }
  
  return files.slice(start, end)
})

// Watch for changes that affect pagination
watch([displayedFiles, currentPage], () => {
  console.log('Pagination state updated:', {
    totalPages: totalPages.value,
    currentPage: currentPage.value,
    totalItems: totalItems.value
  })
}, { immediate: true })



// Handle favorite toggle
const handleFavoriteToggle = async (fileId, event) => {
  event.preventDefault()
  const success = await toggleFavorite(fileId)
  if (success) {
    // Force an update of the display
    await nextTick()
  }
}

// Update files list when needed
const updateFilesList = async () => {
  await nextTick()
  // Only fetch if we're not in favorites mode
  if (!showOnlyFavorites.value) {
    await fetchRecentFiles(currentPage.value)
  }
}

const categories = [
  { id: 'all', label: 'All', icon: PhInfinity },
  { id: 'scale', label: 'Scales', icon: PhCaretCircleUpDown },
  { id: 'song', label: 'Songs', icon: PhMusicNotes },
  { id: 'exercise', label: 'Exercises', icon: PhBarbell }
]

const difficultyLevels = [
  { id: 'all', label: 'All', icon: PhInfinity },
  { id: 'beginner', label: 'Beginner', icon: PhStarHalf },
  { id: 'intermediate', label: 'Intermediate', icon: PhStar },
  { id: 'advanced', label: 'Advanced', icon: PhShootingStar }
]
// Watch for changes in display mode and favorites
watch([showOnlyFavorites, favoriteFiles], async () => {
  // Reset to first page when switching modes
  currentPage.value = 1
  await nextTick()
})

// Debug logs
watch([currentPage, totalPages, displayedFiles], ([newPage, newTotal, newFiles]) => {
  console.log('Page:', newPage, 'Total:', newTotal, 'Files:', newFiles?.length)
})

  // Computed property for pagination display
  const paginationRange = computed(() => {
    const range = []
    const totalPagesValue = totalPages.value
    const currentPageValue = currentPage.value

    // No pagination needed
    if (totalPagesValue <= 1) return range

    // Show all pages if total pages is small
    if (totalPagesValue <= 7) {
      for (let i = 1; i <= totalPagesValue; i++) {
        range.push(i)
      }
      return range
    }

    // Always show first page
    range.push(1)

    // Calculate the range around current page
    const leftBound = Math.max(2, currentPageValue - 1)
    const rightBound = Math.min(totalPagesValue - 1, currentPageValue + 1)

    // Add ellipsis after first page if needed
    if (leftBound > 2) {
      range.push('...')
    }

    // Add pages around current page
    for (let i = leftBound; i <= rightBound; i++) {
      range.push(i)
    }

    // Add ellipsis before last page if needed
    if (rightBound < totalPagesValue - 1) {
      range.push('...')
    }

    // Always show last page
    if (totalPagesValue > 1) {
      range.push(totalPagesValue)
    }

    return range
  })
</script>

<template>
  <div class="tabs-library">
    <div class="library-header">
      <div class="header-top">
        <h2>Tablatures Library</h2>
        <div class="header-controls">
          <div class="search-field">
            <PhMagnifyingGlass :size="20" weight="bold" />
            <input 
              type="text" 
              v-model="searchQuery"
              placeholder="Search tablatures..."
              @input="setSearchQuery($event.target.value)"
            />
          </div>
          <button 
            class="fav-filter-btn" 
            :class="{ active: showOnlyFavorites }"
            @click="() => {
              showOnlyFavorites = !showOnlyFavorites
            }"
          >
            <PhHeart :size="20" :weight="showOnlyFavorites ? 'fill' : 'regular'" />
            <span>Favorites</span>
          </button>
          <div class="sort-buttons">
          <button 
            class="sort-btn" 
            :class="{ active: sortBy === 'title' }"
            :data-order="sortBy === 'title' ? sortOrder : ''"
            @click="toggleSort('title')"
          >
            <span>Title</span>
            <component
              :is="sortBy === 'title' ? (sortOrder === 'asc' ? PhSortAscending : PhSortDescending) : PhArrowsDownUp"
              :size="16"
              weight="bold"
            />
          </button>
          <button 
            class="sort-btn" 
            :class="{ active: sortBy === 'created_at' }"
            :data-order="sortBy === 'created_at' ? sortOrder : ''"
            @click="toggleSort('created_at')"
          >
            <span>Date</span>
            <component
              :is="sortBy === 'created_at' ? (sortOrder === 'asc' ? PhSortAscending : PhSortDescending) : PhArrowsDownUp"
              :size="16"
              weight="bold"
            />
          </button>
          <button 
            class="sort-btn" 
            :class="{ active: sortBy === 'favorite' }"
            :data-order="sortBy === 'favorite' ? sortOrder : ''"
            @click="toggleSort('favorite')"
          >
            <span>Popular</span>
            <component
              :is="sortBy === 'favorite' ? (sortOrder === 'asc' ? PhSortAscending : PhSortDescending) : PhArrowsDownUp"
              :size="16"
              weight="bold"
            />
          </button>
        </div>
        </div>
      </div>
      <div class="filters">
        <div class="category-filter">
          <div class="filter-label">Category:</div>
          <div class="category-slider">
            <div class="slider-track"></div>
            <button 
              v-for="category in categories" 
              :key="category.id"
              :class="['category-level', { active: activeCategory === category.id }]"
              :data-category="category.id"
              @click="filterByCategory(category.id)"
            >
              <component :is="category.icon" :size="20" />
              <span>{{ category.label }}</span>
            </button>
          </div>
        </div>

        <div class="difficulty-filter">
          <div class="filter-label">Difficulty:</div>
          <div class="difficulty-slider">
            <div class="slider-track"></div>
            <button 
              v-for="level in difficultyLevels" 
              :key="level.id"
              :class="['difficulty-level', { active: activeDifficulty === level.id }]"
              :data-difficulty="level.id"
              @click="filterByDifficulty(level.id)"
            >
              <component :is="level.icon" :size="20" />
              <span>{{ level.label }}</span>
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="recent-files-list">
      <div v-if="loading" class="loading-state">
        <div class="loading-spinner"></div>
        <p>Loading recent tablatures...</p>
      </div>
      
      <div v-else-if="error" class="error-state">
        <p>{{ error }}</p>
        <button class="retry-button" @click="fetchRecentFiles">
          Retry
        </button>
      </div>

      <template v-else>
        <div v-if="displayedFiles.length === 0" class="no-files">
          No tablatures match your filters.
        </div>
        <template v-else>
          <div v-for="file in displayedFiles" :key="file.id + '-' + (favoriteStates[file.id] ? 'fav' : 'nofav')" 
               class="tablature-card" 
               :class="{'is-favorite': favoriteStates[file.id]}"
               :data-category="file.category" 
               :data-difficulty="file.difficulty"
               :data-file-id="file.id"
               @click="$emit('reuse', file)">
            <div class="card-content">
              <div class="content-main">
                <div class="card-header">
                  <h4 class="piece-title">{{ file.metadata.title }}</h4>
                  <span class="composer">{{ file.metadata?.composer || 'Unknown' }}</span>
                  <button 
                    class="favorite-btn" 
                    :class="{ 
                      'active': isFavorite(file.id) !== undefined ? isFavorite(file.id) : file.is_favorite,
                    }"
                    @click.stop="handleFavoriteToggle(file.id, $event)"
                    :title="(isFavorite(file.id) !== undefined ? isFavorite(file.id) : file.is_favorite) ? 'Remove from favorites' : 'Add to favorites'"
                  >
                    <PhStar :size="20" :weight="(isFavorite(file.id) !== undefined ? isFavorite(file.id) : file.is_favorite) ? 'fill' : 'regular'" />
                  </button>
                </div>
                <div class="card-tags">
                  <div class="badge" :data-difficulty="file.metadata.difficulty?.toLowerCase()">
                    <component 
                      :is="categories.find(c => c.id === file.metadata.category?.toLowerCase())?.icon" 
                      :size="20" 
                      weight="fill"
                    />
                    <span class="badge-text">{{ file.metadata.difficulty }}</span>
                  </div>
                </div>
              </div>
              <div class="card-actions">

                <div class="hover-arrow">
                  <PhArrowRight :size="24" weight="bold" />
                </div>
              </div>
            </div>
          </div>
        </template>

      </template>
    </div>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="pagination">
      <button 
        class="page-btn prev" 
        :disabled="currentPage === 1"
        @click="() => currentPage = Math.max(1, Math.min(currentPage - 1, totalPages))"
        :title="'Page précédente'"
      >
        <PhCaretLeft 
          :size="20" 
          weight="bold"
        />
      </button>

      <div class="page-numbers">
        <template v-for="page in paginationRange" :key="page">
          <span v-if="page === '...'" class="ellipsis">...</span>
          <button 
            v-else
            :class="['page-number', { active: page === currentPage }]"
            @click="currentPage = page"
          >
            {{ page }}
          </button>
        </template>
      </div>

      <button 
        class="page-btn next" 
        :disabled="currentPage === totalPages"
        @click="() => currentPage = Math.max(1, Math.min(currentPage + 1, totalPages))"
        :title="'Page suivante'"
      >
        <PhCaretRight 
          :size="20" 
          weight="bold"
        />
      </button>
    </div>
  </div>
</template>
