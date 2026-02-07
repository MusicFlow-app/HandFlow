<script setup>
import { ref } from 'vue'
import UploadZone from '../components/UploadZone.vue'
import TabsLibrary from '../components/TabsLibrary.vue'
import FeaturesApp from '../components/FeaturesApp.vue'
import HandpanScaleSelection from '../components/HandpanScaleSelection.vue'
import HandpanDisplay from '../components/HandpanDisplay.vue'
import ScoreReader from '../components/ScoreReader/ScoreReader.vue'
import useHandpanSelection, {
  currentStage,
  selectScale,
  selectDing,
  selectNoteCount,
  fetchNotes,
  goToStage,
  selectedScale,
  notes,
  selectedDing,
  selectedNoteCount
} from '../composables/useHandpanSelection'

import '@/assets/styles/views/home.css'

// Initialize handpan selection state
useHandpanSelection()

// State to manage visibility of the upload modal
const showUploadModal = ref(false)

// State for score reader
const selectedScore = ref(null)
const showScoreReader = ref(false)

// Key to force re-render of HandpanScaleSelection
const scaleSelectionKey = ref(0)

// Handle completion of handpan selection
const onHandpanSelectionComplete = async (selectionData) => {
  console.log('Home: Handpan selection complete, transitioning to display with data:', selectionData);
  console.log('Current stage before processing:', currentStage.value);
  
  try {
    // Update scale
    if (selectionData.scaleId) {
      console.log('Selecting scale:', selectionData.scaleId);
      await selectScale({ 
        id: selectionData.scaleId,
        name: selectionData.scaleName,
        category: selectionData.scaleCategory, // Add category information
        isFavorite: selectionData.isFavorite
      });
    }
    
    // Update ding
    if (selectionData.dingNote) {
      console.log('Selecting ding:', selectionData.dingNote);
      await selectDing(selectionData.dingNote);
    }
    
    // Update note count
    if (selectionData.noteCount) {
      console.log('Selecting note count:', selectionData.noteCount);
      await selectNoteCount(selectionData.noteCount);
    }
    
    // Fetch notes
    console.log('Fetching notes for selected scale and ding');
    await fetchNotes();
    
    // First ensure we're at stage 5 or higher (selection complete)
    if (currentStage.value < 5) {
      console.log('Setting stage to 5 (selection complete)');
      await goToStage(5);
      // Small delay to ensure state updates
      await new Promise(resolve => setTimeout(resolve, 100));
    }
    
    console.log('Stage after ensuring stage 5:', currentStage.value);
    
    // Then advance to stage 6 (display handpan)
    console.log('Attempting to advance to stage 6 (display handpan)');
    await goToStage(6);
    
    console.log('Stage after attempting to advance to stage 6:', currentStage.value);
    
    console.log('All state updated, stage is:', currentStage.value);
    
    // No need to manually set visibility flags anymore
    // The visibility is now controlled by the stage value in the template
  } catch (error) {
    console.error('Error transitioning to handpan display:', error);
  }
}

// Handle reset from handpan display
const onHandpanDisplayReset = () => {
  // The resetSelection and goToStage(1) are already called in HandpanDisplay.vue
  // No need to explicitly set visibility flags since they're controlled by the stage value
  
  // Just verify the stage is correctly set to 1
  if (currentStage.value !== 1) {
    console.log('Ensuring stage is reset to 1');
    goToStage(1);
  }
  
  console.log('Home: Reset triggered, stage is now:', currentStage.value);
}

// Handle back-to-scales from handpan display
const onBackToScales = () => {
  console.log('Home: onBackToScales called, current stage before:', currentStage.value);
  
  // Increment the key to force re-render of HandpanScaleSelection
  scaleSelectionKey.value++;
  console.log('Home: Incremented scaleSelectionKey to force re-render:', scaleSelectionKey.value);
  
  // Verify the stage is correctly set to 3 (scale selection)
  if (currentStage.value !== 3) {
    console.log('Home: Ensuring stage is set to 3 (scale selection)');
    goToStage(3);
  }
  
  console.log('Home: Back to scales triggered, stage is now:', currentStage.value);
  console.log('Home: HandpanScaleSelection visibility should be:', currentStage.value < 6 ? 'visible' : 'hidden');
}

const openUploadModal = () => {
  showUploadModal.value = true
}

const closeUploadModal = () => {
  showUploadModal.value = false
}

// Handle generate tablature request
const onGenerateTab = (handpanConfig) => {
  console.log('Generating tablature with config:', handpanConfig)
  // Here you would implement the logic to generate the tablature
  // or navigate to another view
}

// Handle score selection from TabsLibrary
const onScoreSelected = (file) => {
  console.log('Score selected:', file)
  selectedScore.value = file
  showScoreReader.value = true
}

// Close score reader and go back to handpan display
const closeScoreReader = () => {
  showScoreReader.value = false
  selectedScore.value = null
}
</script>

<template>
  <div class="home">
    <!-- Show handpan-scale-section when stage is less than 6 -->
    <section class="handpan-scale-section" v-if="currentStage < 6">
      <HandpanScaleSelection 
        :key="scaleSelectionKey"
        @selection-complete="onHandpanSelectionComplete" 
      />
    </section>
    
    <!-- Show ScoreReader when a score is selected -->
    <ScoreReader
      v-if="currentStage >= 6 && showScoreReader && selectedScore"
      :score-id="selectedScore.id"
      :score-file="selectedScore"
      @close="closeScoreReader"
    />

    <!-- Show HandpanDisplay when stage is 6 and no score reader -->
    <HandpanDisplay
      v-if="currentStage >= 6 && !showScoreReader"
      :scaleCategory="selectedScale?.category"
      :isFavorite="selectedScale?.isFavorite || false"
      :notes="notes"
      :noteCount="selectedNoteCount"
      :dingNote="selectedDing"
      @reset="onHandpanDisplayReset"
      @back-to-scales="onBackToScales"
      @generate-tab="onGenerateTab"
    />

    <TabsLibrary
      v-if="currentStage >= 6 && !showScoreReader"
      @open-import-modal="openUploadModal"
      @reuse="onScoreSelected"
    />
    <UploadZone :is-open="showUploadModal" @close="closeUploadModal" />
    <FeaturesApp v-if="currentStage < 6" />
  </div>
</template>

<style>
.handpan-scale-section {
  position: relative;
  min-height: 600px;
}

.hidden {
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.5s ease;
}
</style>
