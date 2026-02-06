<template>
  <!-- Always render but show conditionally based on state -->
  <div class="handpan-display" :class="{ 'visible': isReadyToDisplay && currentStage >= 6 && !errorDuringRender }">
    <!-- Notation Modal -->
    <div class="notation-modal" v-if="showNotationModal">
      <div class="modal-content">
        <h3>Handpan Notation</h3>
        <p class="notation-text">{{ formattedNotes }}</p>
        <div class="modal-actions">
          <button class="copy-button" @click="copyNotation">
            <span v-if="!copied"><PhCopy :size="24" /> Copy</span>
            <span v-else><PhCheck :size="24" /> Copied!</span>
          </button>
          <button class="close-button" @click="showNotationModal = false">Close</button>
        </div>
      </div>
    </div>
    
    <div class="handpan-display-content">
      <!-- Header with scale information -->
      <div class="handpan-header">
        <h2>{{ scale?.name || 'Handpan Scale' }}</h2>
        <div class="handpan-meta">
          <span class="category-badge" :style="{ backgroundColor: getColorSafely(props.scaleCategory) }">
            {{ props.scaleCategory || 'Unknown' }}
          </span>
          <span class="ding-badge">Ding: {{ ding || 'None' }}</span>
          <span class="notes-badge">
            <PhMusicNotes :size="28" weight="duotone" />
            <span class="note-count">{{ selectedNotes.length }} notes</span>
            <button class="info-button" @click.stop="showNotationModal = true">
              <PhInfo :size="26" weight="bold" />
            </button>
          </span>
          
          <!-- Favorite button -->
          <button 
            class="favorite-button" 
            :class="{ active: props.isFavorite }"
            @click="toggleFavorite"
          >
            <PhStar :size="24" :weight="props.isFavorite ? 'fill' : 'regular'" />
          </button>
        </div>
      </div>
      
      <!-- Visual representation of the handpan -->
      <div class="handpan-visualization">
        <!-- Top view - always present -->
        <div class="handpan-container top-view">
          <h3 class="view-label">Top View</h3>
          <div class="handpan-instrument" @click="playSlack">
            <div class="handpan-shell handpan-top-shell">
              <!-- Metallic texture overlay -->
              <div class="metal-texture"></div>
              
              <!-- Ding note in the center -->
              <div class="ding-note note" @click.stop="playNote(dingData)" :class="{ 'active': activeNote === 'ding' }">
                <span class="note-name">{{ dingNoteValue }}</span>
              </div>
              
              <!-- Top notes in alternating pattern -->
              <div 
                v-for="(note, index) in topNotes" 
                :key="`top-${index}`"
                class="note tone-field"
                :class="{ 'active': activeNote === index }"
                :style="getNoteStyle(index + 1, topNotes.length, 1, 'top', note)"
                @click.stop="playNote(note, index)"
              >
                <div class="note-content">
                  <span class="note-name">{{ note.note || note.calculated_note || '' }}</span>
                </div>
              </div>
              
              <!-- Inner notes -->
              <div 
                v-for="(note, index) in innerNotes" 
                :key="`inner-${index}`"
                class="note tone-field inner"
                :class="{ 'active': activeNote === index + topNotes.length }"
                :style="getNoteStyle(index + 1, innerNotes.length, 0.7, 'inner', note)"
                @click.stop="playNote(note, index + topNotes.length)"
              >
                <div class="note-content">
                  <span class="note-name">{{ note.note || note.calculated_note || '' }}</span>
                </div>
              </div>
              
              <!-- Light reflections -->
              <div class="light-reflection reflection-1"></div>
              <div class="light-reflection reflection-2"></div>
            </div>
          </div>
        </div>
        
        <!-- Bottom view - only present if bottom notes exist -->
        <div v-if="bottomNotes.length > 0" class="handpan-container bottom-view">
          <h3 class="view-label">Bottom View</h3>
          <div class="handpan-instrument" @click="playSlack">
            <div class="handpan-shell handpan-bottom-shell">
              <!-- Metallic texture overlay -->
              <div class="metal-texture"></div>
              
              <!-- Gu port in the center -->
              <div class="gu-port" @click.stop="playGu" style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);">
                <div class="gu-inner"></div>
              </div>
              
              <!-- Bottom notes -->
              <div 
                v-for="(note, index) in bottomNotes" 
                :key="`bottom-${index}`"
                class="note tone-field bottom"
                :class="{ 'active': activeNote === index + topNotes.length + innerNotes.length }"
                :style="getNoteStyle(index + 1, bottomNotes.length, 1, 'bottom', note)"
                @click.stop="playNote(note, index + topNotes.length + innerNotes.length)"
              >
                <div class="note-content">
                  <span class="note-name">{{ note.note || note.calculated_note || '' }}</span>
                </div>
              </div>
              
              <!-- Light reflections -->
              <div class="light-reflection reflection-1"></div>
              <div class="light-reflection reflection-2"></div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Actions section -->
      <div class="handpan-actions">
        <button class="action-button primary" @click="resetAndGoBack">
          Choose Different Scale
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { PhStar, PhMusicNotes } from '@phosphor-icons/vue'
import useHandpanSelection from '../composables/useHandpanSelection'
import useHandpanDisplay from '../composables/useHandpanDisplay'

// Import notification system for user feedback
import useNotification from '@/composables/useNotification'

// Import Phosphor icons
import { PhInfo, PhCopy, PhCheck } from '@phosphor-icons/vue'

// Constants for audio
const SLACK_PITCH = 127;
const GU_PITCH = 40; // Approximate pitch for the Gu port - can be adjusted

