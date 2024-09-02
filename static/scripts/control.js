document.addEventListener('DOMContentLoaded', () => {
    // Initialize form and control elements once the DOM is fully loaded
    initializeForm();
    initializePartSelect();
    initializeScaleSelect();
    initializeTransposeToggle();
    initializeControlsAutoScroll();
    initializeDisplayToggles();
    initializeSizeControls();
});

// Function to initialize the form submission event
function initializeForm() {
    const form = document.querySelector("form");
    
    form.addEventListener("submit", event => {
        event.preventDefault();
        generateDisplay();
    });
}

// Function to initialize the part selection dropdown
function initializePartSelect() {
    const partSelect = document.getElementById('part_id');
    partSelect.addEventListener('change', () => {
        updatePartName();
        regenerateDisplayIfNeeded();
    });
    updatePartName();
}

// Function to initialize the scale selection dropdown
function initializeScaleSelect() {
    const scaleSelect = document.getElementById('scale');
    scaleSelect.addEventListener('change', () => {
        updatePartName();
        regenerateDisplayIfNeeded();
    });
}

// Function to handle the transpose toggle and related input changes
function initializeTransposeToggle() {
    const autoTransposeCheckbox = document.getElementById('auto_transpose');
    const transposeInput = document.getElementById('transpose');
    const transposeValueDisplay = document.getElementById('transpose_value');

    toggleTransposeSlider(autoTransposeCheckbox.checked);
    updateTransposeDisplay(transposeInput.value, transposeValueDisplay);

    autoTransposeCheckbox.addEventListener('change', () => {
        toggleTransposeSlider(autoTransposeCheckbox.checked);
    });

    transposeInput.addEventListener('input', () => {
        updateTransposeDisplay(transposeInput.value, transposeValueDisplay);
        regenerateDisplayIfNeeded();
    });
}

// Function to initialize various display toggle switches
function initializeDisplayToggles() {
    const toggleSwitchInlinedisplay = document.getElementById('inlineDisplay');
    const toggleSwitchShowHandpanSvg = document.getElementById('showSvg');
    const toggleSwitchRestColor = document.getElementById('showRestColor');
    const toggleSwitchPlayInScale = document.getElementById('togglePlayInScale');

    toggleSwitchInlinedisplay.addEventListener('change', updateFlexDirection);
    toggleSwitchShowHandpanSvg.addEventListener('change', showHandpanSVG);
    toggleSwitchRestColor.addEventListener('change', toggleClassOnRest);
    toggleSwitchPlayInScale.addEventListener('change', togglePlayInScale);
}

// Function to initialize controls for adjusting the SVG size
function initializeSizeControls() {
    const increaseButton = document.getElementById("increase");
    const decreaseButton = document.getElementById("decrease");

    increaseButton.addEventListener("click", () => adjustSvgSize(25, 5, true));
    decreaseButton.addEventListener("click", () => adjustSvgSize(-25, -5, false));
}

// Function to update the part name input field based on the selected part
function updatePartName() {
    const partSelect = document.getElementById("part_id");
    const selectedPartName = partSelect.options[partSelect.selectedIndex].text;
    document.getElementById("part_name").value = selectedPartName;
}

// Function to regenerate the display if necessary
function regenerateDisplayIfNeeded() {
    const mesuresDiv = document.getElementById("generate-container");
    if (mesuresDiv && mesuresDiv.innerHTML.trim() !== '') {
        generateDisplay();
    }
}

// Function to toggle the visibility of the transpose slider
function toggleTransposeSlider(shouldHide) {
    const transposeSlider = document.getElementById('transpose_slider');
    const transposeInput = document.getElementById('transpose');
    
    transposeSlider.style.display = shouldHide ? 'none' : 'block';
    if (shouldHide) {
        transposeInput.value = 0;
        transposeInput.dispatchEvent(new Event('input', { bubbles: true }));
    }
}

// Function to update the display of the transpose value
function updateTransposeDisplay(value, displayElement) {
    const numericValue = parseInt(value, 10);
    displayElement.textContent = numericValue;
    displayElement.style.color = numericValue < 0 ? 'red' : numericValue > 0 ? 'darkgreen' : 'grey';
}

