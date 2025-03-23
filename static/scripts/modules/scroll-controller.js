import { AudioPlayer } from './audio-player.js';

export class ScrollController {
    constructor() {
        this.isPlaying = false;
        this.currentNoteIndex = 0;
        this.totalScrollDistance = 0;
        this.readerBarCurrentX = 0;
        this.scrollTimeout = null;
        this.audioPlayer = new AudioPlayer();

        this.initializeControls();
    }

    initializeControls() {
        // Initialisation des éléments de contrôle
        this.playPauseButton = document.getElementById('playPause');
        this.resetButton = document.getElementById('resetScroll');
        this.scrollRateBpmInput = document.getElementById('scrollRateBpm');
        this.scrollRateBpmValue = document.getElementById('scrollRateBpmValue');
        this.generateContainer = document.getElementById('generate-container');

        if (this.scrollRateBpmValue && this.scrollRateBpmInput) {
            this.scrollRateBpmValue.textContent = `${this.scrollRateBpmInput.value} BPM`;
        }

        this.setupEventListeners();
    }

    setupEventListeners() {
        if (this.playPauseButton) {
            this.playPauseButton.addEventListener('click', () => {
                this.isPlaying = !this.isPlaying;
                if (this.isPlaying) {
                    this.startScrolling();
                } else {
                    this.stopScrolling(true);
                }
            });
        }

        if (this.resetButton) {
            this.resetButton.addEventListener('click', () => {
                this.resetScrolling();
            });
        }

        if (this.scrollRateBpmInput) {
            this.scrollRateBpmInput.addEventListener('input', () => {
                if (this.scrollRateBpmValue) {
                    this.scrollRateBpmValue.textContent = `${this.scrollRateBpmInput.value} BPM`;
                }
                if (this.isPlaying) {
                    this.stopScrolling();
                    this.startScrolling();
                }
            });
        }
    }

    createReaderBar() {
        const readerBar = document.querySelector('.reader-bar');
        const measuresContainer = document.querySelector('.measures-container');
        
        if (!readerBar || !measuresContainer) return;

        const measuresContainerTop = measuresContainer.getBoundingClientRect().top + window.scrollY;
        readerBar.style.top = `${measuresContainerTop}px`;
        readerBar.style.height = `${measuresContainer.offsetHeight}px`;
        readerBar.style.display = "block";
    }

    removeReaderBar() {
        const readerBar = document.querySelector('.reader-bar');
        if (readerBar) {
            readerBar.style.display = "none";
        }
    }

    scrollNextNote() {
        if (!this.isPlaying) return;

        const measuresContainer = document.querySelector('.measures-container');
        const readerBar = document.querySelector('.reader-bar');
        const notes = document.querySelectorAll('.note');

        if (this.currentNoteIndex >= notes.length) return;

        const note = notes[this.currentNoteIndex];
        if (!note) return;

        // Mise à jour de l'en-tête de mesure active
        const measureElement = note.closest('.measure');
        document.querySelectorAll('.measure-header').forEach(header => header.classList.remove('active'));
        const activeMeasureHeader = measureElement?.querySelector('.measure-header');
        if (activeMeasureHeader) {
            activeMeasureHeader.classList.add('active');
        }

        // Lecture de la note
        const bpm = parseInt(this.scrollRateBpmInput?.value || 120, 10);
        const sigD = parseInt(note.getAttribute('sigd') || 4);
        const duration = note.getAttribute('duration');
        const pitches = note.getAttribute('pitches').split(';').map(Number);

        // Jouer la note
        this.audioPlayer.playNote(pitches, duration, bpm, sigD);

        // Animation de défilement
        const scrollSpeed = this.audioPlayer.noteDuration(duration, bpm, sigD) * 1000;
        const noteWidth = note.offsetWidth;
        let noteProgress = 0;

        // Position de la barre de lecture
        if (this.generateContainer && readerBar) {
            const noteXPos = note.getBoundingClientRect().left;
            const containerXPos = this.generateContainer.getBoundingClientRect().left;
            this.readerBarCurrentX = noteXPos - containerXPos;
            readerBar.style.transform = `translateX(${this.readerBarCurrentX}px)`;
        }

        const smoothScroll = () => {
            if (!this.isPlaying) return;

            const adjustFactor = 16;
            const scrollStep = (noteWidth / scrollSpeed) * adjustFactor;
            this.readerBarCurrentX += scrollStep;
            noteProgress += scrollStep;
            this.totalScrollDistance += scrollStep;

            if (readerBar) {
                readerBar.style.transform = `translateX(${this.readerBarCurrentX}px)`;
            }

            if (noteProgress <= noteWidth) {
                requestAnimationFrame(smoothScroll);
            } else {
                this.currentNoteIndex++;
                if (measuresContainer && this.readerBarCurrentX >= measuresContainer.clientWidth * 0.8) {
                    this.generateContainer.scrollLeft = this.totalScrollDistance;
                }
                this.scrollNextNote();
            }
        };

        smoothScroll();
    }

    startScrolling() {
        this.createReaderBar();
        this.scrollNextNote();
        if (this.playPauseButton) {
            this.playPauseButton.textContent = '⏸ Pause';
        }
    }

    stopScrolling(isPaused = false) {
        if (this.playPauseButton) {
            this.playPauseButton.textContent = '▶ Play';
        }
        if (!isPaused) {
            this.removeReaderBar();
        }
        clearTimeout(this.scrollTimeout);
        this.isPlaying = false;
    }

    resetScrolling() {
        this.stopScrolling();
        this.currentNoteIndex = 0;
        this.totalScrollDistance = 0;
        this.readerBarCurrentX = 0;
        
        if (this.generateContainer) {
            this.generateContainer.scrollLeft = 0;
        }

        const readerBar = document.querySelector('.reader-bar');
        if (readerBar) {
            readerBar.style.transform = 'translateX(0px)';
        }
    }

    cleanup() {
        this.stopScrolling();
        this.audioPlayer.cleanup();
        
        // Nettoyage des event listeners
        if (this.playPauseButton) {
            this.playPauseButton.replaceWith(this.playPauseButton.cloneNode(true));
        }
        if (this.resetButton) {
            this.resetButton.replaceWith(this.resetButton.cloneNode(true));
        }
        if (this.scrollRateBpmInput) {
            this.scrollRateBpmInput.replaceWith(this.scrollRateBpmInput.cloneNode(true));
        }
    }
}
