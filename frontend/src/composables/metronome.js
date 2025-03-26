export class Metronome {
    constructor() {
        this.audioContext = null;
        this.gainNode = null;
        this.isPlaying = false;
        this.bpm = 120;
        this.beatsPerBar = 4;
        this.volume = 80;
        this.intervalId = null;
        this.initializeAudio();
    }

    initializeAudio() {
        this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
        this.gainNode = this.audioContext.createGain();
        this.gainNode.connect(this.audioContext.destination);
        this.setVolume(this.volume);
    }

    toggle() {
        this.isPlaying = !this.isPlaying;
        if (this.isPlaying) {
            this.start();
        } else {
            this.stop();
        }
        return this.isPlaying;
    }

    start() {
        if (!this.audioContext) this.initializeAudio();
        
        const beatLength = 60.0 / this.bpm;
        let beat = 0;
        
        const schedule = () => {
            const time = this.audioContext.currentTime;
            this.playClick(time, beat === 0);
            beat = (beat + 1) % this.beatsPerBar;
        };

        this.intervalId = setInterval(schedule, beatLength * 1000);
    }

    stop() {
        if (this.intervalId) {
            clearInterval(this.intervalId);
            this.intervalId = null;
        }
    }

    playClick(time, isAccented) {
        const osc = this.audioContext.createOscillator();
        const gain = this.audioContext.createGain();
        
        osc.connect(gain);
        gain.connect(this.gainNode);

        osc.frequency.value = isAccented ? 1000 : 800;
        gain.gain.value = isAccented ? 1 : 0.7;

        osc.start(time);
        osc.stop(time + 0.1);
    }

    setTimeSignature(beats) {
        this.beatsPerBar = parseInt(beats);
        if (this.isPlaying) {
            this.stop();
            this.start();
        }
    }

    setBpm(newBpm) {
        this.bpm = parseInt(newBpm);
        if (this.isPlaying) {
            this.stop();
            this.start();
        }
        // Dispatch event for synchronization
        document.dispatchEvent(new CustomEvent('bpmChange', { 
            detail: { bpm: this.bpm }
        }));
    }

    setVolume(value) {
        this.volume = parseInt(value);
        if (this.gainNode) {
            this.gainNode.gain.value = this.volume / 100;
        }
    }

    cleanup() {
        this.stop();
        if (this.audioContext) {
            this.audioContext.close();
            this.audioContext = null;
        }
    }
}