// Function to adjust the size of SVG elements
function adjustSvgSize(svgDelta, restDelta, increase) {
    const svgElements = document.querySelectorAll('.svg_container svg');
    let actualSize;

    svgElements.forEach(svg => {
        let currentWidth = parseFloat(window.getComputedStyle(svg).width);
        currentWidth += svg.parentElement.classList.contains('restsvg') ? restDelta : svgDelta;
        actualSize = currentWidth;
        svg.style.width = `${currentWidth}px`;
    });

    if (increase && actualSize <= 1270) {
        textControl('.sigN', true);
        textControl('.sigD', true);
    } else if (!increase && actualSize >= 150) {
        textControl('.sigN', false);
        textControl('.sigD', false);
    }
}

// Function to update the flex direction of measure containers
function updateFlexDirection() {
    const measuresContainer = document.querySelector('.measures-container');
    const measures = document.querySelectorAll('.measure');
    const autoplay = document.querySelectorAll('.autoPlay');
    
    const isInline = this.checked;
    autoplay.forEach(autoplaydiv => {
        autoplaydiv.style.display = isInline ? "block" : "none";
    });
    measuresContainer.style.flexDirection = isInline ? 'row' : 'column';
    measures.forEach(mesure => {
        mesure.style.flexDirection = isInline ? 'column' : 'row';
    });
}

// Function to toggle the visibility of handpan SVG elements
function showHandpanSVG() {
    const svgElements = document.querySelectorAll('.handpansvg svg');
    svgElements.forEach(svg => {
        svg.style.display = this.checked ? "initial" : "none";
    });
}

// Function to toggle the class on rest elements for color changes
function toggleClassOnRest() {
    const elements = document.querySelectorAll('.rest-svg');
    elements.forEach(element => {
        element.classList.toggle('rest-svg-out', !this.checked);
    });
}

// Function to toggle play in scale mode
function togglePlayInScale() {
    const play_only_inscale = document.getElementById('play_only_inscale');
    play_only_inscale.value = this.checked ? "1" : "0";
    regenerateDisplayIfNeeded();
}

// Function to control the font size of text elements
function textControl(selector, increase) {
    const element = document.querySelector(selector);
    if (!element) return console.log(`Element with selector ${selector} not found.`);

    const currentFontSize = parseFloat(window.getComputedStyle(element).fontSize);
    const currentFontSizeInEm = currentFontSize / 16;
    const newFontSizeInEm = currentFontSizeInEm + (increase ? 0.25 : -0.25);
    element.style.fontSize = `${newFontSizeInEm}em`;
    element.style.lineHeight = `${newFontSizeInEm / 2}em`;
}

// Function to map a range of values to another range
function mapRange(value, inMin, inMax, outMin, outMax) {
    return outMin + (outMax - outMin) * ((value - inMin) / (inMax - inMin));
}

// Function to generate the display based on form data
function generateDisplay() {
    const form = document.querySelector("form");
    fetch("/generate", {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded"
        },
        body: new URLSearchParams(new FormData(form)).toString()
    })
    .then(response => response.text())
    .then(htmlCode => {
        const mesuresDiv = document.getElementById("generate-container");
        mesuresDiv.innerHTML = htmlCode;
        document.getElementById("controls").style.display = "block";
        document.getElementById("legends").style.display = "block";
    })
    .catch(error => {
        console.error("Error fetching the HTML:", error);
    });
}

// Function to create the reader bar for tracking note playback
function createReaderBar() {
    const readerBar = document.querySelector('.reader-bar');
    const measuresContainer = document.querySelector('.measures-container');

    const measuresContainerTop = measuresContainer.getBoundingClientRect().top + window.scrollY;
    readerBar.style.top = `${measuresContainerTop}px`;

    const measuresContainerHeight = measuresContainer.offsetHeight;
    readerBar.style.height = `${measuresContainerHeight}px`;
    readerBar.style.display = "block";
}

// Function to remove the reader bar
function removeReaderBar() {
    const readerBar = document.querySelector('.reader-bar');
    if (readerBar) {
        readerBar.style.display = "none";
    }
}

