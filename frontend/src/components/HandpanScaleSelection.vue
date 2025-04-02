<template>
  <div class="heliocentric-container">
    <!-- Indicateur d'étape -->
    <div class="stage-indicator">
      <div 
        v-for="stage in 5" 
        :key="stage" 
        class="stage-dot" 
        :class="{ 
          active: currentStage === stage, 
          completed: currentStage > stage 
        }"
      ></div>
    </div>
    
    <!-- Bouton retour (visible à partir de l'étape 2) -->
    <button 
      v-if="currentStage > 1" 
      class="back-button" 
      @click="goBack"
      :disabled="isTransitioning"
    >
      <PhCaretLeft :size="32" weight="fill" />
    </button>
    
    <!-- Notifications sont maintenant gérées par le composant global NotificationContainer -->
    
    <!-- Indicateur de chargement -->
    <div v-if="isLoading" class="loading-overlay">
      <div class="loading-spinner"></div>
      <p>Loading...</p>
    </div>
    
    <!-- Étape 1: Écran d'accueil -->
    <transition name="stage-transition">
      <div v-if="currentStage === 1" class="stage-content">
        <div class="center-circle call-to-action" @click="goToStage(2)">
          <h3>Select your handpan</h3>
          <p>Click to start</p>
        </div>
      </div>
    </transition>
    
    <!-- Étape 2: Sélection de catégorie -->
    <transition name="stage-transition">
      <div v-if="currentStage === 2" class="stage-content">
        <div class="center-circle" :class="{ 'highlight': hoveredCategory }">
          <h3>{{ hoveredCategory ? hoveredCategory.name : 'Choose a category' }}</h3>
          <p v-if="hoveredCategory">{{ hoveredCategory.description }}</p>
          <p v-else>Select a handpan scale category</p>
        </div>
        
        <div class="orbit">
          <div 
            v-for="(category, index) in displayedCategories" 
            :key="category.id"
            class="orbit-item"
            :class="{ 
              selected: selectedCategory && selectedCategory.id === category.id,
              'moving-to-center': isTransitioning && selectedCategory && selectedCategory.id === category.id,
              'import-category': category.id === 'Import',
              'favorites-category': category.id === 'Favorites'
            }"
            :style="{
              backgroundColor: getCategoryColor(category),
              transform: orbitalPositions[index] ? `translate(${orbitalPositions[index].x}px, ${orbitalPositions[index].y}px)` : 'translate(0, 0)'
            }"
            @click="selectCategory(category)"
            @mouseenter="showCategoryInfo(category)"
            @mouseleave="hideCategoryInfo"
          >
            <!-- L'indicateur d'étoile a été supprimé car nous utilisons maintenant une catégorie Favorites dédiée -->
            
            <!-- Icône pour la catégorie Import -->
            <template v-if="category.id === 'Import'">
              <PhMusicNotesPlus :size="32" weight="fill" class="import-icon" />
            </template>
            
            <!-- Icône pour la catégorie Favoris -->
            <template v-else-if="category.id === 'Favorites'">
              <PhStar :size="32" weight="fill" class="favorites-icon" />
            </template>
            
            <!-- Abréviation du nom pour les autres catégories -->
            <template v-else>
              <span class="category-abbreviation">{{ getCategoryAbbreviation(category.name) }}</span>
            </template>
            

          </div>
        </div>
      </div>
    </transition>
    
    <!-- Étape 3: Sélection d'échelle -->
    <transition name="stage-transition">
      <div v-if="currentStage === 3" class="stage-content">
        <div class="center-circle">
          <h3>{{ hoveredScale ? hoveredScale.name : 'Choose a scale' }}</h3>
          <p v-if="hoveredScale && isViewingFavorites">
            <span>Category: {{ hoveredScale.category === 'Import' ? 'Import' : hoveredScale.category }}</span>
          </p>
          <p v-else>
            {{ selectedCategory ? selectedCategory.name : 'Select a scale' }}
          </p>
          <p v-if="hoveredScale" class="note-count">Notes: 6~{{ hoveredScale.max_notes }}</p>
          <p v-if="hoveredScale && hoveredScale.selectedDing" class="selected-ding">
            <span><strong>Selected Ding:</strong> {{ hoveredScale.selectedDing }}</span>
          </p>
        </div>
        
        <div class="orbit">
          <div 
            v-for="(scale, index) in filteredScales" 
            :key="scale.id"
            class="orbit-item"
            :class="{ 
              selected: selectedScale && selectedScale.id === scale.id,
              'moving-to-center': isTransitioning && selectedScale && selectedScale.id === scale.id
            }"
            :style="getItemStyle(scale, index)"
            @click="selectScale(scale)"
            @mouseenter="showScaleInfo(scale)"
            @mouseleave="hideScaleInfo"
          >
            <div class="scale-content">
              <!-- Indicateur de favori -->
              <span 
                v-if="favorites.some(fav => fav.id === scale.id)" 
                class="favorite-indicator"
              >★</span>
              
              <!-- Abréviation de l'échelle basée sur l'ID -->
              <span class="scale-abbreviation"><strong>{{ getScaleAbbreviation(scale) }}</strong></span>
            </div>
          </div>
        </div>
      </div>
    </transition>
    
    <!-- Étape 4: Sélection du ding -->
    <transition name="stage-transition">
      <div v-if="currentStage === 4" class="stage-content">
        <div class="center-circle">
          <h3 v-if="selectedDing">Selected Ding: {{ selectedDing }}</h3>
          <h3 v-else>Choose a ding</h3>
          <p>{{ selectedScale ? selectedScale.name : 'Select a ding note' }}</p>
        </div>
        
        <!-- Orbite intérieure (8 premiers éléments) -->
        <div class="orbit inner-orbit">
          <div 
            v-for="(ding, index) in innerDings" 
            :key="ding"
            class="orbit-item"
            :class="{ 
              selected: selectedDing === ding,
              'moving-to-center': isTransitioning && selectedDing === ding
            }"
            :style="{
              backgroundColor: getColorVariation(getCategoryColor(selectedScale?.category), index, dings.length),
              borderColor: getColorVariation(getCategoryColor(selectedScale?.category), index, dings.length),
              transform: innerOrbitalPositions[index] ? `translate(${innerOrbitalPositions[index].x}px, ${innerOrbitalPositions[index].y}px)` : 'translate(0, 0)'
            }"
            @click="selectDing(ding)"
          >
            <span class="category-abbreviation">{{ ding }}</span>
          </div>
        </div>
        
        <!-- Orbite extérieure (éléments restants) -->
        <div class="orbit outer-orbit">
          <div 
            v-for="(ding, index) in outerDings" 
            :key="ding"
            class="orbit-item"
            :class="{ 
              selected: selectedDing === ding,
              'moving-to-center': isTransitioning && selectedDing === ding
            }"
            :style="{
              backgroundColor: getColorVariation(getCategoryColor(selectedScale?.category), index + innerDings.length, dings.length),
              borderColor: getColorVariation(getCategoryColor(selectedScale?.category), index + innerDings.length, dings.length),
              transform: outerOrbitalPositions[index] ? `translate(${outerOrbitalPositions[index].x}px, ${outerOrbitalPositions[index].y}px)` : 'translate(0, 0)'
            }"
            @click="selectDing(ding)"
          >
            <span class="category-abbreviation">{{ ding }}</span>
          </div>
        </div>
      </div>
    </transition>
    
    <!-- Étape 5: Sélection du nombre de notes -->
    <transition name="stage-transition">
      <div v-if="currentStage === 5" class="stage-content">
        <div class="center-circle">
          <h3>Choose note count</h3>
          <p>{{ selectedScale ? selectedScale.name : 'Select number of notes' }}</p>
          
          <button 
            class="favorite-button" 
            :class="{ active: isFavorite }"
            @click="toggleFavorite"
          >
            <PhStar :size="24" :weight="isFavorite ? 'fill' : 'regular'" />
          </button>
        </div>
        
        <div class="orbit">
          <div 
            v-for="(count, index) in noteOptions" 
            :key="count"
            class="orbit-item"
            :class="{ 
              selected: selectedNoteCount === count
            }"
            :style="{
              backgroundColor: getColorVariation(getCategoryColor(selectedScale?.category), index, noteOptions.length),
              borderColor: getColorVariation(getCategoryColor(selectedScale?.category), index, noteOptions.length),
              transform: orbitalPositions[index] ? `translate(${orbitalPositions[index].x}px, ${orbitalPositions[index].y}px)` : 'translate(0, 0)'
            }"
            @click="selectNoteCount(count)"
          >
            <span class="category-abbreviation">{{ count }}</span>
          </div>
        </div>
        
        <!-- Affichage des informations sur les notes sélectionnées -->
        <div v-if="selectedNoteCount && notes.length" class="notes-info">
          <h4>Selected Notes</h4>
          <div class="notes-list">
            <div 
              v-for="(note, index) in notes.slice(0, selectedNoteCount)" 
              :key="index"
              class="note-item"
            >
              {{ note.note }} ({{ note.position }})
            </div>
          </div>
        </div>
      </div>
    </transition>
    
    <!-- Formulaire d'importation -->
    <div v-if="showImportForm" class="import-form">
      <button class="close-button" @click="showImportForm = false">&times;</button>
      <h3>Import Custom Scale</h3>
      
      <div class="import-instructions">
        <h4>How to format your notation:</h4>
        <ul>
          <li>Use the format: <code>D/ A B C E G A</code></li>
          <li>The note before the slash (/) is the ding note (minimum 6 notes)</li>
          <li>The minimum bass ding note is E2</li>
          <li>List all other notes separated by spaces</li>
          <li>Use () for set as a bottom note and [] for set as a top note in the inner circle around the ding</li>
          <li>Use # for sharps (e.g., F#).</li>
        </ul>
      </div>
      
      <div class="import-form-group">
        <label for="scale-name">Scale Name:</label>
        <input 
          id="scale-name" 
          v-model="importScaleName" 
          placeholder="My Custom Scale"
          class="import-input"
        />
      </div>
      
      <div class="import-form-group">
        <label for="scale-notation">Notation:</label>
        <textarea 
          id="scale-notation"
          v-model="importNotation" 
          placeholder="e.g. D/ A B C E G A"
          rows="3"
          class="import-textarea"
        ></textarea>
      </div>
      
      <div class="import-actions">
        <button 
          class="cancel-button"
          @click="showImportForm = false"
        >
          Cancel
        </button>
        <button 
          class="import-button"
          @click="importCustomScale"
          :disabled="!importNotation || !importScaleName"
        >
          Import Scale
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import useHandpanSelection from '../composables/useHandpanSelection'
import { PhCaretLeft, PhMusicNotesPlus, PhStar } from '@phosphor-icons/vue'
import '../assets/styles/components/HandpanScaleSelection/heliocentric.css'

