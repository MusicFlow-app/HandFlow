import { defineStore } from 'pinia'

export const useMetronomeStore = defineStore('metronome', {
  state: () => ({
    isPlaying: false,
    bpm: 120,
    beatsPerBar: 4,
    volume: 80,
    audioContext: null,
    oscillator: null,
    gainNode: null
  }),

  actions: {
    initializeMetronome() {
      this.audioContext = new (window.AudioContext || window.webkitAudioContext)()
      this.gainNode = this.audioContext.createGain()
      this.gainNode.connect(this.audioContext.destination)
      this.setVolume(this.volume)
    },

    toggleMetronome() {
      this.isPlaying = !this.isPlaying
      if (this.isPlaying) {
        this.startMetronome()
      } else {
        this.stopMetronome()
      }
    },

    startMetronome() {
      if (!this.audioContext) this.initializeMetronome()
      
      const beatLength = 60.0 / this.bpm
      let beat = 0
      
      const schedule = () => {
        const time = this.audioContext.currentTime
        this.playClick(time, beat === 0)
        beat = (beat + 1) % this.beatsPerBar
      }

      this.intervalId = setInterval(schedule, beatLength * 1000)
    },

    stopMetronome() {
      if (this.intervalId) {
        clearInterval(this.intervalId)
        this.intervalId = null
      }
    },

    playClick(time, isAccented) {
      const osc = this.audioContext.createOscillator()
      const gain = this.audioContext.createGain()
      
      osc.connect(gain)
      gain.connect(this.gainNode)

      osc.frequency.value = isAccented ? 1000 : 800
      gain.gain.value = isAccented ? 1 : 0.7

      osc.start(time)
      osc.stop(time + 0.1)
    },

    setTimeSignature(beats) {
      this.beatsPerBar = parseInt(beats)
      if (this.isPlaying) {
        this.stopMetronome()
        this.startMetronome()
      }
    },

    setBpm(newBpm) {
      this.bpm = parseInt(newBpm)
      if (this.isPlaying) {
        this.stopMetronome()
        this.startMetronome()
      }
      // Sync with auto-scroll
      document.dispatchEvent(new CustomEvent('bpmChange', { detail: { bpm: this.bpm } }))
    },

    setVolume(value) {
      this.volume = parseInt(value)
      if (this.gainNode) {
        this.gainNode.gain.value = this.volume / 100
      }
    },

    cleanup() {
      this.stopMetronome()
      if (this.audioContext) {
        this.audioContext.close()
        this.audioContext = null
      }
    }
  }
})
