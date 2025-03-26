import { ref, onMounted, computed, watch } from 'vue'

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
  const searchQuery = ref('')  
  // Computed property for filtered and paginated data
  const filteredFiles = computed(() => {
    let filtered = [...allFiles.value]

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
    if (selectedCategory.value !== 'all') {
      filtered = filtered.filter(file => file.category === selectedCategory.value)
    }

    // Apply difficulty filter
    if (selectedDifficulty.value !== 'all') {
      filtered = filtered.filter(file => file.difficulty === selectedDifficulty.value)
    }

    // Update pagination state
    const total = filtered.length
    const maxPages = Math.max(1, Math.ceil(total / perPage.value))
    
    // Update refs
    totalItems.value = total
    totalPages.value = maxPages
    
    // Ensure current page is valid
    if (currentPage.value > maxPages) {
      currentPage.value = 1
    }

    // Return paginated slice
    const start = (currentPage.value - 1) * perPage.value
    const end = start + perPage.value
    return filtered.slice(start, end)
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
      const response = await fetch(`/api/tabs/recent?page=1&per_page=1000&sort_by=${sortBy.value}&sort_order=${sortOrder.value}`, {
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
        name: file.filename,
        category: file.metadata.category?.toLowerCase(),
        difficulty: file.metadata.difficulty?.toLowerCase(),
        uploadTime: new Date(file.created_at).toLocaleString(),
        metadata: file.metadata
      }))

      // Update pagination based on all files first
      totalItems.value = allFiles.value.length
      totalPages.value = Math.ceil(totalItems.value / perPage.value) || 1

      // Then update based on filtered results if filters are active
      if (selectedCategory.value !== 'all' || selectedDifficulty.value !== 'all' || searchQuery.value) {
        const filteredCount = filteredFiles.value.length
        totalItems.value = filteredCount
        totalPages.value = Math.ceil(filteredCount / perPage.value) || 1
      }

      // Log initial state
      console.log(`Page: ${currentPage.value} Total: ${totalPages.value} Files: ${allFiles.value.length} Filtered: ${filteredFiles.value.length}`)
    } catch (err) {
      error.value = err.message
      console.error('Error fetching recent files:', err)
    } finally {
      loading.value = false
    }
  }

  const reuseFile = async (fileId) => {
    try {
      const file = files.value.find(f => f.id === fileId)
      if (!file) return
      
      // You can implement additional reuse logic here
      return file
    } catch (err) {
      console.error('Error reusing file:', err)
      throw err
    }
  }

  const setSearchQuery = (query) => {
    searchQuery.value = query
    currentPage.value = 1 // Reset to first page when searching
  }

  // Computed for filtered and sorted files
  const files = computed(() => {
    let result = filteredFiles.value

    // Apply sorting
    result = [...result].sort((a, b) => {
      const aValue = a[sortBy.value]
      const bValue = b[sortBy.value]

      if (sortOrder.value === 'asc') {
        return aValue > bValue ? 1 : -1
      } else {
        return aValue < bValue ? 1 : -1
      }
    })

    // Apply pagination
    const start = (currentPage.value - 1) * perPage.value
    const end = start + perPage.value
    return result.slice(start, end)
  })

  // Function to update files list
  const updateFilesList = async () => {
    await fetchRecentFiles(currentPage.value)
  }

  // Toggle sort order
  const toggleSort = (field) => {
    if (sortBy.value === field) {
      sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
    } else {
      sortBy.value = field
      sortOrder.value = 'asc'
    }
    currentPage.value = 1
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