// Main function to control the auto-scroll functionality
function initializeControlsAutoScroll() {
    let isPlaying = false;
    let currentNoteIndex = 0;
    let totalScrollDistance = 0;
    let scrollTimeout;
    let readerBarCurrentX = 0;
    let audioContext;

    const playPauseButton = document.getElementById('playPause');
    const resetButton = document.getElementById('resetScroll');
    const scrollRateBpmInput = document.getElementById('scrollRateBpm');
    const scrollRateBpmValue = document.getElementById('scrollRateBpmValue');
    const generateContainer = document.getElementById('generate-container');

    // Display the initial BPM value
    scrollRateBpmValue.textContent = `${scrollRateBpmInput.value} BPM`;
    resetScrolling(generateContainer);

    // Event listener for the Play/Pause button
    playPauseButton.addEventListener('click', () => {
        if (!audioContext) {
            audioContext = new (window.AudioContext || window.webkitAudioContext)();
        } else if (audioContext.state === 'suspended') {
            audioContext.resume();
        }
        isPlaying = !isPlaying;
        if (isPlaying) {
            startScrolling();
        } else {
            stopScrolling(true);
        }
    });

    // Event listener for the Reset button
    resetButton.addEventListener('click', () => {
        const generateContainer = document.getElementById('generate-container');
        resetScrolling(generateContainer);
    });

    // Event listener for changing the scroll rate
    scrollRateBpmInput.addEventListener('input', () => {
        scrollRateBpmValue.textContent = `${scrollRateBpmInput.value} BPM`;
        if (isPlaying) {
            stopScrolling();
            startScrolling();
        }
    });

    // Function to handle scrolling to the next note
    function scrollNextNote() {
        if (!isPlaying) return;
        const generateContainer = document.getElementById('generate-container');
        const measuresContainer = document.querySelector('.measures-container');
        const readerBar = document.querySelector('.reader-bar');
        const notes = document.querySelectorAll('.note');
        if (currentNoteIndex >= notes.length) return;

        const note = notes[currentNoteIndex];

        if (note) {
            const measureElement = note.closest('.measure');
            document.querySelectorAll('.measure-header').forEach(header => header.classList.remove('active'));
            const activeMeasureHeader = measureElement?.querySelector('.measure-header');
            activeMeasureHeader ? activeMeasureHeader.classList.add('active') : console.warn('Aucune .measure-header trouvée dans la mesure sélectionnée.');
        } else {
            console.warn('Aucun élément .note trouvé avec la classe spécifiée.');
        }

        const bpm = parseInt(document.getElementById('scrollRateBpm').value, 10);
        const sigD = parseInt(note.getAttribute('sigd'));
        const duration = note.getAttribute('duration');
        const pitches = note.getAttribute('pitches').split(';').map(Number);
        playNoteWithMidi(pitches, duration, bpm, sigD); // Play the notes as sounds

        const scrollSpeed = noteDuration(duration, bpm, sigD) * 1000;
        const noteWidth = note.offsetWidth;
        let noteProgress = 0;
        const noteXPos = note.getBoundingClientRect().left;
        const containerXPos = generateContainer.getBoundingClientRect().left;
        readerBarCurrentX = noteXPos - containerXPos;
        readerBar.style.transform = `translateX(${readerBarCurrentX}px)`;

        function smoothScroll() {
            if (!isPlaying) return;
            const adjustFactor = 16; // Increase this value to speed up the scroll, or decrease it to slow down.
            const scrollStep = (noteWidth / scrollSpeed) * adjustFactor;
            readerBarCurrentX += scrollStep;
            noteProgress += scrollStep;
            totalScrollDistance += scrollStep;
            readerBar.style.transform = `translateX(${readerBarCurrentX}px)`;

            if (noteProgress <= noteWidth) {
                requestAnimationFrame(smoothScroll);
            } else {
                currentNoteIndex++;
                if (readerBarCurrentX >= measuresContainer.clientWidth * 0.8) {
                    generateContainer.scrollLeft = `${totalScrollDistance}`;
                }
                scrollNextNote();
            }
        }
        smoothScroll();
    }

    // Function to start scrolling through notes
    function startScrolling() {
        createReaderBar();
        scrollNextNote();
        playPauseButton.textContent = '⏸ Pause';
    }

    // Function to stop scrolling
    function stopScrolling(isPaused = false) {
        playPauseButton.textContent = '▶ Play';
        if (!isPaused){
            removeReaderBar();
        }
        clearTimeout(scrollTimeout);
        isPlaying = false;
    }

    // Function to reset scrolling to the beginning
    function resetScrolling(container) {
        stopScrolling();
        currentNoteIndex = 0;
        totalScrollDistance = 0;
        readerBarCurrentX = 0;
        container.scrollLeft = 0;

        const readerBar = document.querySelector('.reader-bar');
        if (readerBar) {
            readerBar.style.transform = `translateX(0px)`;
        }
    }

    // Function to convert MIDI pitch to frequency
    function midiToFrequency(pitch) {
        return 440 * Math.pow(2, (pitch - 69) / 12);
    }

    // Function to calculate the duration of a note based on its type, BPM, and time signature denominator
    function noteDuration(type, bpm, sigD) {
        const beatDuration = 60 / bpm; // Duration of a quarter note in seconds
        const durationMapping = {
            'whole': 4,
            'half': 2,
            'quarter': 1,
            'eighth': 0.5,
            '16th': 0.25,
            '32nd': 0.125,
            '64th': 0.0625
        };

        const baseDuration = durationMapping[type];
        if (!baseDuration) {
            console.error(`Invalid note type: ${type}`);
            return 0;
        }

        // Adjust the duration based on the time signature denominator
        const duration = baseDuration * (4 / sigD) * beatDuration;
        return duration;
    }

    // Function to play a MIDI note with the specified duration, BPM, and time signature denominator
    function playNoteWithMidi(pitches, type, bpm = 120, sigD = 4) {
        const duration = noteDuration(type, bpm, sigD);
        playMidi(pitches, duration);
    }

    // Function to create a custom waveform for more complex sounds
    function createCustomWaveform(audioContext) {
        // Define harmonic content for a more complex, bell-like sound
        const real = new Float32Array([0, 1, 0.7, 0.5, 0.3, 0.2, 0.1]); // Emphasize lower harmonics, slightly reduce higher ones
        const imag = new Float32Array(real.length); // Imaginary part remains zeros

        // Create the periodic wave
        const customWave = audioContext.createPeriodicWave(real, imag);

        return customWave;
    }

    // Function to play MIDI pitches with a specified duration and audio effects
    function playMidi(pitches, duration = 2.5) {
        pitches.forEach(pitch => {
            if (pitch === 0) return; // Do not play sound if pitch is 0

            const carrierFrequency = midiToFrequency(pitch);
            const modulationIndex = 30;
            const vibratoRate = 3; // Slow vibrato for a more ethereal effect
            const vibratoDepth = 0.1; // Subtle vibrato depth for smoothness

            const carrierOscillator = audioContext.createOscillator();
            const vibratoOscillator = audioContext.createOscillator();
            const gainNode = audioContext.createGain();
            const modGainNode = audioContext.createGain();

            // Custom waveform to create a richer, rounder sound
            carrierOscillator.setPeriodicWave(createCustomWaveform(audioContext));

            // Configure the carrier oscillator
            carrierOscillator.frequency.value = carrierFrequency;
            carrierOscillator.type = 'sine';

            // Configure the vibrato oscillator to modulate the carrier frequency
            vibratoOscillator.frequency.value = vibratoRate; // Slow vibrato rate
            modGainNode.gain.value = vibratoDepth; // Very subtle vibrato depth
            vibratoOscillator.connect(modGainNode);
            modGainNode.connect(carrierOscillator.frequency);

            // Create a delay node for the echo effect
            const delayNode = audioContext.createDelay();
            delayNode.delayTime.value = 0.5; // Longer delay for a sustained reverb

            // Create a gain node for feedback to simulate reverb
            const feedbackGainNode = audioContext.createGain();
            feedbackGainNode.gain.value = 0.2; // Adjust feedback level for a longer, more ethereal reverb

            // Connect nodes
            carrierOscillator.connect(gainNode);
            gainNode.connect(delayNode);
            delayNode.connect(feedbackGainNode);
            feedbackGainNode.connect(delayNode); // Feedback loop for reverb
            feedbackGainNode.connect(audioContext.destination);

            // Connect directly to the destination for the original signal
            gainNode.connect(audioContext.destination);

            // Start the oscillators
            vibratoOscillator.start();
            carrierOscillator.start();

            // Apply a fade-in to avoid clipping at the start
            gainNode.gain.setValueAtTime(0.001, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(1, audioContext.currentTime + 0.1);

            // Apply a fade-out to avoid clipping at the end
            gainNode.gain.setValueAtTime(1, audioContext.currentTime + duration);
            gainNode.gain.exponentialRampToValueAtTime(0.001, audioContext.currentTime + duration + 1.0);

            // Stop the oscillators after the specified duration
            carrierOscillator.stop(audioContext.currentTime + duration + 1.0);
            vibratoOscillator.stop(audioContext.currentTime + duration + 1.0);
        });
    }
}