// Props to receive data from parent component
const props = defineProps({
  scaleId: String,
  scaleName: String,
  scaleCategory: String, 
  dingNote: String,
  noteCount: Number,
  notes: {
    type: Array,
    default() { return [] }
  },
  isFavorite: {
    type: Boolean,
    default: false
  }
})

// Emit events to parent
const emit = defineEmits(['generate-tab', 'reset', 'back-to-scales'])

// Get handpan selection state from the shared composable
const {
  // Selected data
  selectedScale,
  selectedDing,
  selectedNoteCount,
  notes,
  currentStage,
  
  // Navigation methods
  goToStage,
  resetSelection,
  
  // Helper methods
  getCategoryColor,
  toggleFavorite
} = useHandpanSelection()

// Only initialize the display logic when needed (stage 5+)
const shouldInitDisplay = computed(() => currentStage.value >= 5)

// Local state for errors
const errorDuringRender = ref(false);

// State for notation modal
const showNotationModal = ref(false);
const copied = ref(false);

// Only initialize the display logic when needed (stage 5+)
const {
  // State
  isReadyToDisplay,
  isPlaying,
  activeNote,
  scale,
  ding,
  selectedNotes,
  topNotes,
  innerNotes,
  bottomNotes,
  dingData,
  
  // Utility functions
  getNoteStyle,
  
  // Audio functions
  playNote,
  playSlack,
  playGu,
  
  // Debug functions
  
  // Cleanup
  cleanup
} = useHandpanDisplay({
  // Use props if available, fallback to shared state
  notes: props.notes && props.notes.length ? props.notes : notes,
  selectedScale,
  selectedDing: props.dingNote || selectedDing,
  selectedNoteCount: props.noteCount,
  currentStage
})

// Define safe color retrieval utility function
const getColorSafely = (category) => {
  if (!category || typeof getCategoryColor !== 'function') {
    return '#607D8B'; // Default gray
  }
  try {
    return getCategoryColor(category);
  } catch {
    return '#607D8B'; // Default gray
  }
};



// Function to copy notation to clipboard
const copyNotation = () => {
  navigator.clipboard.writeText(formattedNotes.value)
    .then(() => {
      copied.value = true;
      // Reset copied state after 2 seconds
      setTimeout(() => {
        copied.value = false;
      }, 2000);
      
      // Show notification
      const { showNotification } = useNotification();
      showNotification('Notation copied to clipboard!', 'success');
    })
    .catch(err => {
      console.error('Failed to copy notation:', err);
      const { showNotification } = useNotification();
      showNotification('Failed to copy notation', 'error');
    });
};

// Get the actual ding note value directly from props or selectedDing
const dingNoteValue = computed(() => {
  // First try to get it from props
  if (props.dingNote && typeof props.dingNote === 'string') {
    return props.dingNote;
  }

  // Then try to get it from selectedDing
  if (selectedDing && typeof selectedDing === 'string') {
    return selectedDing;
  }

  if (selectedDing && typeof selectedDing === 'object') {
    if (selectedDing.note) return selectedDing.note;
    if (selectedDing.value) return selectedDing.value;
  }

  // Fallback to a default value
  return 'D';
});

// Computed property for formatted notes string
const formattedNotes = computed(() => {
  if (!dingNoteValue.value) {
    return 'No notes';
  }

  // Format as "ding/ 1st_note 2nd_note etc..." with position formatting
  const dingNote = `${dingNoteValue.value}`;
  
  // Create a prioritized list of notes based on position
  // This will respect the selectedNoteCount limit
  const allFormattedNotes = [];
  
  // Add top notes first
  topNotes.value.forEach(note => {
    const noteValue = note.note || note.calculated_note || '?';
    allFormattedNotes.push(noteValue); // Top notes as is
  });
  
  // Then add inner notes
  innerNotes.value.forEach(note => {
    const noteValue = note.note || note.calculated_note || '?';
    allFormattedNotes.push(`[${noteValue}]`); // Inner notes in square brackets
  });
  
  // Finally add bottom notes
  bottomNotes.value.forEach(note => {
    const noteValue = note.note || note.calculated_note || '?';
    allFormattedNotes.push(`(${noteValue})`); // Bottom notes in parentheses
  });
  
  // Join all formatted notes with spaces
  const otherNotes = allFormattedNotes.join(' ');

  return `${dingNote}/ ${otherNotes}`;
});

// Methods
const generateTab = () => {
  emit('generate-tab')
}

const resetAndGoBack = () => {
  // Don't reset everything, just go back to scale selection (stage 3)
  // Keep the selected category but clear scale, ding and note count
  selectedScale.value = null;
  selectedDing.value = null;
  selectedNoteCount.value = null;

  // Go to stage 3 (scale selection)
  goToStage(3);

  // Emit event to notify parent component
  emit('back-to-scales');
}

// Debug functions removed - we rely on the API for notes

// Initialize when stage changes to 5+
watch(() => currentStage.value, (newStage, oldStage) => {
  if (newStage < 5 && oldStage >= 5) {
    // Use the cleanup function from the composable if we were past stage 5
    if (typeof cleanup === 'function') {
      cleanup();
    }
  }
});


// Clean up on unmount
onMounted(() => {
  // Return cleanup function
  return () => {
    if (typeof cleanup === 'function') {
      cleanup();
    }
  }
});
</script>

<style>
@import '../assets/styles/components/HandpanDisplay/base.css';
@import '../assets/styles/components/HandpanDisplay/handpan3d.css';
</style>
