import { ref, onMounted, computed, watch } from 'vue'
import { apiUrl } from '@/services/api'

export function useTabsLibrary() {
  const allFiles = ref([])
  const loading = ref(false)
  const error = ref(null)
  const currentPage = ref(1)
  const totalPages = ref(0)
  const totalItems = ref(0)
  const perPage = ref(9)
  const selectedCategory = ref('all')
  const selectedDifficulty = ref('all')
  const sortBy = ref('created_at')
  const sortOrder = ref('desc')
  const _searchQuery = ref('')  // private ref

  // Expose searchQuery as computed with getter/setter
  const searchQuery = computed({
    get: () => _searchQuery.value,
    set: (value) => {
      _searchQuery.value = value
      currentPage.value = 1 // Reset to first page when searching
    }
  })

  // Computed property for filtered and sorted data
  const filteredFiles = computed(() => {
    let filtered = [...allFiles.value]

    // Apply search filter
    if (_searchQuery.value) {
      const query = _searchQuery.value.toLowerCase()
      filtered = filtered.filter(file => 
        (file.name || '').toLowerCase().includes(query) ||
        (file.composer || '').toLowerCase().includes(query) ||
        (file.arranger || '').toLowerCase().includes(query) ||
        file.category.toString().includes(query) || // Convert to string only for search
        file.difficulty.toString().includes(query)  // Convert to string only for search
      )
    }

    // Apply category filter
    if (selectedCategory.value !== 'all') {
      filtered = filtered.filter(file => file.category === parseInt(selectedCategory.value))
    }

    // Apply difficulty filter
    if (selectedDifficulty.value !== 'all') {
      filtered = filtered.filter(file => file.difficulty === parseInt(selectedDifficulty.value))
    }

    // Apply sorting
    filtered.sort((a, b) => {
      if (sortBy.value === 'created_at') {
        const aDate = new Date(a.uploadTime)
        const bDate = new Date(b.uploadTime)
        return sortOrder.value === 'asc' ? aDate - bDate : bDate - aDate
      } else if (sortBy.value === 'favorite') {
        const aFav = a.favoriteCount || 0
        const bFav = b.favoriteCount || 0
        return sortOrder.value === 'asc' ? aFav - bFav : bFav - aFav
      } else if (sortBy.value === 'title') {
        const aName = a.name || a.filename
        const bName = b.name || b.filename
        return sortOrder.value === 'asc' 
          ? aName.localeCompare(bName)
          : bName.localeCompare(aName)
      }
      return 0
    })

    return filtered
  })

  // Update pagination whenever filtered files change
  watch(filteredFiles, (files) => {
    const total = files.length
    totalItems.value = total
    totalPages.value = Math.max(1, Math.ceil(total / perPage.value))

    // Ensure current page is valid
    if (currentPage.value > totalPages.value) {
      currentPage.value = 1
    }
  }, { immediate: true })

  // Computed property for paginated files
  const paginatedFiles = computed(() => {
    const start = (currentPage.value - 1) * perPage.value
    const end = start + perPage.value
    return filteredFiles.value.slice(start, end)
  })

  // Initialize data
  onMounted(async () => {
    await fetchRecentFiles()
  })

  const fetchRecentFiles = async (page = 1) => {
    loading.value = true
    error.value = null
    currentPage.value = page
    
    try {
      const response = await fetch(apiUrl(`/api/library?page=1&per_page=1000&sort_by=${sortBy.value}&sort_order=${sortOrder.value}`), {
        method: 'GET',
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
        },
        credentials: 'include',
        mode: 'cors'
      })
      if (!response.ok) {
        const errorText = await response.text()
        console.error('Server response:', response.status, errorText)
        throw new Error(`Failed to fetch recent files: ${response.status} ${errorText || response.statusText}`)
      }
      
      const data = await response.json()
      allFiles.value = data.tabs.map(file => ({
        id: file.id,
        name: file.metadata.work_title || 'Unknown',
        composer: file.metadata.composer || 'Unknown',
        arranger: file.metadata.arranger || 'Unknown',
        category: file.metadata.category || 2, // Keep as number
        difficulty: file.metadata.difficulty || 1, // Keep as number
        uploadTime: file.created_at,
        favoriteCount: file.favorite_count || 0,
        metadata: file.metadata
      }))

      // Update pagination based on filtered files
      const filteredCount = filteredFiles.value.length
      totalItems.value = filteredCount
      totalPages.value = Math.max(1, Math.ceil(filteredCount / perPage.value))
      
      // Log initial state
      loading.value = false
    } catch (err) {
      console.error('Error fetching recent files:', err)
      error.value = err.message
    } finally {
      loading.value = false
    }
  }

  const reuseFile = async (fileId) => {
    try {
      const file = filteredFiles.value.find(f => f.id === fileId)
      if (!file) return
      
      // You can implement additional reuse logic here
      return file
    } catch (err) {
      console.error('Error reusing file:', err)
      throw err
    }
  }

  const setSearchQuery = (query) => {
    searchQuery.value = query // This will trigger the computed setter
  }

  // Function to update files list
  const updateFilesList = async () => {
    await fetchRecentFiles(currentPage.value)
  }

  // Toggle sort order
  const toggleSort = async (field) => {
    if (sortBy.value === field) {
      sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
    } else {
      sortBy.value = field
      sortOrder.value = 'asc'
    }
    currentPage.value = 1
    // Re-fetch data with new sort parameters
    await fetchRecentFiles()
  }

  return {
    allFiles,
    loading,
    error,
    currentPage,
    totalPages,
    totalItems,
    perPage,
    selectedCategory,
    selectedDifficulty,
    sortBy,
    sortOrder,
    searchQuery,
    updateFilesList,
    toggleSort,
    setSearchQuery,
    reuseFile,
    filterByCategory: (category) => {
      selectedCategory.value = category
      currentPage.value = 1
    },
    filterByDifficulty: (difficulty) => {
      selectedDifficulty.value = difficulty
      currentPage.value = 1
    },
    fetchRecentFiles,
    filteredFiles
  }
}
