<script setup>
import { ref } from 'vue'
import { PhUploadSimple, PhFileArrowUp, PhFileArrowDown, PhMagicWand, PhGear, PhX } from '@phosphor-icons/vue'
import { useFileUpload } from '@/composables/useFileUpload'

const props = defineProps({
  isOpen: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits(['close'])

const isDragging = ref(false)
const fileInput = ref(null)
const selectedFile = ref(null)

const { uploadFile } = useFileUpload()

const closeModal = () => {
  emit('close')
}

const triggerFileInput = () => {
  fileInput.value.click()
}

const handleFileSelect = async (event) => {
  const file = event.target.files[0]
  if (file) {
    selectedFile.value = file
    await uploadFile(file)
  }
}

const handleDrop = async (event) => {
  isDragging.value = false
  const file = event.dataTransfer.files[0]
  if (file && (file.name.endsWith('.mscz') || file.name.endsWith('.mid'))) {
    selectedFile.value = file
    await uploadFile(file)
  }
}
</script>

<template>
  <div v-if="isOpen" class="upload-modal-overlay" @click.self="closeModal">
    <div class="upload-modal">
      <div class="modal-header">
        <h2>Import MuseScore Files</h2>
        <button class="close-button" @click="closeModal">
          <PhX :size="24" weight="bold" />
        </button>
      </div>
      <div class="upload-section">
        <div class="upload-intro">
          <p class="upload-description">Transform your MuseScore compositions into beautiful handpan tablatures. Simply drag & drop your file or click below to start.</p>
        </div>
    
        <div 
          class="drop-zone" 
          @dragover.prevent="isDragging = true"
          @dragleave.prevent="isDragging = false"
          @drop.prevent="handleDrop"
          @click="triggerFileInput"
          :class="{ 'drag-over': isDragging }"
        >
          <div class="drop-zone-content">
            <PhUploadSimple :size="48" />
            <p class="drop-zone-title">Drop your MuseScore file here</p>
            <p class="drop-zone-subtitle">or click to browse your files</p>
            <p class="drop-zone-format">Accepts .mscz & .mid files</p>
          </div>
          <input 
            type="file" 
            ref="fileInput"
            accept=".mscz, .mid" 
            style="display: none;"
            @change="handleFileSelect"
          >
          <p v-if="selectedFile" class="file-name">{{ selectedFile.name }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@import '@/assets/styles/components/UploadZone/upload.css';

</style>
