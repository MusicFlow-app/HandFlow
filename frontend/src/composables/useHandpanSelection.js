import { ref, computed, watch, nextTick } from 'vue'
import { useCookies } from '@vueuse/integrations/useCookies'
import useNotification from '@/composables/useNotification'

// Définition des couleurs avec dégradé arc-en-ciel pour les catégories
const CATEGORY_COLORS = {
  'Modal': 'var(--helio-category-1)',        // Rouge pastel vif
  'Oriental': 'var(--helio-category-2)',     // Rose corail
  'PentatonicAsian': 'var(--helio-category-3)', // Violet lilas
  'MinorFolk': 'var(--helio-category-4)',    // Bleu-violet
  'Mystical': 'var(--helio-category-5)',     // Bleu azur
  'Ethnic': 'var(--helio-category-6)',       // Turquoise
  'Experimental': 'var(--helio-category-7)', // Vert menthe
  'Import': 'var(--helio-category-8)',       // Jaune ambre
  'Favorites': 'var(--helio-category-9)'     // Orange pastel
}

export default function useHandpanSelection() {
  // Initialiser le système de notification
  const notification = useNotification()
  
  // État de l'interface
  const currentStage = ref(1)
  const isLoading = ref(false)
  const isTransitioning = ref(false)
  const error = ref(null) // Gardé pour compatibilité avec le code existant
  const errorType = ref('error') // Gardé pour compatibilité avec le code existant
  const showImportForm = ref(false)
  
  // Données
  const categories = ref([])
  const scales = ref([])
  const dings = ref([])
  const noteOptions = ref([])
  const notes = ref([])
  
  // Sélections de l'utilisateur
  const selectedCategory = ref(null)
  const selectedScale = ref(null)
  const selectedDing = ref(null)
  const selectedNoteCount = ref(null)
  const importNotation = ref('')
  const importScaleName = ref('My Custom Scale')
  const cameFromImport = ref(false) // Pour suivre si on vient de l'importation
  
  // Favoris
  const favorites = ref([])
  
  // Indique si nous sommes dans la vue des favoris
  const isViewingFavorites = ref(false)
  
  // Échelles personnalisées (pour les favoris)
  const customScales = ref([])
  
  // Filtres
  const filteredScales = computed(() => {
    // Si nous sommes dans la vue des favoris, retourner les favoris
    if (isViewingFavorites.value) {
      return customScales.value
    }
    
    if (!scales.value.length) return []
    if (!selectedCategory.value) return scales.value
    
    // Filtrer les échelles par catégorie
    let result = scales.value.filter(scale => 
      scale.category === selectedCategory.value.id
    )
    
    // Trier les échelles pour appliquer le dégradé de contraste de manière cohérente
    // Trier par nom pour avoir un ordre prévisible
    result.sort((a, b) => {
      // D'abord trier par nombre de notes (du plus petit au plus grand)
      if (a.max_notes !== b.max_notes) {
        return a.max_notes - b.max_notes
      }
      // Ensuite par nom
      return a.name.localeCompare(b.name)
    })
    
    return result
  })
  
  // Calcul des positions orbitales
  const calculateOrbitalPositions = (items, radius = 250) => {
    const positions = []
    const count = items?.length || 0
    
    // Si le tableau est vide, retourner un tableau vide
    if (count === 0) {
      return positions
    }
    
    // Ajuster le rayon en fonction de la largeur de l'écran
    let adjustedRadius = radius;
    
    // Réduire le rayon pour les écrans étroits
    if (window.innerWidth < 560) {
      adjustedRadius = Math.min(radius * 0.8, 200); // 20% plus petit, max 200px
    }
    if (window.innerWidth < 480) {
      adjustedRadius = Math.min(radius * 0.7, 175); // 30% plus petit, max 175px
    }
    if (window.innerWidth < 360) {
      adjustedRadius = Math.min(radius * 0.6, 150); // 40% plus petit, max 150px
    }
    
    // Réduire davantage si la hauteur est également contrainte
    if (window.innerHeight < 650) {
      adjustedRadius = adjustedRadius * 0.9; // 10% plus petit
    }
    
    for (let i = 0; i < count; i++) {
      // Calculer l'angle en radians (distribuer uniformément autour du cercle)
      const angle = (i * 2 * Math.PI / count) - Math.PI/2 // Commencer en haut
      
      // Calculer les coordonnées x et y avec le rayon ajusté
      const x = Math.cos(angle) * adjustedRadius
      const y = Math.sin(angle) * adjustedRadius
      
      positions.push({ x, y, angle })
    }
    
    return positions
  }
  
  // Charger les données depuis l'API
  const fetchCategories = async () => {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await fetch('/api/handpan/categories')
      if (!response.ok) throw new Error('Failed to fetch categories')
      
      const data = await response.json()
      categories.value = data
      
      // Ajouter la catégorie d'importation avec description
      categories.value.push({
        id: 'Import',
        name: 'Import Custom Scale',
        description: 'Import your own custom scale notation'
      })
      
      // Charger les favoris depuis les cookies
      loadFavoritesFromCookies()
    } catch (err) {
      // Error handled by notification
      notification.error(err.message)
    } finally {
      isLoading.value = false
    }
  }
  
  const fetchScales = async () => {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await fetch('/api/handpan/scales')
      if (!response.ok) throw new Error('Failed to fetch scales')
      
      scales.value = await response.json()
    } catch (err) {
      // Error handled by notification
      notification.error(err.message)
    } finally {
      isLoading.value = false
    }
  }
  
  const fetchDings = async () => {
    isLoading.value = true
    error.value = null
    
    try {
      const response = await fetch('/api/handpan/dings')
      if (!response.ok) throw new Error('Failed to fetch dings')
      
      dings.value = await response.json()

    } catch (err) {
      // Error handled by notification
      notification.error(err.message)
    } finally {
      isLoading.value = false
    }
  }
  
  const fetchNotes = async () => {
    if (!selectedScale.value || !selectedDing.value) return
    
    isLoading.value = true
    error.value = null
    
    try {
      let response;
      
      // Check if this is an imported scale (category === 'Import')
      if (selectedScale.value.category === 'Import') {


        
        // Check if this is a favorite with stored notation
        const favorite = favorites.value.find(fav => 
          fav.id === selectedScale.value.id && fav.selectedDing === selectedDing.value
        );
        

        
        // Get notation from favorite if available, otherwise from the scale
        const notation = favorite?.notation || selectedScale.value.notation || '';
        

        
        // For imported scales, use the POST endpoint with the notation and selected ding
        response = await fetch('/api/handpan/import/notation', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            notation: notation, // Use the notation from favorite or scale
            ding_wanted: selectedDing.value, // Use the selected ding
            name: selectedScale.value.name || 'Custom Scale'
          })
        })
      } else {
        // For built-in scales, use the existing GET endpoint
        // Properly encode URI components to handle special characters like #
        const scaleId = encodeURIComponent(selectedScale.value.id)
        const dingNote = encodeURIComponent(selectedDing.value)
        
        response = await fetch(`/api/handpan/${scaleId}/notes/${dingNote}`)
      }
      
      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || 'Failed to fetch notes')
      }
      
      const responseData = await response.json()
      
      // Handle different response formats based on the scale type
      if (selectedScale.value.category === 'Import') {
        // For imported scales, the response has a 'notes' property
        notes.value = responseData.notes || []
      } else {
        // For built-in scales, the response is the notes array directly
        notes.value = responseData
      }
      
      // Générer les options de nombre de notes
      generateNoteCountOptions()
    } catch (err) {
      // Error handled by notification
      notification.error(err.message)
    } finally {
      isLoading.value = false
    }
  }
  
  const importCustomScale = async () => {
    if (!importNotation.value || !importScaleName.value) {
      notification.error('Please fill in all required fields (name and notation)')
      return
    }
    
    isLoading.value = true
    error.value = null
    
    try {
      // Valider le format de la notation avec regex
      const notationValue = importNotation.value.trim();
      
      // Check for exactly one slash
      if ((notationValue.match(/\//g) || []).length !== 1) {
        throw new Error('The notation must include exactly one slash (/) to separate the ding from other notes')
      }
      
      // Split into ding and other notes
      const [dingNote, otherNotesStr] = notationValue.split('/');
      const trimmedDing = dingNote.trim();
      const otherNotes = otherNotesStr ? otherNotesStr.trim().split(/\s+/) : [];
      
      // Validate ding format: must be A-G, optionally followed by a number (2-6), optionally with a #
      const dingRegex = /^[A-G](#)?([2-6])?$/;
      if (!dingRegex.test(trimmedDing)) {
        throw new Error('The ding must be a valid note (A-G), optionally with a sharp (#) and/or a octave between 2-6')
      }
      
      // Validate other notes format and count
      const noteRegex = /^[A-G](#)?([2-6])?$/;
      const invalidNotes = otherNotes.filter(note => !noteRegex.test(note));
      
      if (invalidNotes.length > 0) {
        throw new Error(`The following notes are invalid: ${invalidNotes.join(', ')}. Notes must be A-G, optionally with a sharp (#) and/or a octave between 2-6`)
      }
      
      // Check minimum number of notes (6 notes + ding)
      if (otherNotes.length < 6) {
        throw new Error(`The notation must include at least 6 notes plus the ding. Currently you have ${otherNotes.length} notes.`)
      }
      
      // Vérifier si le ding est dans la liste des dings possibles
      if (dings.value.length > 0) {
        
        // Check the structure of dings to determine the correct property

        
        // Get the property that contains the note value (could be 'note', 'name', or the ding itself)
        let validDings = [];
        if (dings.value.length > 0) {
          // Check if dings are objects with a note property
          if (typeof dings.value[0] === 'object' && dings.value[0] !== null) {
            // Look for common properties that might contain the note value
            if ('note' in dings.value[0]) {
              validDings = dings.value.map(d => d.note);
            } else if ('name' in dings.value[0]) {
              validDings = dings.value.map(d => d.name);
            } else {
              // If we can't determine the property, use the first property available
              const firstKey = Object.keys(dings.value[0])[0];
              validDings = dings.value.map(d => d[firstKey]);
            }
          } else {
            // If dings are primitive values (strings), use them directly
            validDings = dings.value;
          }
        }
        

        
        if (validDings.length > 0 && !validDings.includes(trimmedDing)) {
          throw new Error(`The ding "${trimmedDing}" is not valid. Valid dings are: ${validDings[0]} -> ${validDings[validDings.length - 1]}`)
        }
      }
      

      
      const response = await fetch('/api/handpan/import/notation', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          notation: importNotation.value,
          ding_wanted: '', // Nous sélectionnerons le ding à l'étape 4
          name: importScaleName.value
        })
      })
      
      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || 'Failed to import custom scale')
      }
      
      const importedNotes = await response.json()
      notes.value = importedNotes
      
      // Fonction pour sanitizer le nom pour l'ID
      const sanitizeNameForId = (name) => {
        // Garder les lettres (casse préservée) et les chiffres, supprimer les espaces et caractères spéciaux
        return name.replace(/[^a-zA-Z0-9]/g, '');
      };
      
      // Créer un objet d'échelle personnalisée
      const customScale = {
        id: sanitizeNameForId(importScaleName.value) || `custom-${Date.now()}`,
        name: importScaleName.value,
        category: 'Import', // Utiliser 'Import' au lieu de 'Custom'
        notes: importedNotes,
        max_notes: importedNotes.length,
        notation: importNotation.value // Store the original notation for later use
      }
      
      // Ne pas ajouter automatiquement aux favoris
      // L'utilisateur pourra le faire à l'étape 5 avec le bouton favori
      
      // Ajouter l'échelle personnalisée aux échelles disponibles
      scales.value.push(customScale)
      
      // Sélectionner l'échelle importée
      selectedScale.value = customScale
      
      // Réinitialiser le formulaire
      importScaleName.value = ''
      importNotation.value = ''
      showImportForm.value = false
      
      notification.success(`Scale "${customScale.name}" has been successfully imported!`)
      
      // Marquer que nous venons de l'importation
      cameFromImport.value = true
      
      // Passer d'abord à l'étape 3 pour s'assurer que l'échelle est sélectionnée
      currentStage.value = 3
      
      // Puis passer à l'étape 4 pour sélectionner le ding après un court délai
      setTimeout(() => {
        goToStage(4)
      }, 100)
    } catch (err) {
      // Error handled by notification
      notification.error(err.message)
    } finally {
      isLoading.value = false
    }
  }
  
  // Générer les options de nombre de notes
  const generateNoteCountOptions = () => {
    if (!notes.value.length) return
    
    const minNotes = 6
    const maxNotes = notes.value.length
    
    noteOptions.value = []
    for (let i = minNotes; i <= maxNotes; i++) {
      noteOptions.value.push(i)
    }
  }
  
  // Réinitialiser toutes les sélections
  const resetSelection = () => {
    selectedCategory.value = null
    selectedScale.value = null
    selectedDing.value = null
    selectedNoteCount.value = null
    importNotation.value = ''
    showImportForm.value = false
    notes.value = []
  }

  // Navigation entre les étapes
  const goToStage = (stage) => {

    
    if (isTransitioning.value) {

      return
    }
    
    isTransitioning.value = true
    
    // Vérifier si on peut avancer à cette étape
    // Exceptions:
    // 1. L'importation peut aller directement à l'étape 4
    // 2. Les favoris avec un ding sélectionné peuvent aller directement à l'étape 5
    const isFavoriteWithDing = selectedScale.value && 
                              favorites.value.some(fav => 
                                fav.id === selectedScale.value.id && fav.selectedDing === selectedDing.value);
                             
    if (stage > currentStage.value + 1 && 
        stage !== 1 && 
        !(showImportForm.value && stage === 4) && 
        !(isFavoriteWithDing && stage === 5)) {

      isTransitioning.value = false
      return
    }
    
    // Logique spécifique à chaque étape
    if (stage === 1) {
      // Retour à l'écran d'accueil (sans reset)
      currentStage.value = stage
      isTransitioning.value = false
      return
    } else if (stage === 2) {
      // Passer à la sélection de catégorie
      // Charger les catégories si ce n'est pas déjà fait
      if (categories.value.length === 0) {
        fetchCategories().then(() => {
          currentStage.value = stage
          isTransitioning.value = false
        })
      } else {
        currentStage.value = stage
        isTransitioning.value = false
      }
      return
    } else if (stage === 3) {
      // Passer à la sélection d'échelle
      // Vérifier si une catégorie a été sélectionnée
      if (!selectedCategory.value) {

        isTransitioning.value = false
        return
      }
      
      // Si la catégorie est "Import", ouvrir le formulaire d'importation
      if (selectedCategory.value.id === 'Import') {
        showImportForm.value = true
        if (dings.value.length === 0) {
          fetchDings()
        }
        isTransitioning.value = false
        return
      }
      
      // Charger les échelles si ce n'est pas déjà fait
      if (scales.value.length === 0) {
        fetchScales().then(() => {
          // Filtrer les échelles par catégorie
          filteredScales.value = scales.value.filter(scale => 
            scale.category === selectedCategory.value.id
          )
          currentStage.value = stage
          isTransitioning.value = false
        })
      } else {
        // Filtrer les échelles par catégorie
        filteredScales.value = scales.value.filter(scale => 
          scale.category === selectedCategory.value.id
        )
        currentStage.value = stage
        isTransitioning.value = false
      }
      return
    } else if (stage === 4) {
      // Passer à la sélection du ding
      // Vérifier si une échelle a été sélectionnée
      if (!selectedScale.value) {

        isTransitioning.value = false
        return
      }
      
      // Charger les dings si ce n'est pas déjà fait
      if (dings.value.length === 0) {
        fetchDings().then(() => {
          currentStage.value = stage
          isTransitioning.value = false
        })
      } else {
        currentStage.value = stage
        isTransitioning.value = false
      }
      return
    } else if (stage === 5) {
      // Passer à la sélection du nombre de notes
      // Vérifier si un ding a été sélectionné
      if (!selectedDing.value) {

        isTransitioning.value = false
        return
      }
      
      // Charger les notes pour cette échelle et ce ding
      fetchNotes().then(() => {
        currentStage.value = stage
        isTransitioning.value = false
      })
      return
    }
    
    // Si on arrive ici, c'est qu'il y a un problème

    isTransitioning.value = false
  }
  
  const goBack = () => {
    if (isTransitioning.value || currentStage.value === 1) return
    
    // Réinitialiser les sélections en fonction de l'étape actuelle avant de revenir en arrière
    const currentStageValue = currentStage.value
    
    // Gestion spéciale pour le retour après importation
    if (currentStageValue === 4 && cameFromImport.value) {
      // Si on vient de l'importation et qu'on est à l'étape 4, revenir à l'étape 2
      selectedDing.value = null
      selectedNoteCount.value = null
      notes.value = []
      
      // Réinitialiser le flag d'importation
      cameFromImport.value = false
      
      // Aller directement à l'étape 2
      goToStage(2)
      return
    }
    
    if (currentStageValue === 5) {
      // Revenir de la sélection du nombre de notes à la sélection du ding
      selectedNoteCount.value = null
      notes.value = []
    } else if (currentStageValue === 4) {
      // Revenir de la sélection du ding à la sélection de l'échelle
      selectedDing.value = null
      selectedNoteCount.value = null
      notes.value = []
    } else if (currentStageValue === 3) {
      // Revenir de la sélection de l'échelle à la sélection de la catégorie
      selectedScale.value = null
      selectedDing.value = null
      selectedNoteCount.value = null
      notes.value = []
    }
    // Pas de réinitialisation si on revient de l'étape 2 à l'étape 1
    
    // Maintenant, aller à l'étape précédente
    goToStage(currentStageValue - 1)
  }
  
  // Sélection des éléments
  const selectCategory = (category) => {

    selectedCategory.value = category
    
    // Gestion spéciale pour la catégorie Import
    if (category.id === 'Import') {
      showImportForm.value = true
      return
    }
    
    // Gestion spéciale pour la catégorie Favoris
    if (category.id === 'Favorites') {

      
      // Vérifier si nous avons des favoris
      if (favorites.value.length === 0) {
        // Aucun favori - afficher un message
        error.value = 'You don\'t have any favorites yet. Select a scale and click on the star icon to add it to your favorites.'
        errorType.value = 'warning'
        setTimeout(() => error.value = null, 5000) // Effacer le message après 5 secondes
        return
      }
      
      
      // S'assurer que les favoris ont toutes les propriétés nécessaires
      const validFavorites = favorites.value.map(fav => {

        return {
          id: fav.id || `unknown-${Math.random().toString(36).substring(7)}`,
          name: fav.name || 'Unnamed Scale',
          category: fav.category || 'Modal',
          notes: fav.notes || [],
          max_notes: fav.max_notes || 8,
          description: fav.description || '',
          selectedDing: fav.selectedDing || null
        }
      })
      
      // Mettre à jour les échelles personnalisées et activer la vue des favoris
      customScales.value = validFavorites
      isViewingFavorites.value = true

      
      // Forcer une mise à jour du DOM avant de passer à l'étape suivante
      setTimeout(() => {
        // Passer à l'étape 3
        goToStage(3)
      }, 50)
      return
    }
    
    // Pour les autres catégories, désactiver la vue des favoris
    isViewingFavorites.value = false
    
    // Précharger les échelles pour cette catégorie
    if (scales.value.length === 0) {
      fetchScales().then(() => {
        filteredScales.value = scales.value.filter(scale => 
          scale.category === category.id
        )
        setTimeout(() => goToStage(3), 100)
      })
    } else {
      filteredScales.value = scales.value.filter(scale => 
        scale.category === category.id
      )
      setTimeout(() => goToStage(3), 100)
    }
  }
  
  const selectScale = (scale) => {

    
    // Check if this is a favorite with a saved ding
    const favorite = favorites.value.find(fav => fav.id === scale.id);
    if (favorite && favorite.selectedDing) {


      
      // Create a new scale object that includes properties from both the original scale and the favorite
      const mergedScale = {
        ...scale,
        // Ensure we have the notation from the favorite if it exists
        notation: favorite.notation || scale.notation || '',
        // Include any other properties from the favorite that might be needed
        selectedDing: favorite.selectedDing,
        uniqueId: favorite.uniqueId
      }
      
      // Set the merged scale as the selected scale
      selectedScale.value = mergedScale
      
      // Set the saved ding
      selectedDing.value = favorite.selectedDing
      
      // Go directly to stage 5 (note count selection) - fetchNotes will be called there
      setTimeout(() => goToStage(5), 100)
      return
    }
    
    // If not a favorite, just set the scale as is
    selectedScale.value = scale
    
    // Précharger les dings si nécessaire
    if (dings.value.length === 0) {
      fetchDings().then(() => {
        setTimeout(() => goToStage(4), 100)
      })
    } else {
      setTimeout(() => goToStage(4), 100)
    }
  }
  
  const selectDing = (ding) => {

    selectedDing.value = ding
    
    // Transition to stage 5 which will handle fetching notes
    setTimeout(() => goToStage(5), 100)
  }
  
  const selectNoteCount = (count) => {
    selectedNoteCount.value = count
    
    // Filtrer les notes en fonction du nombre sélectionné
    if (notes.value.length > count) {
      notes.value = notes.value.slice(0, count)
    }
  }
  
  // Gestion des favoris
  const toggleFavorite = () => {
    if (!selectedScale.value) return
    
    // Create a unique ID combining scale ID and selected ding
    const currentDing = selectedDing.value || null
    const uniqueId = `${selectedScale.value.id}:${currentDing}`
    
    // Find the favorite with the same scale ID and ding
    const index = favorites.value.findIndex(fav => 
      fav.id === selectedScale.value.id && fav.selectedDing === currentDing
    )
    
    if (index === -1) {
      // Ajouter aux favoris - s'assurer que toutes les propriétés nécessaires sont copiées
      const favoriteScale = {
        id: selectedScale.value.id,
        name: selectedScale.value.name,
        category: selectedScale.value.category,
        // For imported scales, ensure we store just the notes array, not the entire response
        notes: selectedScale.value.notes?.notes || selectedScale.value.notes || [],
        max_notes: selectedScale.value.max_notes || 8,
        // Save the selected ding if available
        selectedDing: currentDing,
        // Store the unique ID for reference
        uniqueId: uniqueId,
        // Store the original notation for imported scales
        notation: selectedScale.value.notation || ''
      }
      favorites.value.push(favoriteScale)

      
      // Notification for adding to favorites
      const dingInfo = currentDing ? ` with ding ${currentDing}` : ''
      notification.success(`"${selectedScale.value.name}"${dingInfo} added to favorites`)
    } else {
      // Retirer des favoris
      favorites.value.splice(index, 1)

      
      // Notification for removing from favorites
      const dingInfo = currentDing ? ` with ding ${currentDing}` : ''
      notification.warning(`"${selectedScale.value.name}"${dingInfo} removed from favorites`)
    }
    
    // Forcer la mise à jour immédiate des cookies
    nextTick(() => {
      saveFavoritesToCookies()

    })
    
    // Si nous sommes dans la catégorie Favoris, mettre à jour la liste des échelles filtrées
    if (selectedCategory.value && selectedCategory.value.id === 'Favorites') {
      const validFavorites = favorites.value.map(fav => ({
        id: fav.id,
        name: fav.name,
        category: fav.category,
        notes: fav.notes || [],
        max_notes: fav.max_notes || 8,
        description: fav.description || '',
        selectedDing: fav.selectedDing || null,
        notation: fav.notation || ''
      }))
      filteredScales.value = validFavorites

    }
  }
  
  const isFavorite = computed(() => {
    if (!selectedScale.value) return false
    const currentDing = selectedDing.value || null
    return favorites.value.some(fav => 
      fav.id === selectedScale.value.id && fav.selectedDing === currentDing
    )
  })
  
  // Gestion des cookies pour les favoris
  const cookie_Key = 'handflow_handpan_favorites'
  const saveFavoritesToCookies = () => {
    const cookies = useCookies([cookie_Key])
    const favoritesJson = JSON.stringify(favorites.value)
    cookies.set(cookie_Key, favoritesJson, { path: '/', maxAge: 31536000 }) // 1 an

  }
  
  const loadFavoritesFromCookies = () => {

    const cookies = useCookies([cookie_Key])
    const favoritesJson = cookies.get(cookie_Key)

    
    if (favoritesJson) {
      try {
        // Handle the case where the value might already be an object
        // (useCookies might automatically parse JSON unlike document.cookie)
        let parsed = favoritesJson
        
        // If it's a string, try to parse it
        if (typeof favoritesJson === 'string') {

          parsed = JSON.parse(favoritesJson)
        } else {

        }

        
        // Vérifier que les favoris sont un tableau valide
        if (Array.isArray(parsed)) {

          
          // S'assurer que chaque favori a toutes les propriétés nécessaires
          const validFavorites = parsed.map(fav => {

            return {
              id: fav.id || `unknown-${Math.random().toString(36).substring(7)}`,
              name: fav.name, // Preserve the original name without fallback
              category: fav.category || 'Import',
              notes: fav.notes || [],
              max_notes: fav.max_notes || 8,
              // Load the selected ding if available
              selectedDing: fav.selectedDing || null,
              // Preserve the original notation for imported scales
              notation: fav.notation || '',
              // Preserve the unique ID
              uniqueId: fav.uniqueId || `${fav.id}:${fav.selectedDing || ''}`
            }
          })
          
          favorites.value = validFavorites

        } else {

          favorites.value = []
        }
      } catch (err) {

        favorites.value = []
      }
    } else {

      favorites.value = []
    }
  }
  
  // Obtenir la couleur pour une catégorie
  const getCategoryColor = (category) => {
  if (!category) return '#CCCCCC'
  
  // Si c'est un objet avec un id, utiliser l'id
  if (typeof category === 'object' && category.id) {
    return CATEGORY_COLORS[category.id] || '#CCCCCC'
  }
  
  // Pour les catégories provenant de l'API, elles sont au format "Category"
  // Nous devons les convertir au format attendu par CATEGORY_COLORS
  if (typeof category === 'string') {
    // Essayer d'abord avec la valeur exacte
    if (CATEGORY_COLORS[category]) {
      return CATEGORY_COLORS[category]
    }

    // Essayer en enlevant les guillemets si présents
    const cleanCategory = category.replace(/["']/g, '')
    if (CATEGORY_COLORS[cleanCategory]) {
      return CATEGORY_COLORS[cleanCategory]
    }

    // Essayer toutes les clés pour voir si l'une d'elles est contenue dans la catégorie
    for (const key in CATEGORY_COLORS) {
      if (category.includes(key)) {
        return CATEGORY_COLORS[key]
      }
    }
  }
    
    // Valeur par défaut
    return '#CCCCCC'
  }
  
  // Fonction spécifique pour les dings avec gradient inversé (sombre -> clair)
  const getDingVariation = (hexColor, index, total) => {
    try {
      // Vérifier que la couleur est au format hexadécimal valide
      if (!hexColor || typeof hexColor !== 'string' || !hexColor.startsWith('#') || hexColor.length !== 7) {

        hexColor = '#CCCCCC' // Utiliser une couleur par défaut
      }
      
      // Convertir la couleur hexadécimale en composantes RGB
      const r = parseInt(hexColor.slice(1, 3), 16)
      const g = parseInt(hexColor.slice(3, 5), 16)
      const b = parseInt(hexColor.slice(5, 7), 16)
      
      // Vérifier que les composantes RGB sont valides
      if (isNaN(r) || isNaN(g) || isNaN(b)) {

        return '#CCCCCC'
      }
      
      // INVERSÉ: Calculer le facteur de variation pour la luminosité
      // Le premier élément est 40% plus sombre, puis on devient progressivement plus clair
      // Variation de -40% (plus sombre) à +20% de blanc (plus clair)
      // Nous inversons la formule originale de getVariation
      const variationFactor = 0.6 + (0.6 * index / (total - 1 || 1))
      
      // Appliquer la variation de luminosité
      let newR, newG, newB
      
      if (variationFactor >= 1) {
        // Éclaircir (ajouter du blanc)
        const whiteAmount = (variationFactor - 1) * 255
        newR = Math.min(255, Math.round(r + whiteAmount))
        newG = Math.min(255, Math.round(g + whiteAmount))
        newB = Math.min(255, Math.round(b + whiteAmount))
      } else {
        // Assombrir
        newR = Math.max(0, Math.round(r * variationFactor))
        newG = Math.max(0, Math.round(g * variationFactor))
        newB = Math.max(0, Math.round(b * variationFactor))
      }
      
      // Convertir en hexadécimal
      return `#${newR.toString(16).padStart(2, '0')}${newG.toString(16).padStart(2, '0')}${newB.toString(16).padStart(2, '0')}`
    } catch (error) {

      return '#CCCCCC'
    }
  }

  // Créer des variations de couleur pour les éléments d'une même catégorie
  const getColorVariation = (baseColor, index, total) => {
    // Assurer que nous avons un index et un total valides
    if (typeof index !== 'number' || typeof total !== 'number' || total <= 0) {
      index = 0
      total = 1
    }
    
    // Log pour débogage

    
    // Déterminer si nous sommes dans l'étape de sélection des dings (stage 4)
    const isDingSelection = currentStage.value === 4
    
    // Gérer les variables CSS
    if (baseColor && typeof baseColor === 'string' && baseColor.startsWith('var(')) {
      // Extraire le nom de la variable CSS
      const varName = baseColor.match(/var\((.*?)\)/)?.[1]
      
      if (varName) {
        // Obtenir la valeur de la variable CSS
        const hexColor = getComputedStyle(document.documentElement).getPropertyValue(varName).trim()
        
        // Si la valeur est une couleur hexadécimale valide, l'utiliser
        if (hexColor && hexColor.startsWith('#')) {
          // Utiliser la fonction appropriée selon l'étape
          return isDingSelection 
            ? getDingVariation(hexColor, index, total) 
            : getVariation(hexColor, index, total)
        }
      }
      
      // Couleur par défaut si la variable n'est pas reconnue
      return isDingSelection 
        ? getDingVariation('#CCCCCC', index, total) 
        : getVariation('#CCCCCC', index, total)
    }
    
    // Si c'est déjà une couleur hexadécimale, l'utiliser directement
    if (baseColor && baseColor.startsWith('#')) {
      return isDingSelection 
        ? getDingVariation(baseColor, index, total) 
        : getVariation(baseColor, index, total)
    }
    
    // Couleur par défaut

    return isDingSelection 
      ? getDingVariation('#CCCCCC', index, total) 
      : getVariation('#CCCCCC', index, total)
  }
  
  // Fonction interne pour calculer la variation de couleur
  const getVariation = (hexColor, index, total) => {
    try {
      // Vérifier que la couleur est au format hexadécimal valide
      if (!hexColor || typeof hexColor !== 'string' || !hexColor.startsWith('#') || hexColor.length !== 7) {

        hexColor = '#CCCCCC' // Utiliser une couleur par défaut
      }
      
      // Convertir la couleur hexadécimale en composantes RGB
      const r = parseInt(hexColor.slice(1, 3), 16)
      const g = parseInt(hexColor.slice(3, 5), 16)
      const b = parseInt(hexColor.slice(5, 7), 16)
      
      // Vérifier que les composantes RGB sont valides
      if (isNaN(r) || isNaN(g) || isNaN(b)) {

        return '#CCCCCC'
      }
      
      // Calculer le facteur de variation pour la luminosité
      // Le premier élément est 20% plus clair, puis on devient progressivement plus sombre
      // Variation de +20% de blanc à -40% (plus sombre) pour un contraste plus prononcé
      const variationFactor = 1.2 - (0.6 * index / (total - 1 || 1))
      
      // Appliquer la variation de luminosité
      // Pour éclaircir, on ajoute du blanc (augmente toutes les composantes)
      // Pour assombrir, on réduit toutes les composantes
      let newR, newG, newB
      
      if (variationFactor >= 1) {
        // Éclaircir (ajouter du blanc)
        const whiteAmount = (variationFactor - 1) * 255
        newR = Math.min(255, Math.round(r + whiteAmount))
        newG = Math.min(255, Math.round(g + whiteAmount))
        newB = Math.min(255, Math.round(b + whiteAmount))
      } else {
        // Assombrir
        newR = Math.max(0, Math.round(r * variationFactor))
        newG = Math.max(0, Math.round(g * variationFactor))
        newB = Math.max(0, Math.round(b * variationFactor))
      }
      
      // Convertir en hexadécimal
      return `#${newR.toString(16).padStart(2, '0')}${newG.toString(16).padStart(2, '0')}${newB.toString(16).padStart(2, '0')}`
    } catch (error) {

      return '#CCCCCC'
    }
  }
  
  return {
    // État
    currentStage,
    isLoading,
    isTransitioning,
    error,
    errorType,
    showImportForm,
    
    // Données
    categories,
    scales,
    dings,
    noteOptions,
    notes,
    filteredScales,
    favorites,
    isViewingFavorites,
    cameFromImport,
    
    // Sélections
    selectedCategory,
    selectedScale,
    selectedDing,
    selectedNoteCount,
    importNotation,
    importScaleName,
    
    // Méthodes de navigation
    goToStage,
    goBack,
    
    // Méthodes de sélection
    selectCategory,
    selectScale,
    selectDing,
    selectNoteCount,
    
    // Méthodes d'API
    fetchCategories,
    fetchScales,
    fetchDings,
    fetchNotes,
    importCustomScale,
    
    // Méthodes utilitaires
    calculateOrbitalPositions,
    getCategoryColor,
    getColorVariation,
    getDingVariation,
    
    // Gestion des favoris
    toggleFavorite,
    isFavorite,
    loadFavoritesFromCookies,
    saveFavoritesToCookies,
    
    // Réinitialisation
    resetSelection
  }
}
