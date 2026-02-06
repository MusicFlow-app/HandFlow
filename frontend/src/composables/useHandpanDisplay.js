import { ref, computed, watch } from 'vue';
import { apiUrl } from '@/services/api';

/**
 * Composable for handling the handpan display logic
 * Only initializes when called at the appropriate stage
 */
export default function useHandpanDisplay(options) {
  // Audio cache to store preloaded sounds
  const audioCache = ref({});
  
  // Store audio elements for cleanup
  const audioElements = [];

  // Create local ref for notes to ensure reactivity
  // Important: We need to unwrap the ref if it's already a ref
  const notesRef = ref(options.notes && options.notes.value ? options.notes.value : options.notes);
  
  // Watch for changes in the original notes ref if it is a ref
  if (options.notes && typeof options.notes === 'object' && options.notes.value !== undefined) {
    watch(() => options.notes.value, (newNotes) => {
      if (newNotes && Array.isArray(newNotes)) {
        console.log('Notes updated from props:', newNotes.length);
        notesRef.value = newNotes;
      }
    }, { immediate: true });
  }
  
  // Create local refs for all options to ensure reactivity
  const selectedNoteCountRef = ref(options.selectedNoteCount);
  console.log('Initial selectedNoteCountRef value:', selectedNoteCountRef.value, 'type:', typeof selectedNoteCountRef.value);
  
  const {
    selectedScale,
    selectedDing,
    currentStage
  } = options;
  
  // Local state
  const activeNote = ref(null);
  const isPlaying = ref(false);
  const audio = ref(null);
  const errorDuringRender = ref(false);
  const componentActivated = ref(false);
  
  // Convert selectedDing to ref if it's not already one
  const dingRef = ref(selectedDing);
  console.log('Initial dingRef value:', dingRef.value, 'type:', typeof dingRef.value);
  
  // Check if we have all required data
  const isReadyToDisplay = computed(() => {
    // MOST CRITICAL CHECK FIRST: Are we in the correct stage?
    const isCorrectStage = currentStage.value >= 6;
    if (!isCorrectStage) {
      console.log('HandpanDisplay not ready: incorrect stage:', currentStage.value);
      return false;
    }
    
    // Next, check if we have scale and ding selected
    const hasScale = !!selectedScale.value;
    const hasDing = !!dingRef.value;
    
    if (!hasScale || !hasDing) {
      console.log('HandpanDisplay not ready: missing scale or ding');
      return false;
    }
    
    // Check if we have notes - more robust check
    const hasNotes = notesRef.value && Array.isArray(notesRef.value) && notesRef.value.length > 0;
    
    // Log the notes for debugging
    console.log('Notes check in isReadyToDisplay:', {
      notesRef: notesRef.value,
      isArray: Array.isArray(notesRef.value),
      length: notesRef.value?.length || 0
    });
    
    // If we don't have notes yet but we're at stage 6 with scale and ding, log an error
    if (!hasNotes && isCorrectStage && hasScale && hasDing) {
      console.log('Display stage correct but notes are empty - this should not happen');
      // No fallback to mock notes - we expect real data from the API
      errorDuringRender.value = true;
    }
    
    const hasNoteCount = selectedNoteCountRef.value !== undefined && selectedNoteCountRef.value > 0;
    console.log('Note count check:', { selectedNoteCount: selectedNoteCountRef.value, hasNoteCount });
    
    // Detailed logging for debugging
    console.log('HandpanDisplay readiness check:', {
      isCorrectStage,
      hasScale,
      hasDing,
      hasNotes,
      notesArray: notesRef.value,
      notesLength: notesRef.value?.length || 0,
      hasNoteCount,
      selectedNoteCount: selectedNoteCountRef.value,
      result: isCorrectStage && hasScale && hasDing && (hasNotes || hasNoteCount)
    });
    
    // Relax the condition slightly - if we have scale, ding and we're at stage 6,
    // we can proceed even without notes (mock data will be used)
    return isCorrectStage && hasScale && hasDing && (hasNotes || hasNoteCount);
  });
  
  // Computed properties for convenience
  const scale = computed(() => selectedScale.value);
  
  // Ensure ding is properly extracted as a string value
  const ding = computed(() => {
    const rawDing = dingRef.value;
    console.log('Computing ding from rawDing:', rawDing, 'type:', typeof rawDing);
    
    // If it's already a string, return it
    if (typeof rawDing === 'string') return rawDing;
    
    // If it's an object, try to extract the note value
    if (typeof rawDing === 'object' && rawDing !== null) {
      // Try different properties that might contain the note
      if (rawDing.note) return rawDing.note;
      if (rawDing.value) return rawDing.value;
      if (rawDing.calculated_note) return rawDing.calculated_note;
    }
    
    // Fallback to a default or the string representation
    return rawDing ? String(rawDing) : 'D';
  });
  
  // Create a data object for the ding note
  const dingData = computed(() => {
    // Find the note with ID 0 or note_index 0 in the notes array
    if (notesRef.value && Array.isArray(notesRef.value) && notesRef.value.length > 0) {
      const dingNote = notesRef.value.find(note => 
        note && (note.id === 0 || note.note_index === 0)
      );
      
      if (dingNote) {
        console.log('Using API note with ID 0 as ding:', dingNote);
        return {
          ...dingNote,
          note_index: 'ding', // Keep the special note_index for UI identification
        };
      }
    }
    
    // Fallback to the current implementation if no note with ID 0 is found
    return {
      note_index: 'ding',
      position: "Top",
      note: ding.value,
      calculated_note: ding.value,
      calculated_pitch: 40 // Default pitch for the ding
    };
  });
  
  // Selected notes (limited by count and prioritized by position)
  const selectedNotes = computed(() => {
    // More robust handling of notes data
    if (!notesRef.value || !Array.isArray(notesRef.value)) {
      console.warn('Notes is not an array or is undefined:', notesRef.value);
      return [];
    }
    
    // Always use the selected note count if available
    // Make sure we're using the exact value passed in props
    const count = selectedNoteCountRef.value;
    console.log('Using note count:', count, 'from available notes:', notesRef.value.length, 'selectedNoteCount:', selectedNoteCountRef.value);
    
    // Filter out the note with ID 0 or note_index 0 (ding note)
    // The ding note is already displayed separately in the center
    const filteredNotes = notesRef.value.filter(note => {
      if (!note) return false;
      
      // Skip the note with ID 0 or note_index 0
      if (note.id === 0 || note.note_index === 0) {
        console.log('Filtering out ding note (ID 0) from regular notes:', note);
        return false;
      }
      
      return true;
    });
    
    // Categorize notes by position
    const topNotesList = [];
    const innerNotesList = [];
    const bottomNotesList = [];
    
    // First pass to categorize all notes
    filteredNotes.forEach(note => {
      if (!note) return; // Skip null or undefined notes
      
      if (note.position === 'Top' || !note.position) {
        topNotesList.push(note);
      } else if (note.position === 'Inner') {
        innerNotesList.push(note);
      } else if (note.position === 'Bottom') {
        bottomNotesList.push(note);
      }
    });
    
    console.log('Categorized notes:', {
      top: topNotesList.length,
      inner: innerNotesList.length,
      bottom: bottomNotesList.length
    });
    
    // Combine notes in priority order: top, inner, bottom
    const prioritizedNotes = [
      ...topNotesList,
      ...innerNotesList,
      ...bottomNotesList
    ];
    
    // ALWAYS limit the notes to the selectedNoteCount - this is essential
    // Force using the number directly as an integer
    const noteCountInt = parseInt(selectedNoteCountRef.value, 10);
    console.log('Using note count (parsed):', noteCountInt, 'type:', typeof noteCountInt);
    
    // Only limit if we have a valid number
    let limitedNotes = [];
    if (!isNaN(noteCountInt) && noteCountInt > 0) {
      limitedNotes = prioritizedNotes.slice(0, noteCountInt);
    } else {
      // Fallback to default behavior if no valid noteCount
      limitedNotes = prioritizedNotes;
    }
    
    console.log('FINAL NOTE COUNT: limited to', limitedNotes.length, 'from total:', prioritizedNotes.length, 'with requested limit:', selectedNoteCountRef.value);
    
    return limitedNotes;
  });
  
  // Top, inner, and bottom notes from the prioritized selectedNotes
  const topNotes = computed(() => {
    // Debug the selected notes
    console.log('Computing topNotes from selectedNotes:', selectedNotes.value);
    
    // Handle case where selectedNotes is not properly reactive or is empty
    if (!selectedNotes.value || !Array.isArray(selectedNotes.value) || selectedNotes.value.length === 0) {
      console.warn('No valid selectedNotes for topNotes calculation');
      return [];
    }
    
    // Filter notes for top position
    const result = selectedNotes.value.filter(note => {
      // Skip null or undefined notes
      if (!note) return false;
      
      // Keep notes that explicitly have "Top" position
      if (note.position && note.position === "Top") return true;
      
      // For notes without position property or with undefined value,
      // default to showing them on top
      if (!note.position) return true;
      
      // For any other positions, filter out
      return false;
    });
    
    console.log('Filtered topNotes:', result);
    return result;
  });
  
  // Inner notes
  const innerNotes = computed(() => {
    // Handle case where selectedNotes is not properly reactive or is empty
    if (!selectedNotes.value || !Array.isArray(selectedNotes.value) || selectedNotes.value.length === 0) {
      console.warn('No valid selectedNotes for innerNotes calculation');
      return [];
    }
    
    // Filter notes for inner position
    const result = selectedNotes.value.filter(note => {
      // Skip null or undefined notes
      if (!note) return false;
      
      // Only include notes that explicitly have "Inner" position
      return note.position && note.position === "Inner";
    });
    
    console.log('Filtered innerNotes:', result);
    return result;
  });
  
  const bottomNotes = computed(() => {
    // Debug the selected notes
    console.log('Computing bottomNotes from selectedNotes:', selectedNotes.value);
    
    // Handle case where selectedNotes is not properly reactive or is empty
    if (!selectedNotes.value || !Array.isArray(selectedNotes.value) || selectedNotes.value.length === 0) {
      console.warn('No valid selectedNotes for bottomNotes calculation');
      return [];
    }
    
    // Filter notes for bottom position
    const result = selectedNotes.value.filter(note => {
      // Skip null or undefined notes
      if (!note) return false;
      
      // Only include notes that explicitly have "Bottom" position
      return note.position && note.position === "Bottom";
    });
    
    console.log('Filtered bottomNotes:', result);
    return result;
  });
  
  // Color utilities
  const getCategoryColor = (category) => {
    // Default colors for each category
    const colors = {
      'Traditional': '#5D4037',  // Brown
      'Ethnic': '#7B1FA2',      // Purple
      'Modern': '#1976D2',      // Blue
      'Experimental': '#388E3C', // Green
      'Melodic': '#D32F2F',     // Red
      'Custom': '#F57C00'       // Orange
    };
    
    return colors[category] || '#607D8B'; // Default gray if category not found
  };

  // Darken a color by percentage
  const darkenColor = (color, percent) => {
    if (!color) return '#607D8B';
    
    // Handle hex colors
    if (color.startsWith('#')) {
      let r = parseInt(color.substr(1, 2), 16);
      let g = parseInt(color.substr(3, 2), 16);
      let b = parseInt(color.substr(5, 2), 16);
      
      r = Math.floor(r * (1 - percent / 100));
      g = Math.floor(g * (1 - percent / 100));
      b = Math.floor(b * (1 - percent / 100));
      
      r = r < 0 ? 0 : r;
      g = g < 0 ? 0 : g;
      b = b < 0 ? 0 : b;
      
      return `#${r.toString(16).padStart(2, '0')}${g.toString(16).padStart(2, '0')}${b.toString(16).padStart(2, '0')}`;
    }
    
    // For other formats, return original
    return color;
  };
  
  // Convert a note like 'A4', 'C#3' to a numeric value representing pitch
  const noteToPitchValue = (noteStr) => {
    if (!noteStr) return 0;
    
    // Extract note and octave - note can have '#' or 'b'
    const match = noteStr.match(/([A-G][#b]?)([0-9])/);
    if (!match) return 0;
    
    const [, note, octave] = match;
    
    // Note values (C=0, C#=1, D=2, etc.)
    const noteValues = {
      'C': 0, 'C#': 1, 'Db': 1,
      'D': 2, 'D#': 3, 'Eb': 3,
      'E': 4, 'F': 5, 'F#': 6, 'Gb': 6,
      'G': 7, 'G#': 8, 'Ab': 8, 
      'A': 9, 'A#': 10, 'Bb': 10,
      'B': 11
    };
    
    // Calculate pitch value: octave * 12 + note value
    // Higher value = higher pitch
    return parseInt(octave) * 12 + noteValues[note];
  };
  
  // Cache the sorted pitches of all notes to use when calculating scale factors
  let sortedNoteCache = null;
  const getSortedNotes = () => {
    // Use cached result if available
    if (sortedNoteCache) return sortedNoteCache;
    
    // Get all notes including top, inner, and bottom
    const allNotes = selectedNotes.value || [];
    if (allNotes.length === 0) return [];
    
    // Extract and sort pitches
    const notesWithPitches = allNotes.map(note => {
      const noteStr = note.note || note.calculated_note || '';
      let pitch;
      
      // Use calculated_pitch if available
      if (note.calculated_pitch !== undefined) {
        pitch = parseInt(note.calculated_pitch, 10);
      } else {
        pitch = noteToPitchValue(noteStr);
      }
      
      return {
        note: noteStr,
        pitch: pitch,
        originalNote: note // Keep reference to original note object
      };
    });
    
    // Sort by pitch (lowest to highest)
    notesWithPitches.sort((a, b) => a.pitch - b.pitch);
    
    console.log('Sorted notes by pitch:', notesWithPitches);
    sortedNoteCache = notesWithPitches;
    return notesWithPitches;
  };
  
  // Calculate scale factor based on a note's pitch relative to others
  const calculateScaleFactor = (noteObj) => {
    if (!noteObj) return 1.0; // Default scale if no note object
    
    // Use calculated_pitch directly if available, otherwise fallback to note name
    let pitch;
    if (noteObj.calculated_pitch !== undefined) {
      // Use the calculated_pitch directly
      pitch = parseInt(noteObj.calculated_pitch, 10);
      console.log(`Using calculated_pitch for ${noteObj.note}: ${pitch}`);
    } else {
      // Fallback to calculating from note name
      const noteStr = noteObj.note || noteObj.calculated_note || '';
      pitch = noteToPitchValue(noteStr);
      console.log(`Calculated pitch from note name ${noteStr}: ${pitch}`);
    }
    
    // Get all notes sorted by pitch
    const sortedNotes = getSortedNotes();
    if (sortedNotes.length <= 1) return 1.0;
    
    // Find the position of this note in the sorted array by matching pitch
    // or by matching note names if pitch values don't match exactly
    let pitchRank = sortedNotes.findIndex(n => n.pitch === pitch);
    if (pitchRank === -1) {
      // If exact pitch not found, try matching by note name
      const noteStr = noteObj.note || noteObj.calculated_note || '';
      pitchRank = sortedNotes.findIndex(n => n.note === noteStr);
      
      // If still not found, use the nearest pitch as fallback
      if (pitchRank === -1) {
        // Find the closest pitch value
        let minDiff = Infinity;
        sortedNotes.forEach((n, idx) => {
          const diff = Math.abs(n.pitch - pitch);
          if (diff < minDiff) {
            minDiff = diff;
            pitchRank = idx;
          }
        });
      }
    }
    
    // Safety check - default to middle if somehow still not found
    if (pitchRank === -1) pitchRank = Math.floor(sortedNotes.length / 2);
    
    // Define scale range - keep it subtle to avoid overflow
    const maxScale = 1.1;  // For lowest pitch
    const minScale = 0.8;  // For highest pitch
    
    // Calculate step size between each note
    const noteCount = sortedNotes.length;
    const step = (maxScale - minScale) / (noteCount - 1);
    
    // Calculate scale factor based on position in pitch ranking
    // Lower rank (lower pitch) gets larger scale
    const scaleFactor = maxScale - (pitchRank * step);
    
    const noteStr = noteObj.note || noteObj.calculated_note || '';
    console.log(`Note ${noteStr} (pitch ${pitch}) has rank ${pitchRank}/${noteCount-1}, scale: ${scaleFactor.toFixed(2)}`);
    
    return scaleFactor;
  };

  // Calculate position for top notes
  const getTopNoteStyle = (index, total, scaleMultiplier = 1, position = 'top', note = null) => {
    // Safety check
    if (!total || total < 1) return {};
    
    console.log(`Positioning top note ${index} of ${total} notes`);
    
    // Get scale factor based on note pitch if note object is provided
    const scaleFactor = note ? calculateScaleFactor(note) : 1.0;
    
    // Musical handpan placement logic
    // Start at the bottom (6 o'clock, 180°) and alternate left-right as pitch increases
    let angleInDegrees;
    
    // We're implementing a formula that creates a musically balanced layout
    // following traditional handpan design principles:
    // 1. Start at the bottom (180°)
    // 2. Alternate sides (left/right) as we ascend in pitch
    // 3. Create a symmetrical pattern along the vertical axis
    // 4. Remember that the ding note is handled separately in the center
      
    // Calculate how many pairs of notes we have (left-right pairs)
    const pairs = Math.floor(total / 2);
      
    // Calculate step size between notes based on total number
    // We use slightly smaller steps for more notes to maintain spacing
    let angleStep;
      
    if (index === total) {
      // Last note always at the top (12 o'clock)
      angleInDegrees = 0;
    } else {
      // For all other notes, calculate based on index
      // Determine if we're working with an odd or even total
      const isOdd = (total % 2) !== 0;
      
      // For odd totals, we need to adjust the pair number calculation
      // to account for the first note already being at the first increment
      let pairNumber;
      if (isOdd) {
        pairNumber = Math.ceil((index) / 2);
        angleStep = 180 / (pairs+1);
        // For odd totals, adjust the calculation to avoid overlap
        // Determine side (0 = right, 1 = left)
        const side = (index % 2) === 0 ? 0 : 1;
        
        // Calculate angle: start from bottom, move up by steps
        // Side determines left or right placement
        if (side === 0) { // right side
          angleInDegrees = 180 + (pairNumber * angleStep);
        } else { // left side
          angleInDegrees = 180 - (pairNumber * angleStep);
        }
      } else {
        // Standard calculation for even totals
        // Determine side (0 = right, 1 = left)
        angleStep = 180 / pairs;
        pairNumber = Math.ceil((index - 1) / 2);
        
        if (index !== 1) {
          const side = (index % 2) === 0 ? 0 : 1;
          
          // Calculate angle: start from bottom, move up by steps
          // Side determines left or right placement
          if (side === 0) { // right side
            angleInDegrees = 180 + (pairNumber * angleStep);
          } else { // left side
            angleInDegrees = 180 - (pairNumber * angleStep);
          }
        } else {
          angleInDegrees = 180;
        }
      }
    }
      
    // Ensure angle is within 0-359 range
    angleInDegrees = angleInDegrees % 360;
    
    console.log(`Musical note placement: Note ${index}/${total} at ${angleInDegrees.toFixed(1)}° (${Math.round(angleInDegrees/30)} o'clock)`);
    
    // Convert to radians for calculations
    const angleInRadians = angleInDegrees * (Math.PI / 180);
    
    // Radius for standard 8-note configuration - adjusted to fit within the shell
    // Keep notes well inside the 200px shell radius
    const radius = 135;
    
    // Calculate x and y coordinates
    // We use sin for x and -cos for y because:
    // - 0 degrees should be at the top (12 o'clock)
    // - Positive x is to the right
    // - Positive y is down (in DOM coordinates)
    const x = Math.sin(angleInRadians) * radius;
    const y = -Math.cos(angleInRadians) * radius;
    
    console.log(`Top note ${index} position: x=${x.toFixed(2)}, y=${y.toFixed(2)}, angle=${angleInDegrees}°`);
    
    // Get the base color from the scale category
    const baseColor = getCategoryColor(scale.value?.category) || 'var(--secondary-color)';
    const shadowColor = darkenColor(baseColor, 20);
    
    // Calculate rotation angle to point toward center with conditional rotation:
    // - Add 90 degrees if angle > 180 (bottom half of circle)
    // - Subtract 90 degrees if angle <= 180 (top half of circle)
    // This creates a more natural orientation based on position
    const rotationDegrees = angleInDegrees > 180 ? angleInDegrees + 90 : angleInDegrees - 90;
    
    return {
      // Apply dynamic scaling based on total notes
      transform: `translate(${x}px, ${y}px) rotate(${rotationDegrees}deg) scale(${scaleFactor})`,
      boxShadow: `inset 2px 2px 5px rgba(255, 255, 255, 0.4), 
                  inset -2px -2px 5px rgba(0, 0, 0, 0.2), 
                  0 2px 5px rgba(0, 0, 0, 0.2)`,
      transformOrigin: 'center center', // Ensure rotation happens from the center
      position: 'absolute', // Ensure absolute positioning
      zIndex: '5' // Keep notes above the shell background
    };
  };

  // Calculate position for inner notes
  const getInnerNoteStyle = (index, total, scaleMultiplier = 0.7, position = 'inner', note = null) => {
    // Safety check
    if (!total || total < 1) return {};
    
    console.log(`Positioning inner note ${index} of ${total} notes`);
    
    // Get scale factor based on note pitch if note object is provided
    const scaleFactor = note ? calculateScaleFactor(note) : 1.0;
    
    // For inner notes, we distribute them evenly in a smaller circle
    const baseAngle = 360 / total;
    const angleInDegrees = (index - 1) * baseAngle;
    
    // Convert to radians for calculations
    const angleInRadians = angleInDegrees * (Math.PI / 180);
    
    // Radius for inner circle (smaller than the top notes)
    const radius = 70;
    
    // Calculate x and y coordinates
    const x = Math.sin(angleInRadians) * radius;
    const y = -Math.cos(angleInRadians) * radius;
    
    console.log(`Inner note ${index} position: x=${x.toFixed(2)}, y=${y.toFixed(2)}, angle=${angleInDegrees}°`);
    
    // Get the base color from the scale category but make it slightly different
    const baseColor = getCategoryColor(scale.value?.category, true) || 'var(--secondary-color)';
    const shadowColor = darkenColor(baseColor, 20);
    
    // Calculate rotation angle to point toward center with conditional rotation:
    // - Add 90 degrees if angle > 180 (bottom half of circle)
    // - Subtract 90 degrees if angle <= 180 (top half of circle)
    // This creates a more natural orientation based on position
    const rotationDegrees = angleInDegrees > 180 ? angleInDegrees + 90 : angleInDegrees - 90;
    
    return {
      // Apply dynamic scaling based on total notes
      transform: `translate(${x}px, ${y}px) rotate(${rotationDegrees}deg) scale(${scaleFactor})`,
      boxShadow: `inset 2px 2px 5px rgba(255, 255, 255, 0.4), 
                  inset -2px -2px 5px rgba(0, 0, 0, 0.2), 
                  0 2px 5px rgba(0, 0, 0, 0.2)`,
      // Use a slightly different appearance for inner notes
      opacity: '0.92',
      transformOrigin: 'center center', // Ensure rotation happens from the center
      position: 'absolute', // Ensure absolute positioning
      zIndex: '5' // Keep notes above the shell background
    };
  };

  // Calculate position for bottom notes
  const getBottomNoteStyle = (index, total, scaleMultiplier = 1, position = 'bottom', note = null) => {
    // Safety check
    if (!total || total < 1) return {};
    
    console.log(`Positioning bottom note ${index} of ${total} notes`);
    
    // Get scale factor based on note pitch if note object is provided
    const scaleFactor = note ? calculateScaleFactor(note) : 1.0;
    
    // IMPORTANT: The bottom view is like looking at the handpan from underneath
    // This means the layout should mirror the top view, but with angles that make
    // sense when viewed from the bottom - we need to use the same pattern as the top notes
    // but with adjusted angles to account for bottom perspective
    
    // Use the same preferred order as top, but mirror it for bottom perspective
    // For a bottom note at position X, we need to calculate 360 - angle(X) to mirror it
    const topPreferredOrder = [
      180,  // bottom (6 o'clock)
      225,  // bottom right (7:30 o'clock)
      135,  // bottom left (4:30 o'clock)
      270,  // right (9 o'clock)
      90,   // left (3 o'clock)
      315,  // top right (10:30 o'clock)
      45,   // top left (1:30 o'clock)
      0     // top (12 o'clock)
    ];
    
    // Generate bottom order by reversing angles (mirrorAngle = 180 + originalAngle) 180° + x°
    // This creates the correct mirroring effect for the bottom view
    const preferredBottomOrder = topPreferredOrder.map(angle => {
      // The formula for mirroring: if we add 180° to each angle, it will flip it to the opposite side
      // but we need to normalize to keep it in the 0-359 range
      return (angle + 180) % 360;
    });
    
    let angleInDegrees;
    
    // If we have 8 or fewer notes, use the preferred bottom order
    if (total <= 8) {
      // The index is 1-based, so we subtract 1 to get the 0-based index
      angleInDegrees = preferredBottomOrder[index - 1];
      console.log(`Using preferred angle for bottom note ${index}: ${angleInDegrees}° (mirrored from top)`);
    } else {
      // For more than 8 notes, distribute them evenly with mirroring
      const baseAngle = 360 / total;
      // Base angle for top would be (index-1) * baseAngle
      // For bottom, we mirror it by adding 180°
      const topAngle = (index - 1) * baseAngle;
      angleInDegrees = (topAngle + 180) % 360;
      console.log(`Calculated angle for bottom note ${index}: ${angleInDegrees}° (mirrored from ${topAngle}°)`);
    }
    
    // Convert to radians for calculations
    const angleInRadians = angleInDegrees * (Math.PI / 180);
    
    // Use the same radius as top notes for visual consistency
    // Keep notes well inside the shell
    const radius = 130;
    
    // Calculate x and y coordinates
    const x = Math.sin(angleInRadians) * radius;
    const y = -Math.cos(angleInRadians) * radius;
    
    console.log(`Bottom note ${index} position: x=${x.toFixed(2)}, y=${y.toFixed(2)}, angle=${angleInDegrees}°`);
    
    // Get the base color from the scale category, but use a slightly different color for bottom notes
    const baseColor = getCategoryColor(scale.value?.category) || 'var(--secondary-color)';
    const shadowColor = darkenColor(baseColor, 20);
    
    // Calculate rotation angle to point toward center with conditional rotation:
    // - Add 90 degrees if angle > 180 (bottom half of circle)
    // - Subtract 90 degrees if angle <= 180 (top half of circle)
    // This creates a more natural orientation based on position
    const rotationDegrees = angleInDegrees > 180 ? angleInDegrees + 90 : angleInDegrees - 90;
    
    return {
      transform: `translate(${x}px, ${y}px) rotate(${rotationDegrees}deg) scale(${scaleFactor})`,
      boxShadow: `inset 2px 2px 5px rgba(255, 255, 255, 0.4), 
                  inset -2px -2px 5px rgba(0, 0, 0, 0.2), 
                  0 2px 5px rgba(0, 0, 0, 0.2)`,
      // Use a slightly different appearance for bottom notes
      opacity: '0.95',
      transformOrigin: 'center center', // Ensure rotation happens from the center
      position: 'absolute', // Ensure absolute positioning
      zIndex: '5' // Keep notes above the shell background
    };
  };

  // Main function that delegates to the appropriate positioning function
  const getNoteStyle = (index, total, scaleMultiplier = 1, noteType = 'top', note = null) => {
    // Safety check
    if (!total || total < 1) return {};
    
    console.log(`Delegating positioning for ${noteType} note ${index} of ${total} notes`);
    
    // Call the appropriate function based on the note type
    // Pass all parameters including the note object
    switch (noteType.toLowerCase()) {
      case 'inner':
        return getInnerNoteStyle(index, total, scaleMultiplier, noteType, note);
      case 'bottom':
        return getBottomNoteStyle(index, total, scaleMultiplier, noteType, note);
      case 'top':
      default:
        return getTopNoteStyle(index, total, scaleMultiplier, noteType, note);
    }
  };

  // Note to MIDI pitch mapping for audio playback
  const noteToPitchMap = {
    'C3': 48, 'C#3': 49, 'D3': 50, 'D#3': 51, 'E3': 52, 'F3': 53, 
    'F#3': 54, 'G3': 55, 'G#3': 56, 'A3': 57, 'A#3': 58, 'B3': 59,
    'C4': 60, 'C#4': 61, 'D4': 62, 'D#4': 63, 'E4': 64, 'F4': 65, 
    'F#4': 66, 'G4': 67, 'G#4': 68, 'A4': 69, 'A#4': 70, 'B4': 71,
    'C5': 72, 'C#5': 73, 'D5': 74, 'D#5': 75, 'E5': 76, 'F5': 77,
    'F#5': 78, 'G5': 79, 'G#5': 80, 'A5': 81, 'A#5': 82, 'B5': 83
  };

  // Preload all note sounds for handpan display
  const preloadAllNoteSounds = () => {
    console.log('Preloading all note sounds for handpan display...');
    
    try {
      // Get all selected notes to preload
      const allNotes = selectedNotes.value || [];
      
      // Special sounds to preload
      const specialSounds = [
        { key: 'slack', url: apiUrl('/api/audio/slack') },
        { key: 'gu', url: apiUrl('/api/audio/gu') }
      ];
      
      // Process all selected notes
      allNotes.forEach(note => {
        // Get the pitch with fallback mechanisms
        let pitch = note.calculated_pitch;
        
        // If there's no calculated pitch, try to determine based on note name
        if (!pitch && (note.note || note.calculated_note)) {
          const noteStr = note.note || note.calculated_note;
          pitch = noteToPitchMap[noteStr];
        }
        
        // If we still don't have a pitch, skip this note
        if (!pitch) {
          console.warn('No pitch information available for note:', note);
          return;
        }
        
        // Create and preload the audio
        if (!audioCache.value[pitch]) {
          const audio = new Audio(apiUrl(`/api/audio/${pitch}`));
          audio.preload = 'auto';
          
          // Force preloading
          audio.load();
          
          // Store in cache
          audioCache.value[pitch] = audio;
          console.log(`Preloaded note sound for pitch ${pitch}`);
        }
      });
      
      // Preload special sounds (slack and gu)
      specialSounds.forEach(({key, url}) => {
        const audio = new Audio(url);
        audio.preload = 'auto';
        audio.load();
        audioCache.value[key] = audio;
        console.log(`Preloaded special sound: ${key}`);
      });
      
      console.log(`Successfully preloaded ${Object.keys(audioCache.value).length} sounds for handpan display.`);
    } catch (error) {
      console.error('Error preloading note sounds:', error);
    }
  };
  
  // Watch for changes in the stage to trigger preloading
  watch(() => currentStage.value, (newStage) => {
    if (newStage >= 5 && selectedNotes.value && selectedNotes.value.length > 0) {
      console.log('Stage changed to', newStage, '- preloading note sounds...');
      preloadAllNoteSounds();
    }
  }, { immediate: true });

  // Audio functions
  const playNote = (note) => {
    try {
      if (!note) return;
      
      // Set the active note for visual feedback
      activeNote.value = note.note_index;
      
      // Get the pitch - with fallback mechanisms
      let pitch = note.calculated_pitch;
      
      // If there's no calculated pitch, try to determine based on note name
      if (!pitch && (note.note || note.calculated_note)) {
        const noteStr = note.note || note.calculated_note;
        pitch = noteToPitchMap[noteStr];
      }
      
      // If we still don't have a pitch, use a default
      if (!pitch) {
        console.warn('No pitch information available for note:', note);
        pitch = 60; // Default to middle C
      }
      
      // Use cached note sound if available, otherwise create new
      let audioElement;
      
      if (audioCache.value[pitch]) {
        // Use cached audio - clone it to allow multiple simultaneous plays
        audioElement = audioCache.value[pitch].cloneNode();
        console.log(`Using cached note sound for pitch ${pitch}`);
      } else {
        // Create a new audio element as fallback
        audioElement = new Audio(apiUrl(`/api/audio/${pitch}`));
        console.log(`Creating new note sound for pitch ${pitch} (not in cache)`);
      }
      
      // Play the audio
      audioElement.play()
        .catch(error => {
          console.error('Error playing audio:', error);
        });
      
      // Store reference for cleanup
      audioElements.push(audioElement);
      audio.value = audioElement;
      
      // Reset active note after a short delay
      setTimeout(() => {
        if (activeNote.value === note.note_index) {
          activeNote.value = null;
        }
      }, 500);
    } catch (error) {
      console.error('Error playing note:', error);
      activeNote.value = null;
    }
  };
  
  // Play sound for slack (shell rim)
  const playSlack = () => {
    try {
      // Use cached slack sound if available
      let audioElement;
      
      if (audioCache.value['slack']) {
        audioElement = audioCache.value['slack'].cloneNode();
        console.log('Using cached slack sound');
      } else {
        audioElement = new Audio(apiUrl('/api/audio/slack'));
        console.log('Creating new slack sound (not in cache)');
      }
      
      audioElement.play()
        .catch(error => {
          console.error('Error playing slack sound:', error);
        });
      audioElements.push(audioElement);
    } catch (error) {
      console.error('Error playing slack:', error);
    }
  };
  
  // Play sound for Gu (bottom center)
  const playGu = () => {
    try {
      // Use cached gu sound if available
      let audioElement;
      
      if (audioCache.value['gu']) {
        audioElement = audioCache.value['gu'].cloneNode();
        console.log('Using cached gu sound');
      } else {
        audioElement = new Audio(apiUrl('/api/audio/gu'));
        console.log('Creating new gu sound (not in cache)');
      }
      
      audioElement.play()
        .catch(error => {
          console.error('Error playing Gu sound:', error);
        });
      audioElements.push(audioElement);
    } catch (error) {
      console.error('Error playing Gu:', error);
    }
  };

  // Empty function that does nothing - mock notes are no longer needed
  const forceLoadMockNotes = () => {
    console.log('Mock notes functionality has been removed');
    return notesRef.value;
  };
  
  // Cleanup function
  const cleanup = () => {
    // Stop any playing audio
    if (audio.value) {
      try {
        audio.value.pause();
        audio.value.src = '';
      } catch (e) {
        console.error('Error stopping audio:', e);
      }
      audio.value = null;
    }
    
    // Stop any playing audio in the audioElements array
    audioElements.forEach(audio => {
      try {
        audio.pause();
        audio.src = '';
      } catch (e) {
        console.error('Error cleaning up audio:', e);
      }
    });
    
    // Clean up cached audio
    Object.values(audioCache.value).forEach(audio => {
      try {
        audio.pause();
        audio.src = '';
      } catch (e) {
        console.error('Error cleaning up cached audio:', e);
      }
    });
    
    // Clear the cache
    audioCache.value = {};
    
    activeNote.value = null;
    isPlaying.value = false;
  };

  return {
    // State
    errorDuringRender,
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
    notes: notesRef,
    
    // Utility functions
    getCategoryColor,
    darkenColor,
    getNoteStyle,
    
    // Audio functions
    playNote,
    playSlack,
    playGu,
    
    // Preloading function
    preloadAllNoteSounds,
    
    // Cleanup
    cleanup
  };
}
