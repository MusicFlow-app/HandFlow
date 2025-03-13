use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
// GainNode est importé automatiquement via web_sys dans les fonctions


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetronomeSettings {
    pub bpm: u32,
    pub subdivision: u32,
    pub volume: f32,
    pub is_playing: bool,
    pub current_beat: u32,
    pub beats_per_bar: u32,
}

impl Default for MetronomeSettings {
    fn default() -> Self {
        Self {
            bpm: 120,
            subdivision: 1,
            volume: 1.0,
            is_playing: false,
            current_beat: 0,
            beats_per_bar: 4,
        }
    }
}

#[wasm_bindgen]
pub struct Metronome {
    settings: Arc<Mutex<MetronomeSettings>>,
    audio_context: web_sys::AudioContext,
}

#[wasm_bindgen]
impl Metronome {
    pub fn new() -> Result<Metronome, JsValue> {
        let audio_context = web_sys::AudioContext::new()?;
        
        Ok(Metronome {
            settings: Arc::new(Mutex::new(MetronomeSettings::default())),
            audio_context,
        })
    }

    pub async fn start(&self) -> Result<(), JsValue> {
        let mut settings = self.settings.lock().await;
        settings.is_playing = true;
        self.schedule_beats().await
    }

    pub async fn stop(&self) -> Result<(), JsValue> {
        let mut settings = self.settings.lock().await;
        settings.is_playing = false;
        Ok(())
    }

    pub async fn set_bpm(&self, bpm: u32) -> Result<(), JsValue> {
        let mut settings = self.settings.lock().await;
        settings.bpm = bpm.clamp(40, 240);
        Ok(())
    }

    pub async fn set_subdivision(&self, subdivision: u32) -> Result<(), JsValue> {
        let mut settings = self.settings.lock().await;
        settings.subdivision = subdivision;
        Ok(())
    }

    pub async fn set_volume(&self, volume: f32) -> Result<(), JsValue> {
        let mut settings = self.settings.lock().await;
        settings.volume = volume.clamp(0.0, 1.0);
        Ok(())
    }

    pub async fn set_beats_per_bar(&self, beats: u32) -> Result<(), JsValue> {
        let mut settings = self.settings.lock().await;
        settings.beats_per_bar = beats.clamp(1, 12);
        Ok(())
    }

    async fn schedule_beats(&self) -> Result<(), JsValue> {
        let mut settings = self.settings.lock().await;
        let base_interval = (60.0 / settings.bpm as f64) * 1000.0;
        let subdivision_interval = base_interval / settings.subdivision as f64;
        
        let click_buffer = self.load_sound_buffer(true).await?;
        let clave_buffer = self.load_sound_buffer(false).await?;
        
        while settings.is_playing {
            let is_first_beat = settings.current_beat % (settings.beats_per_bar * settings.subdivision) == 0;
            let buffer = if is_first_beat { &click_buffer } else { &clave_buffer };
            
            self.play_sound(buffer, is_first_beat).await?;
            
            settings.current_beat = (settings.current_beat + 1) % (settings.beats_per_bar * settings.subdivision);
            tokio::time::sleep(tokio::time::Duration::from_millis(subdivision_interval as u64)).await;
        }
        
        settings.current_beat = 0;
        Ok(())
    }

    async fn load_sound_buffer(&self, is_click: bool) -> Result<web_sys::AudioBuffer, JsValue> {
        let sound_path = if is_click {
            "/static/audio/metronome/clic.flac"
        } else {
            "/static/audio/metronome/clav.flac"
        };
        
        let window = web_sys::window().unwrap();
        let response = JsFuture::from(window.fetch_with_str(sound_path)).await?;
        let array_buffer = JsFuture::from(response.dyn_into::<web_sys::Response>()?.array_buffer()?).await?;
        let audio_data = array_buffer.dyn_into::<js_sys::ArrayBuffer>()?;
        
        JsFuture::from(self.audio_context.decode_audio_data(&audio_data)?)
            .await?
            .dyn_into::<web_sys::AudioBuffer>()
    }

    async fn play_sound(&self, buffer: &web_sys::AudioBuffer, is_accent: bool) -> Result<(), JsValue> {
        let source = self.audio_context.create_buffer_source()?;
        source.set_buffer(Some(buffer));
        
        let gain = self.audio_context.create_gain()?;
        let settings = self.settings.lock().await;
        gain.gain().set_value(if is_accent { settings.volume } else { settings.volume * 0.8 });
        
        source.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&self.audio_context.destination())?;
        
        source.start()?;
        Ok(())
    }
}
