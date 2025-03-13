import { Metronome } from './metronome.js';

export class AudioController {
    constructor() {
        console.log('Initializing AudioController...');
        this.metronome = new Metronome();
        this.isPlaying = false;
        this.bpm = 120;
        this.playInScaleOnly = false;
        this.eventListeners = [];
        this.customEventListeners = [];
        this.initializeEventListeners();
        console.log('AudioController initialized');
    }

    initializeEventListeners() {
        // Helper to safely add event listeners
        const addListener = (id, event, handler) => {
            const element = document.getElementById(id);
            if (element) {
                const boundHandler = handler.bind(this);
                element.addEventListener(event, boundHandler);
                this.eventListeners.push({ element, event, handler: boundHandler });
            } else {
                console.warn(`Element with id '${id}' not found`);
            }
        };

        // Helper to add custom event listeners
        const addCustomListener = (event, handler) => {
            const boundHandler = handler.bind(this);
            document.addEventListener(event, boundHandler);
            this.customEventListeners.push({ event, handler: boundHandler });
        };

        // Playback controls
        addListener('playPause', 'click', this.togglePlayback);
        addListener('resetScroll', 'click', this.resetScroll);
        addListener('togglePlayInScale', 'change', (e) => {
            this.playInScaleOnly = e.target.checked;
            this.updatePlayInScale();
        });

        // BPM control
        addListener('scrollRateBpm', 'input', (e) => {
            this.bpm = parseInt(e.target.value);
            this.updateBpm();
        });

        // Metronome controls
        addListener('metronomeToggle', 'click', this.toggleMetronome);
        addListener('beatsPerBar', 'change', (e) => {
            this.metronome.setTimeSignature(e.target.value);
        });
        addListener('metronomeVolume', 'input', (e) => {
            this.metronome.setVolume(e.target.value);
        });

        // Sync BPM between metronome and scroll
        addCustomListener('bpmChange', (e) => {
            this.bpm = e.detail.bpm;
            const bpmInput = document.getElementById('scrollRateBpm');
            if (bpmInput) {
                bpmInput.value = this.bpm;
                const bpmValue = document.getElementById('scrollRateBpmValue');
                if (bpmValue) {
                    bpmValue.textContent = `${this.bpm} BPM`;
                }
            }
        });
    }

    togglePlayback() {
        this.isPlaying = !this.isPlaying;
        const playButton = document.getElementById('playPause');
        playButton.textContent = this.isPlaying ? '⏸ Pause' : '▶ Play';
        playButton.classList.toggle('playing', this.isPlaying);
        
        document.dispatchEvent(new CustomEvent('togglePlayback', {
            detail: { isPlaying: this.isPlaying }
        }));
    }

    resetScroll() {
        document.dispatchEvent(new CustomEvent('resetScroll'));
    }

    updateBpm() {
        this.metronome.setBpm(this.bpm);
        document.getElementById('scrollRateBpmValue').textContent = `${this.bpm} BPM`;
    }

    updatePlayInScale() {
        document.dispatchEvent(new CustomEvent('playInScaleChange', {
            detail: { playInScaleOnly: this.playInScaleOnly }
        }));
    }

    toggleMetronome() {
        const isPlaying = this.metronome.toggle();
        const metronomeButton = document.getElementById('metronomeToggle');
        metronomeButton.textContent = isPlaying ? 'Stop' : 'Start';
        metronomeButton.classList.toggle('active', isPlaying);
    }

    cleanup() {
        console.log('Cleaning up AudioController...');
        
        // Clean up DOM event listeners
        this.eventListeners.forEach(({ element, event, handler }) => {
            element.removeEventListener(event, handler);
        });
        this.eventListeners = [];

        // Clean up custom event listeners
        this.customEventListeners.forEach(({ event, handler }) => {
            document.removeEventListener(event, handler);
        });
        this.customEventListeners = [];

        // Clean up metronome
        if (this.metronome) {
            this.metronome.cleanup();
        }

        console.log('AudioController cleanup complete');
    }
}