const {
      // État
      currentStage,
      isTransitioning,
      isLoading,
      error,
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
      
      // Gestion des favoris
      toggleFavorite,
      isFavorite
    } = useHandpanSelection()
    
    // Positions orbitales calculées
    const orbitalPositions = ref([])
    const innerOrbitalPositions = ref([])
    const outerOrbitalPositions = ref([])
    
    // Catégorie et échelle survolées pour affichage dans le cercle central
    const hoveredCategory = ref(null)
    const hoveredScale = ref(null)
    
    // Catégories affichées (incluant les favoris si disponibles)
    const displayedCategories = computed(() => {
      let result = [...categories.value]
      
      // Vérifier si nous avons des favoris et ajouter la catégorie Favoris
      if (favorites.value.length > 0) {
        // Vérifier si la catégorie Favoris existe déjà
        const favoritesCategory = result.find(cat => cat.id === 'Favorites')
        if (!favoritesCategory) {
          result.push({
            id: 'Favorites',
            name: 'Favorites',
            description: 'Your favorite handpan scales collection for quick access'
          })
        }
      }
      
      return result
    })
    
    // Obtenir l'abréviation d'un nom de catégorie
    const getCategoryAbbreviation = (name) => {
      if (!name) return ''
      
      // Si le nom est un seul mot, prendre les 2-3 premières lettres
      if (!name.includes(' ')) {
        return name.substring(0, Math.min(3, name.length)).toUpperCase()
      }
      
      // Si le nom contient plusieurs mots, prendre la première lettre de chaque mot
      const words = name.split(' ')
      return words.map(word => word.charAt(0)).join('').toUpperCase()
    }
    
    // Cette fonction n'est plus nécessaire car les descriptions sont maintenant fournies par l'API
    
    // Afficher les informations d'une catégorie dans le cercle central
    const showCategoryInfo = (category) => {
      hoveredCategory.value = category
    }
    
    // Afficher les informations d'une échelle dans le cercle central
    const showScaleInfo = (scale) => {
      hoveredScale.value = scale
    }
    
    // Cacher les informations d'une échelle
    const hideScaleInfo = () => {
      hoveredScale.value = null
    }
    
    // Cacher les informations de catégorie
    const hideCategoryInfo = () => {
      hoveredCategory.value = null
    }
    
    // Obtenir l'abréviation d'une échelle en utilisant son ID
    const getScaleAbbreviation = (scale) => {
      if (!scale || !scale.id) return ''
      
      const id = scale.id
      
      // Si l'ID est court (moins de 5 caractères), le retourner tel quel
      if (id.length <= 4) return id
      
      // Prendre les 3 premières lettres
      let abbreviation = id.substring(0, 3)
      
      // Ajouter toutes les lettres majuscules de l'ID (sauf celles déjà incluses dans les 3 premières lettres)
      const capsLetters = id.split('').filter(char => char >= 'A' && char <= 'Z')
      
      // Ajouter uniquement les lettres majuscules qui ne sont pas déjà dans l'abréviation
      for (const char of capsLetters) {
        if (!abbreviation.includes(char)) {
          abbreviation += char
        }
      }
      
      return abbreviation
    }
    
    // Générer le style pour un élément orbital avec dégradé de contraste
    const getItemStyle = (scale, index) => {
      // Mapper les catégories aux couleurs hexadécimales
      const categoryColors = {
        'Modal': '#FF6B6B',        // Rouge pastel vif
        'Oriental': '#FF85A1',     // Rose corail
        'PentatonicAsian': '#BA7CF9', // Violet lilas
        'MinorFolk': '#7B68EE',    // Bleu-violet
        'Mystical': '#5DA9E9',     // Bleu azur
        'Ethnic': '#66E0E0',       // Turquoise
        'Experimental': '#6BD490', // Vert menthe
        'Import': '#FFD166',       // Jaune ambre
        'Favorites': '#FF9A3C'     // Orange pastel
      }
      
      // Obtenir la catégorie de l'échelle
      let category = ''
      if (typeof scale.category === 'string') {
        category = scale.category
      } else if (scale.category && scale.category.id) {
        category = scale.category.id
      }
      
      // Trouver la couleur correspondante
      let baseColor = '#CCCCCC' // Couleur par défaut
      
      // Essayer avec la catégorie exacte
      if (categoryColors[category]) {
        baseColor = categoryColors[category]
      } else {
        // Essayer de trouver une correspondance partielle
        for (const [key, color] of Object.entries(categoryColors)) {
          if (category.includes(key)) {
            baseColor = color
            break
          }
        }
      }
      
      // Calculer la variation de luminosité
      const total = filteredScales.value.length
      
      // Le premier élément est 20% plus clair, puis on devient progressivement plus sombre
      // Variation de +20% de blanc à -40% (plus sombre) pour un contraste plus prononcé
      const variationFactor = 1.2 - (0.6 * index / (total - 1 || 1))
      
      // Convertir la couleur hexadécimale en composantes RGB
      const r = parseInt(baseColor.slice(1, 3), 16)
      const g = parseInt(baseColor.slice(3, 5), 16)
      const b = parseInt(baseColor.slice(5, 7), 16)
      
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
      const backgroundColor = `#${newR.toString(16).padStart(2, '0')}${newG.toString(16).padStart(2, '0')}${newB.toString(16).padStart(2, '0')}`
      
      // Utiliser la même couleur pour la bordure que pour le fond
      const borderColor = backgroundColor;
      
      // Créer une couleur d'ombre basée sur la couleur de la catégorie (60% de la couleur, 40% transparent)
      const shadowR = Math.round(r * 0.6)
      const shadowG = Math.round(g * 0.6)
      const shadowB = Math.round(b * 0.6)
      const shadowColor = `rgba(${shadowR}, ${shadowG}, ${shadowB}, 0.6)`
      
      // Retourner l'objet de style complet
      return {
        backgroundColor,
        borderColor,
        boxShadow: `0 8px 20px ${shadowColor}`,
        transform: orbitalPositions.value[index] ? `translate(${orbitalPositions.value[index].x}px, ${orbitalPositions.value[index].y}px)` : 'translate(0, 0)'
      }
    }
    
    // Fonction pour mettre à jour les positions orbitales
    const updateOrbitalPositions = () => {
      // Déterminer quelle collection utiliser en fonction de l'étape actuelle
      let items = []
      let radius = 250 // Rayon par défaut
      
      if (currentStage.value === 2) {
        // Utiliser une copie des valeurs de displayedCategories pour éviter l'erreur "computed value is readonly"
        items = [...(displayedCategories.value || [])]
        radius = 230 // Légèrement réduit pour s'adapter à la hauteur réduite
        orbitalPositions.value = calculateOrbitalPositions(items, radius)
      } else if (currentStage.value === 3) {
        items = filteredScales.value || []
        radius = 230
        orbitalPositions.value = calculateOrbitalPositions(items, radius)
      } else if (currentStage.value === 4) {
        // Pour l'étape 4, calculer deux orbites distinctes
        const innerItems = innerDings.value || []
        const outerItems = outerDings.value || []
        
        // Ajuster les rayons des orbites en fonction de la taille de l'écran
        let innerRadius = 150;
        let outerRadius = 250;
        
        // Réduire les rayons pour les écrans étroits
        if (window.innerWidth < 560) {
          innerRadius = 120;
          outerRadius = 200;
        }
        if (window.innerWidth < 480) {
          innerRadius = 100;
          outerRadius = 170;
        }
        if (window.innerWidth < 360) {
          innerRadius = 80;
          outerRadius = 140;
        }
        
        // Réduire davantage si la hauteur est également contrainte
        if (window.innerHeight < 650) {
          innerRadius = innerRadius * 0.9;
          outerRadius = outerRadius * 0.9;
        }
        
        // Orbite intérieure avec un rayon ajusté
        innerOrbitalPositions.value = calculateOrbitalPositions(innerItems, innerRadius)
        
        // Orbite extérieure avec un rayon ajusté et un décalage pour aligner avec l'orbite intérieure
        // Calculer l'angle de décalage basé sur le nombre d'éléments dans l'orbite intérieure
        const offsetAngle = innerItems.length > 0 ? (2 * Math.PI / innerItems.length) : 0
        
        // Calculer les positions avec un décalage
        const outerPositions = []
        const outerCount = outerItems.length || 0
        
        for (let i = 0; i < outerCount; i++) {
          // Calculer l'angle en radians avec le décalage pour aligner avec le dernier élément de l'orbite intérieure
          const angle = (i * 2 * Math.PI / outerCount) - Math.PI/2 - offsetAngle
          
          // Calculer les coordonnées x et y avec le rayon ajusté
          const x = Math.cos(angle) * outerRadius
          const y = Math.sin(angle) * outerRadius
          
          outerPositions.push({ x, y, angle })
        }
        
        outerOrbitalPositions.value = outerPositions
        
        // Maintenir la compatibilité avec le reste du code
        orbitalPositions.value = calculateOrbitalPositions(dings.value || [], radius)
      } else if (currentStage.value === 5) {
        items = noteOptions.value || []
        radius = 230
        orbitalPositions.value = calculateOrbitalPositions(items, radius)
      }
      
      //console.log(`Updating orbital positions for stage ${currentStage.value}`)
    }
    
    // Computed properties pour les orbites intérieure et extérieure
    const innerDings = computed(() => {
      return dings.value.slice(0, 8) // Les 8 premiers éléments
    })

    const outerDings = computed(() => {
      return dings.value.slice(8) // Les éléments restants
    })

    // Recalculer les positions orbitales lorsque les données changent
    watch([categories, dings, noteOptions], () => {
      updateOrbitalPositions()
    })
    
    // Surveiller spécifiquement les échelles filtrées pour s'assurer que les favoris s'affichent correctement
    watch(filteredScales, (newScales) => {
      //console.log('filteredScales changed, count:', newScales.length)
      if (currentStage.value === 3) {
        //console.log('Updating orbital positions for stage 3 with new filtered scales')
        updateOrbitalPositions()
      }
    }, { deep: true })

    // Recalculer les positions lorsque l'étape change
    watch(currentStage, () => {
      // Attendre que le DOM soit mis à jour et que les données soient chargées
      setTimeout(() => {
        updateOrbitalPositions()
      }, 50)
      
      // Recalculer à nouveau après un délai plus long pour s'assurer que tout est chargé
      setTimeout(() => {
        updateOrbitalPositions()
      }, 300)
    })
    
    // Initialiser les données au chargement du composant
    onMounted(() => {
      // Charger les catégories pour qu'elles soient prêtes quand l'utilisateur passe à l'étape 2
      fetchCategories()
      fetchScales()
      fetchDings()
      
      // Initialiser les positions orbitales pour l'étape 1
      orbitalPositions.value = calculateOrbitalPositions([], 230)
      
      // Recalculer les positions après le rendu initial
      nextTick(() => {
        updateOrbitalPositions()
      })
      
      // Recalculer à nouveau après un délai pour s'assurer que tout est chargé
      setTimeout(() => {
        updateOrbitalPositions()
      }, 100)
      
      // Recalculer une dernière fois après un délai plus long
      setTimeout(() => {
        updateOrbitalPositions()
      }, 800)
    })
    
    // Fin des hooks et des fonctions
</script>

<style>
/* Les styles sont importés depuis le fichier CSS externe */
</style>
