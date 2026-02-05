// useFileUpload.js
import { ref } from 'vue'
import axios from 'axios'
import { apiUrl } from '@/services/api'

export function useFileUpload() {
  const isUploading = ref(false)
  const uploadError = ref(null)
  const uploadProgress = ref(0)

  const uploadFile = async (file) => {
    isUploading.value = true
    uploadError.value = null
    uploadProgress.value = 0

    const formData = new FormData()
    formData.append('file', file)

    try {
      const response = await axios.post(apiUrl('/api/upload'), formData, {
        headers: {
          'Content-Type': 'multipart/form-data'
        },
        onUploadProgress: (progressEvent) => {
          uploadProgress.value = Math.round(
            (progressEvent.loaded * 100) / progressEvent.total
          )
        }
      })
      return response.data
    } catch (error) {
      uploadError.value = error.response?.data?.message || 'Upload failed'
      throw error
    } finally {
      isUploading.value = false
    }
  }

  return {
    isUploading,
    uploadError,
    uploadProgress,
    uploadFile
  }
}
