import { ref, computed } from 'vue';

/**
 * Composable for converting score data into timed note events
 * Transforms musical notation into a timeline of events for playback
 *
 * Score data format from backend (PostgreSQL JSONB):
 * {
 *   parts: [{
 *     id: number,
 *     name: string,
 *     measures: [{
 *       id: number,
 *       time_signature: [beats, beatType] | null,  // e.g. [4, 4]
 *       chords: [
 *         [{ pitch: 60, duration: "Quarter", note_type: "Normal", hand: "Right" }]
 *       ]
 *     }]
 *   }]
 * }
 */
export default function useNoteScheduler() {
  // Store scheduled note events
  const scheduledEvents = ref([]);

  // Score metadata
  const scoreDuration = ref(0);
  const tempo = ref(120);
  const timeSignature = ref({ beats: 4, beatType: 4 });

  /**
   * Convert MIDI pitch to note name (e.g., 60 -> "C4")
   * @param {number} pitch - MIDI pitch value (0-127)
   * @returns {string} Note name with octave
   */
  const midiPitchToNoteName = (pitch) => {
    const noteNames = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    const octave = Math.floor(pitch / 12) - 1;
    const noteIndex = pitch % 12;
    return `${noteNames[noteIndex]}${octave}`;
  };

  /**
   * Find the handpan note index that matches a given MIDI pitch
   * @param {number} pitch - MIDI pitch to match
   * @param {Array} handpanNotes - Array of handpan note objects from useHandpanDisplay
   * @returns {number} Index of matching note, or -1 if not found
   */
  const findHandpanNoteIndex = (pitch, handpanNotes) => {
    if (!handpanNotes || !Array.isArray(handpanNotes)) return -1;

    // First try exact match with calculated_pitch
    const exactIndex = handpanNotes.findIndex(note =>
      note.calculated_pitch === pitch
    );
    if (exactIndex !== -1) return exactIndex;

    // Try matching by note name (ignoring octave for flexibility)
    const noteName = midiPitchToNoteName(pitch);
    const noteNameWithoutOctave = noteName.replace(/[0-9]/g, '');
    const nameMatchIndex = handpanNotes.findIndex(note => {
      const handpanNoteName = (note.note || note.calculated_note || '').replace(/[0-9]/g, '');
      return handpanNoteName === noteNameWithoutOctave;
    });

    return nameMatchIndex;
  };

  /**
   * Calculate absolute time in milliseconds for a note position
   * @param {number} measureId - 1-based measure number
   * @param {number} beatPosition - Position within measure in beats (0-based)
   * @param {number} bpm - Tempo in beats per minute
   * @param {Object} timeSig - Time signature { beats, beatType }
   * @returns {number} Absolute time in milliseconds
   */
  const calculateAbsoluteTime = (measureId, beatPosition, bpm, timeSig) => {
    const msPerBeat = 60000 / bpm;
    const beatsPerMeasure = timeSig.beats;

    // Calculate measure start time
    const measureStart = (measureId - 1) * beatsPerMeasure * msPerBeat;

    // Add beat offset within measure
    const beatOffset = beatPosition * msPerBeat;

    return measureStart + beatOffset;
  };

  /**
   * Convert note duration enum to beats
   * Backend uses capitalized enums: "Whole", "Half", "Quarter", etc.
   * @param {string} duration - Duration type from backend
   * @returns {number} Duration in beats
   */
  const durationToBeats = (duration) => {
    // Handle both capitalized (from backend) and lowercase formats
    const durationLower = (duration || 'Quarter').toLowerCase();

    const durationMap = {
      'whole': 4.0,
      'half': 2.0,
      'quarter': 1.0,
      'eighth': 0.5,
      'sixteenth': 0.25,
      'thirtysecond': 0.125,
      'sixtyfourth': 0.0625
    };

    return durationMap[durationLower] || 1.0;
  };

  /**
   * Convert note duration to milliseconds
   * @param {string} duration - Duration type
   * @param {number} bpm - Tempo in beats per minute
   * @returns {number} Duration in milliseconds
   */
  const durationToMs = (duration, bpm) => {
    const msPerBeat = 60000 / bpm;
    const beats = durationToBeats(duration);
    return beats * msPerBeat;
  };

  /**
   * Generate a unique ID for a note event
   * @param {number} measureId - Measure number
   * @param {number} chordIndex - Chord index within measure
   * @param {number} noteIndex - Note index within chord
   * @returns {string} Unique event ID
   */
  const generateEventId = (measureId, chordIndex, noteIndex) => {
    return `m${measureId}-c${chordIndex}-n${noteIndex}`;
  };

  /**
   * Convert note type enum to string name
   * Backend uses: "Rest", "Normal", "Ghost", "Dead", "Grace", "Cue", "Slash", "Harmonic"
   * @param {string|number} noteType - Note type from backend
   * @returns {string} Note type name (lowercase)
   */
  const getNoteTypeName = (noteType) => {
    // If it's a number (legacy format), convert
    if (typeof noteType === 'number') {
      const types = {
        0: 'rest',
        1: 'normal',
        2: 'ghost',
        3: 'dead',
        4: 'grace',
        5: 'cue',
        6: 'slash',
        7: 'harmonic'
      };
      return types[noteType] || 'normal';
    }

    // If it's a string (current format), just lowercase it
    return (noteType || 'Normal').toLowerCase();
  };

  /**
   * Convert hand enum to lowercase
   * Backend uses: "Left", "Right"
   * @param {string} hand - Hand from backend
   * @returns {string} "left" or "right"
   */
  const getHandName = (hand) => {
    return (hand || 'Right').toLowerCase();
  };

  /**
   * Schedule all notes from score data
   * Transforms score structure into flat array of timed events
   * @param {Object} scoreData - Score data from backend (score_data field)
   * @param {Array} handpanNotes - Handpan notes for index mapping
   * @param {Object} options - Optional settings { bpm, partIndex }
   * @returns {Array} Array of TimedNoteEvent objects
   */
  const scheduleScore = (scoreData, handpanNotes, options = {}) => {
    if (!scoreData || !scoreData.parts || scoreData.parts.length === 0) {
      console.warn('No score data to schedule');
      scheduledEvents.value = [];
      scoreDuration.value = 0;
      return [];
    }

    // Use provided BPM or default
    const bpm = options.bpm || tempo.value;
    tempo.value = bpm;

    // Select which part to use (default to first)
    const partIndex = options.partIndex || 0;
    const part = scoreData.parts[partIndex];

    if (!part || !part.measures) {
      console.warn('No measures found in selected part');
      scheduledEvents.value = [];
      scoreDuration.value = 0;
      return [];
    }

    const events = [];
    let currentTimeSig = { beats: 4, beatType: 4 };
    let maxTime = 0;

    // Process each measure
    part.measures.forEach((measure) => {
      // Update time signature if changed
      // Backend format: time_signature: [4, 4] or null
      if (measure.time_signature && Array.isArray(measure.time_signature)) {
        currentTimeSig = {
          beats: measure.time_signature[0],
          beatType: measure.time_signature[1]
        };
        timeSignature.value = currentTimeSig;
      }

      // Process each chord in the measure
      if (!measure.chords || !Array.isArray(measure.chords)) return;

      // Calculate total duration of all chords to distribute them across the measure
      let totalChordBeats = 0;
      measure.chords.forEach((chord) => {
        if (Array.isArray(chord) && chord.length > 0) {
          // Use the duration of the first note in the chord
          totalChordBeats += durationToBeats(chord[0].duration);
        }
      });

      // Track current beat position within the measure
      let currentBeatPosition = 0;

      measure.chords.forEach((chord, chordIndex) => {
        // Skip empty chords
        if (!Array.isArray(chord) || chord.length === 0) return;

        // Get the duration of this chord (from first note)
        const chordDuration = durationToBeats(chord[0].duration);

        // Process each note in the chord
        chord.forEach((note, noteIndex) => {
          // Skip rest notes
          const noteTypeName = getNoteTypeName(note.note_type);
          if (noteTypeName === 'rest') return;

          const absoluteTime = calculateAbsoluteTime(
            measure.id,
            currentBeatPosition,
            bpm,
            currentTimeSig
          );

          const duration = durationToMs(note.duration, bpm);
          const pitch = note.pitch;

          // Create timed note event
          const event = {
            id: generateEventId(measure.id, chordIndex, noteIndex),
            absoluteTime,
            duration,
            pitch,
            handpanNoteIndex: findHandpanNoteIndex(pitch, handpanNotes),
            hand: getHandName(note.hand),
            noteType: noteTypeName,
            measureId: measure.id,
            beatPosition: currentBeatPosition
          };

          events.push(event);

          // Track max time for duration
          const endTime = absoluteTime + duration;
          if (endTime > maxTime) maxTime = endTime;
        });

        // Advance beat position by chord duration
        currentBeatPosition += chordDuration;
      });
    });

    // Sort events by time
    events.sort((a, b) => a.absoluteTime - b.absoluteTime);

    scheduledEvents.value = events;
    scoreDuration.value = maxTime;

    console.log(`Scheduled ${events.length} note events, duration: ${maxTime}ms`);

    return events;
  };

  /**
   * Get events within a time window (for rendering visible notes)
   * @param {number} currentTime - Current playback time in ms
   * @param {number} leadTime - How far ahead to look (ms)
   * @param {number} trailTime - How far behind to include (ms)
   * @returns {Array} Events within the window
   */
  const getEventsInWindow = (currentTime, leadTime = 2000, trailTime = 200) => {
    const windowStart = currentTime - trailTime;
    const windowEnd = currentTime + leadTime;

    return scheduledEvents.value.filter(event =>
      event.absoluteTime >= windowStart && event.absoluteTime <= windowEnd
    );
  };

  /**
   * Get events at exactly the current time (for triggering audio)
   * @param {number} currentTime - Current playback time in ms
   * @param {number} tolerance - Time tolerance in ms (default 50ms)
   * @returns {Array} Events at current time
   */
  const getEventsAtTime = (currentTime, tolerance = 50) => {
    return scheduledEvents.value.filter(event =>
      Math.abs(event.absoluteTime - currentTime) <= tolerance
    );
  };

  /**
   * Clear all scheduled events
   */
  const clearSchedule = () => {
    scheduledEvents.value = [];
    scoreDuration.value = 0;
  };

  // Computed: total number of events
  const eventCount = computed(() => scheduledEvents.value.length);

  return {
    // State
    scheduledEvents,
    scoreDuration,
    tempo,
    timeSignature,
    eventCount,

    // Methods
    scheduleScore,
    getEventsInWindow,
    getEventsAtTime,
    clearSchedule,
    midiPitchToNoteName,
    findHandpanNoteIndex,
    calculateAbsoluteTime,
    durationToMs,
    durationToBeats
  };
}
