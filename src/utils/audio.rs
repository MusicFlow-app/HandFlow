use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioBuffer, AudioBufferSourceNode, AudioContext, 
    BiquadFilterNode, ConvolverNode, DelayNode, GainNode,
    Request, RequestInit, RequestMode, Response};
use js_sys::ArrayBuffer;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub reverb_level: f32,
    pub delay_time: f32,
    pub delay_feedback: f32,
    pub eq_low: f32,
    pub eq_mid: f32,
    pub eq_high: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            reverb_level: 0.3,
            delay_time: 0.3,
            delay_feedback: 0.2,
            eq_low: 0.0,
            eq_mid: 0.0,
            eq_high: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub midi_note: u8,
    pub name: String,
    pub octave: i8,
    pub volume: f32,
    pub pan: f32,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            midi_note: 60,
            name: String::from("C"),
            octave: 4,
            volume: 1.0,
            pan: 0.0,
        }
    }
}

#[allow(dead_code)]
pub struct AudioEngine {
    context: AudioContext,
    samples: Arc<Mutex<HashMap<u8, AudioBuffer>>>,
    settings: Arc<Mutex<AudioSettings>>,
    master_gain: GainNode,
    reverb: Option<ConvolverNode>,
    delay: DelayNode,
    delay_feedback: GainNode,
    eq_low: BiquadFilterNode,
    eq_mid: BiquadFilterNode,
    eq_high: BiquadFilterNode,
    metronome_gain: GainNode,
    active_sources: Arc<Mutex<Vec<AudioBufferSourceNode>>>,
}

