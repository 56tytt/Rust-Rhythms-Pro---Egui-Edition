use gst::prelude::*;
use gstreamer as gst;
use gstreamer::glib;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerState {
    Stopped,
    Loading,
    Playing,
    Paused,
}

#[derive(Debug, Clone)]
pub enum AudioCommand {
    LoadFile(String),
    Play,
    Pause,
    Stop,
    SetVolume(f64),
    Seek(f32),
    Shutdown,
    SetEq(usize, f64),
    SetSpatialAudio(bool), // Toggles the stereowiden effect
}

#[derive(Debug, Clone)]
pub enum AudioStatus {
    StateChanged(PlayerState),
    PositionUpdated(f64),
    DurationUpdated(f64),
    Error(String),
    EndOfStream,
}

pub struct AudioEngine {
    command_tx: Sender<AudioCommand>,
    event_rx: Receiver<AudioStatus>,
    worker: Option<JoinHandle<()>>,

    pub current_state: PlayerState,
    pub current_duration: f64,
    pub current_position: f64,
    pub spectrum_data: Arc<Mutex<Vec<f32>>>,
}

impl AudioEngine {
    pub fn new_headless() -> Self {
        gst::init().expect("Failed to init GStreamer");

        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            run_loop(cmd_rx, event_tx);
        });

        Self {
            command_tx: cmd_tx,
            event_rx,
            worker: Some(handle),
            current_state: PlayerState::Stopped,
            current_duration: 0.0,
            current_position: 0.0,
            spectrum_data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn load(&self, path: &str) {
        let uri = if path.starts_with("file://") {
            path.to_string()
        } else {
            match glib::filename_to_uri(path, None) {
                Ok(u) => u.to_string(),
                Err(e) => {
                    eprintln!("URI conversion error: {}", e);
                    return;
                }
            }
        };
        let _ = self.command_tx.send(AudioCommand::LoadFile(uri));
    }

    pub fn play(&self) {
        let _ = self.command_tx.send(AudioCommand::Play);
    }

    pub fn pause(&self) {
        let _ = self.command_tx.send(AudioCommand::Pause);
    }

    // ==========================================
    // DSP & Equalizer Bridge
    // ==========================================
    pub fn set_equalizer(&mut self, bands: &[f32; 10], bass_boost: f32) {
        // The bass boost slider is 0.0 to 1.0. We convert this to up to +12.0 dB of raw sub-bass power.
        let extra_bass_db = bass_boost * 12.0;

        for (i, val) in bands.iter().enumerate() {
            let mut final_gain = *val as f64;

            // Inject the extra Bass Boost into Band 0 (32Hz) and Band 1 (64Hz)
            if i == 0 || i == 1 {
                final_gain += extra_bass_db as f64;
            }

            // Fire the specific band update to the GStreamer engine
            let _ = self.command_tx.send(AudioCommand::SetEq(i, final_gain));
        }
    }

    pub fn set_spatial_audio(&self, enabled: bool) {
        let _ = self.command_tx.send(AudioCommand::SetSpatialAudio(enabled));
    }

    pub fn set_volume(&self, volume_percent: f32) {
        let v = (volume_percent as f64).clamp(0.0, 1.0);
        let _ = self.command_tx.send(AudioCommand::SetVolume(v));
    }

    pub fn seek(&self, percent: f32) {
        let percent = percent.clamp(0.0, 100.0);
        let _ = self.command_tx.send(AudioCommand::Seek(percent));
    }

    pub fn update(&mut self) -> bool {
        let mut finished = false;

        loop {
            match self.event_rx.try_recv() {
                Ok(AudioStatus::PositionUpdated(p)) => self.current_position = p,
                Ok(AudioStatus::DurationUpdated(d)) => self.current_duration = d,
                Ok(AudioStatus::StateChanged(s)) => self.current_state = s,
                Ok(AudioStatus::EndOfStream) => {
                    self.current_state = PlayerState::Stopped;
                    self.current_position = 0.0;
                    finished = true;
                }
                Ok(AudioStatus::Error(e)) => eprintln!("Audio error: {}", e),
                Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => break,
            }
        }

        // Smooth Visualization Physics
        if self.current_state == PlayerState::Playing {
            if let Ok(mut data) = self.spectrum_data.lock() {
                if data.len() != 40 {
                    *data = vec![0.0; 40];
                }

                let time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs_f64();

                for i in 0..40 {
                    let x = i as f64 * 0.2;
                    let wave1 = (time * 8.0 + x).sin();
                    let wave2 = (time * 4.0 - x * 1.5).cos();
                    let wave3 = ((time * 2.0) + (i as f64 * 0.5)).sin();

                    let combined = (wave1 + wave2 + wave3).abs() / 3.0;
                    let target_val = -50.0 + (combined * 50.0);
                    data[i] = data[i] * 0.8 + (target_val as f32) * 0.2;
                }
            }
        }

        finished
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        let _ = self.command_tx.send(AudioCommand::Shutdown);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

// =========================================================
// Run Loop - GStreamer Background Engine
// =========================================================
fn run_loop(cmd_rx: Receiver<AudioCommand>, event_tx: Sender<AudioStatus>) {
    let pipeline = match gst::ElementFactory::make("playbin").build() {
        Ok(p) => p,
        Err(e) => {
            let _ = event_tx.send(AudioStatus::Error(format!(
                "Failed to create playbin: {}",
                e
            )));
            return;
        }
    };

    let fakesink = gst::ElementFactory::make("fakesink")
        .build()
        .expect("Failed to create fakesink");
    pipeline.set_property("video-sink", &fakesink);
    fakesink.set_property("sync", true);

    // --- 🚀 DSP Audio Filter Bin Creation ---
    let dsp_bin = gst::Bin::new();

    let equalizer = gst::ElementFactory::make("equalizer-10bands")
        .build()
        .expect("Missing equalizer plugin");
    // Switch to 'audioecho' for true Room Spatial Audio without phase distortion
    let spatial_fx = gst::ElementFactory::make("audioecho")
        .build()
        .expect("Missing audioecho plugin");

    dsp_bin
        .add_many(&[&equalizer, &spatial_fx])
        .expect("Failed to add elements to bin");
    gst::Element::link_many(&[&equalizer, &spatial_fx]).expect("Failed to link elements");

    // Expose GhostPads
    let sink_pad = equalizer
        .static_pad("sink")
        .expect("Failed to get sink pad");
    dsp_bin
        .add_pad(&gst::GhostPad::with_target(&sink_pad).unwrap())
        .unwrap();

    let src_pad = spatial_fx.static_pad("src").expect("Failed to get src pad");
    dsp_bin
        .add_pad(&gst::GhostPad::with_target(&src_pad).unwrap())
        .unwrap();

    // 3D Studio Room Configuration (Early Reflections)
    spatial_fx.set_property("delay", 20_000_000u64); // 20ms delay mimics room walls
    spatial_fx.set_property("max-delay", 20_000_000u64);
    spatial_fx.set_property("feedback", 0.05f32); // Almost no tail, just width
    spatial_fx.set_property("intensity", 0.0f32); // OFF by default

    pipeline.set_property("audio-filter", &dsp_bin);
    // ------------------------------------------

    let bus = match pipeline.bus() {
        Some(b) => b,
        None => return,
    };

    let mut current_state = PlayerState::Stopped;
    let mut last_update = std::time::Instant::now();

    loop {
        // --- Process Commands ---
        while let Ok(cmd) = cmd_rx.try_recv() {
            match cmd {
                AudioCommand::LoadFile(uri) => {
                    let _ = pipeline.set_state(gst::State::Ready);
                    pipeline.set_property("uri", &uri);
                    let _ = pipeline.set_state(gst::State::Playing);
                }
                AudioCommand::Play => {
                    let _ = pipeline.set_state(gst::State::Playing);
                }
                AudioCommand::Pause => {
                    let _ = pipeline.set_state(gst::State::Paused);
                }
                AudioCommand::Stop => {
                    let _ = pipeline.set_state(gst::State::Null);
                    current_state = PlayerState::Stopped;
                    let _ = event_tx.send(AudioStatus::StateChanged(current_state.clone()));
                }
                AudioCommand::SetVolume(v) => {
                    pipeline.set_property("volume", v);
                }
                AudioCommand::SetEq(band_idx, gain) => {
                    let prop_name = format!("band{}", band_idx);
                    let safe_gain = gain.clamp(-24.0, 12.0); // Clamp to prevent distortion
                    equalizer.set_property(&prop_name, safe_gain);
                }

                AudioCommand::SetSpatialAudio(enabled) => {
                    // Mix in 15% of the room reflection for the 3D effect. 0.0 turns it off completely.
                    let mix = if enabled { 0.15f32 } else { 0.0f32 };
                    spatial_fx.set_property("intensity", mix);
                    println!(
                        "🎧 3D SPATIAL AUDIO: {}",
                        if enabled { "ON" } else { "OFF" }
                    );
                }

                AudioCommand::Seek(percent) => {
                    if let Some(dur) = pipeline.query_duration::<gst::ClockTime>() {
                        let target_ns = (dur.nseconds() as f64 * (percent as f64 / 100.0)) as u64;
                        let _ = pipeline.seek_simple(
                            gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                            gst::ClockTime::from_nseconds(target_ns),
                        );
                    }
                }
                AudioCommand::Shutdown => {
                    let _ = pipeline.set_state(gst::State::Null);
                    return;
                }
            }
        }

        // --- Process GStreamer Bus Messages ---
        if let Some(msg) = bus.timed_pop(gst::ClockTime::from_mseconds(30)) {
            use gst::MessageView;
            match msg.view() {
                MessageView::Eos(..) => {
                    let _ = pipeline.set_state(gst::State::Ready);
                    current_state = PlayerState::Stopped;
                    let _ = event_tx.send(AudioStatus::EndOfStream);
                }
                MessageView::DurationChanged(..) => {
                    if let Some(dur) = pipeline.query_duration::<gst::ClockTime>() {
                        let _ = event_tx.send(AudioStatus::DurationUpdated(dur.seconds() as f64));
                    }
                }
                MessageView::StateChanged(s) => {
                    if s.src()
                        .map(|src| src == pipeline.upcast_ref::<gst::Object>())
                        .unwrap_or(false)
                    {
                        let new_state = match s.current() {
                            gst::State::Playing => PlayerState::Playing,
                            gst::State::Paused => PlayerState::Paused,
                            _ => PlayerState::Stopped,
                        };
                        if new_state != current_state {
                            current_state = new_state.clone();
                            let _ = event_tx.send(AudioStatus::StateChanged(current_state.clone()));
                        }
                    }
                }
                _ => {}
            }
        }

        // --- Track Progress Update ---
        if current_state == PlayerState::Playing && last_update.elapsed().as_millis() > 100 {
            if let Some(pos) = pipeline.query_position::<gst::ClockTime>() {
                let _ = event_tx.send(AudioStatus::PositionUpdated(pos.seconds() as f64));
                last_update = std::time::Instant::now();
            }
            if let Some(dur) = pipeline.query_duration::<gst::ClockTime>() {
                let _ = event_tx.send(AudioStatus::DurationUpdated(dur.seconds() as f64));
            }
        }
    }
}
