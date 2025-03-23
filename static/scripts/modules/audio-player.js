export class AudioPlayer {
    constructor() {
        this.audioContext = null;
        this.isPlaying = false;
        this.currentNoteIndex = 0;
    }

    initialize() {
        if (!this.audioContext) {
            this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
        } else if (this.audioContext.state === 'suspended') {
            this.audioContext.resume();
        }
    }

    midiToFrequency(pitch) {
        return 440 * Math.pow(2, (pitch - 69) / 12);
    }

    noteDuration(type, bpm, sigD) {
        const beatDuration = 60 / bpm;
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

        return baseDuration * (4 / sigD) * beatDuration;
    }

    createCustomWaveform() {
        const real = new Float32Array([0, 1, 0.7, 0.5, 0.3, 0.2, 0.1]);
        const imag = new Float32Array(real.length);
        return this.audioContext.createPeriodicWave(real, imag);
    }

    playNote(pitches, type, bpm = 120, sigD = 4) {
        this.initialize();
        const duration = this.noteDuration(type, bpm, sigD);

        pitches.forEach(pitch => {
            if (pitch === 0) return;

            const carrierFrequency = this.midiToFrequency(pitch);
            const carrierOscillator = this.audioContext.createOscillator();
            const vibratoOscillator = this.audioContext.createOscillator();
            const gainNode = this.audioContext.createGain();
            const modGainNode = this.audioContext.createGain();
            const delayNode = this.audioContext.createDelay();
            const feedbackGainNode = this.audioContext.createGain();

            // Configuration des oscillateurs
            carrierOscillator.setPeriodicWave(this.createCustomWaveform());
            carrierOscillator.frequency.value = carrierFrequency;
            vibratoOscillator.frequency.value = 3;
            modGainNode.gain.value = 0.1;

            // Configuration du delay et feedback
            delayNode.delayTime.value = 0.5;
            feedbackGainNode.gain.value = 0.2;

            // Connexions des nodes
            vibratoOscillator.connect(modGainNode);
            modGainNode.connect(carrierOscillator.frequency);
            carrierOscillator.connect(gainNode);
            gainNode.connect(delayNode);
            delayNode.connect(feedbackGainNode);
            feedbackGainNode.connect(delayNode);
            feedbackGainNode.connect(this.audioContext.destination);
            gainNode.connect(this.audioContext.destination);

            // Démarrage et enveloppe sonore
            const startTime = this.audioContext.currentTime;
            const releaseTime = startTime + duration;
            const stopTime = releaseTime + 1.0;

            gainNode.gain.setValueAtTime(0.001, startTime);
            gainNode.gain.exponentialRampToValueAtTime(1, startTime + 0.1);
            gainNode.gain.setValueAtTime(1, releaseTime);
            gainNode.gain.exponentialRampToValueAtTime(0.001, stopTime);

            vibratoOscillator.start(startTime);
            carrierOscillator.start(startTime);
            vibratoOscillator.stop(stopTime);
            carrierOscillator.stop(stopTime);
        });
    }

    cleanup() {
        if (this.audioContext) {
            this.audioContext.close();
            this.audioContext = null;
        }
    }
}