impl AudioEngine {
    pub fn new() -> Result<Self, JsValue> {
        let context = web_sys::AudioContext::new()?;
        
        // Création des nodes audio
        let master_gain = context.create_gain()?;
        master_gain.connect_with_audio_node(&context.destination())?;
        
        let delay = context.create_delay_with_max_delay_time(5.0)?;
        let delay_feedback = context.create_gain()?;
        delay_feedback.gain().set_value(0.2);
        
        // Configuration de l'EQ à 3 bandes
        let eq_low = context.create_biquad_filter()?;
        eq_low.set_type(web_sys::BiquadFilterType::Lowshelf);
        eq_low.frequency().set_value(200.0);
        
        let eq_mid = context.create_biquad_filter()?;
        eq_mid.set_type(web_sys::BiquadFilterType::Peaking);
        eq_mid.frequency().set_value(1000.0);
        eq_mid.q().set_value(0.5);
        
        let eq_high = context.create_biquad_filter()?;
        eq_high.set_type(web_sys::BiquadFilterType::Highshelf);
        eq_high.frequency().set_value(3000.0);
        
        // Gain pour le métronome
        let metronome_gain = context.create_gain()?;
        
        // Chaînage des nodes audio
        eq_low.connect_with_audio_node(&eq_mid)?;
        eq_mid.connect_with_audio_node(&eq_high)?;
        eq_high.connect_with_audio_node(&master_gain)?;
        
        delay.connect_with_audio_node(&delay_feedback)?;
        delay_feedback.connect_with_audio_node(&delay)?;
        delay.connect_with_audio_node(&master_gain)?;
        
        metronome_gain.connect_with_audio_node(&master_gain)?;
        
        Ok(Self {
            context,
            samples: Arc::new(Mutex::new(HashMap::new())),
            settings: Arc::new(Mutex::new(AudioSettings::default())),
            master_gain,
            reverb: None,
            delay,
            delay_feedback,
            eq_low,
            eq_mid,
            eq_high,
            metronome_gain,
            active_sources: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn load_handpan_samples(&self) -> Result<(), JsValue> {
        let mut samples = self.samples.lock().await;
        
        // Chargement des notes normales (40-81)
        for midi_note in 40..=81 {
            let path = format!("/static/audio/handpan/{}.flac", midi_note);
            match self.load_audio_file(&path).await {
                Ok(buffer) => { samples.insert(midi_note, buffer); }
                Err(_) => continue, // Ignorer les notes manquantes
            }
        }

        // Chargement du son gu (D2)
        if let Ok(buffer) = self.load_audio_file("/static/audio/handpan/38_gu.flac").await {
            samples.insert(38, buffer);
        }

        // Chargement du son clac
        if let Ok(buffer) = self.load_audio_file("/static/audio/handpan/127_clac.flac").await {
            samples.insert(127, buffer);
        }

        Ok(())
    }

    pub async fn load_metronome_samples(&self) -> Result<(), JsValue> {
        let mut samples = self.samples.lock().await;
        
        // Chargement des sons du métronome
        let click_buffer = self.load_audio_file("/static/audio/metronome/clic.flac").await?;
        let clave_buffer = self.load_audio_file("/static/audio/metronome/clav.flac").await?;
        
        samples.insert(254, click_buffer);  // Réservé pour le clic du métronome
        samples.insert(255, clave_buffer);  // Réservé pour la clave du métronome
        
        Ok(())
    }

    pub async fn play_note(&self, note: Note) -> Result<(), JsValue> {
        let samples = self.samples.lock().await;
        if let Some(buffer) = samples.get(&note.midi_note) {
            let source = self.context.create_buffer_source()?;
            source.set_buffer(Some(buffer));
            
            // Création du gain pour la note
            let note_gain = self.context.create_gain()?;
            note_gain.gain().set_value(note.volume);
            
            // Création du panner pour la position stéréo
            let panner = self.context.create_panner()?;
            panner.set_position(f64::from(note.pan), 0.0, 0.0);
            
            // Chaînage : source -> gain -> panner -> eq -> effets
            source.connect_with_audio_node(&note_gain)?;
            note_gain.connect_with_audio_node(&panner)?;
            panner.connect_with_audio_node(&self.eq_low)?;
            
            // Connexion à la delay line si activée
            let settings = self.settings.lock().await;
            if settings.delay_time > 0.0 {
                note_gain.connect_with_audio_node(&self.delay)?;
                self.delay.delay_time().set_value(settings.delay_time);
                self.delay_feedback.gain().set_value(settings.delay_feedback);
            }
            
            // Démarrage de la source
            source.start()?;
            
            // Stockage de la source active
            let mut active_sources = self.active_sources.lock().await;
            active_sources.push(source);
        }
        Ok(())
    }
    
    // Ces méthodes sont conservées pour une utilisation future, mais marquées avec #[allow(dead_code)]
    // pour supprimer les avertissements de compilation
    
    #[allow(dead_code)]
    pub async fn update_settings(&self, settings: AudioSettings) -> Result<(), JsValue> {
        let mut current_settings = self.settings.lock().await;
        *current_settings = settings.clone();
        
        // Mise à jour des paramètres audio
        self.master_gain.gain().set_value(settings.master_volume);
        self.eq_low.gain().set_value(settings.eq_low);
        self.eq_mid.gain().set_value(settings.eq_mid);
        self.eq_high.gain().set_value(settings.eq_high);
        self.delay.delay_time().set_value(settings.delay_time);
        self.delay_feedback.gain().set_value(settings.delay_feedback);
        
        Ok(())
    }
    
    #[allow(dead_code)]
    pub async fn stop_all_notes(&self) -> Result<(), JsValue> {
        let mut active_sources = self.active_sources.lock().await;
        for source in active_sources.iter() {
            // Les deux méthodes stop et stop_with_when sont dépréciées
            // Mais nous devons en utiliser une, alors ignorons l'avertissement
            #[allow(deprecated)]
            source.stop()?;
        }
        active_sources.clear();
        Ok(())
    }
    
    #[allow(dead_code)]
    pub async fn play_metronome_tick(&self, accent: bool) -> Result<(), JsValue> {
        let samples = self.samples.lock().await;
        let midi_note = if accent { 254 } else { 255 };
        
        if let Some(buffer) = samples.get(&midi_note) {
            let source = self.context.create_buffer_source()?;
            source.set_buffer(Some(buffer));

            let click_gain = self.context.create_gain()?;
            click_gain.gain().set_value(if accent { 1.0 } else { 0.8 });

            source.connect_with_audio_node(&click_gain)?;
            click_gain.connect_with_audio_node(&self.metronome_gain)?;

            source.start()?;
        }

        Ok(())
    }

    async fn load_audio_file(&self, path: &str) -> Result<AudioBuffer, JsValue> {
        let opts = RequestInit::new();
        // Utiliser set_method et set_mode au lieu de method et mode (dépréciés)
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(path, &opts)?;
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;

        let array_buffer = JsFuture::from(resp.array_buffer()?).await?;
        let array_buffer: ArrayBuffer = array_buffer.dyn_into()?;

        let buffer = JsFuture::from(self.context.decode_audio_data(&array_buffer)?).await?;
        Ok(buffer.dyn_into::<AudioBuffer>()?) 
    }
}

#[wasm_bindgen]
pub struct HandpanAudio {
    engine: Arc<AudioEngine>,
}

#[wasm_bindgen]
impl HandpanAudio {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<HandpanAudio, JsValue> {
        let engine = Arc::new(AudioEngine::new()?);
        Ok(HandpanAudio { engine })
    }

    pub async fn initialize(&self) -> Result<(), JsValue> {
        self.engine.load_handpan_samples().await?;
        self.engine.load_metronome_samples().await
    }

    pub async fn play_note(&self, midi_note: u8) -> Result<(), JsValue> {
        self.engine.play_note(Note { 
            midi_note,
            pan: 0.0,
            name: String::from("C"),
            octave: 4,
            volume: 1.0
        }).await
    }
}
