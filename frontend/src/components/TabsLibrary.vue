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

// Computed property for favorite-only files
const favoriteOnlyFiles = computed(() => {
  return filteredFiles.value.filter(file => isFavorite(file.id))
})

// Computed property for displayed files
const displayedFiles = computed(() => {
  return showOnlyFavorites.value ? favoriteOnlyFiles.value : filteredFiles.value
})



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
    const showEllipsis = totalPages.value > 7

    // Always show first page
    range.push(1)

    if (showEllipsis) {
      const current = currentPage.value
      const lastPage = totalPages.value

      // Show ellipsis after first page if current page is far enough
      if (current > 4) {
        range.push('...')
      }

      // Calculate the range around current page
      const start = Math.max(2, current - 2)
      const end = Math.min(lastPage - 1, current + 2)

      for (let i = start; i <= end; i++) {
        if (i === 1 || i === lastPage) continue
        range.push(i)
      }

      // Show ellipsis before last page if needed
      if (current < lastPage - 3) {
        range.push('...')
      }
    } else {
      // If total pages is small, show all pages
      for (let i = 2; i < totalPages.value; i++) {
        range.push(i)
      }
    }

    // Always show last page if there is more than one page
    if (totalPages.value > 1) {
      range.push(totalPages.value)
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
        <!-- Page numbers -->
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
