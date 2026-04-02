# 🎵 Rust Rhythms Pro - Egui Edition

![Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)
![GStreamer](https://img.shields.io/badge/Powered%20by-GStreamer-blue.svg)
![Egui](https://img.shields.io/badge/UI-Egui-yellow.svg)

**Rust Rhythms Pro** is a high-performance, ultra-lightweight (~6.6MB) premium audio player. Built natively in Rust, it features a powerful GStreamer audio engine, a highly responsive `egui` interface, studio-grade DSP effects, and deep UI customization.

## ✨ Key Features

* **🎧 Advanced Audio Engine:** Powered by GStreamer for flawless playback of MP3, WAV, FLAC, OGG, and M4A.
* **🎛️ Studio-Grade DSP Equalizer:** * 10-Band Graphic Equalizer with a sleek hardware aesthetic.
  * Built-in premium presets (Flat, Bass Boost, Vocal Clear, Electronic).
  * Dedicated Sub-Bass Boost and true 3D Spatial Audio (Room Reflection) effects.
* **🌈 PRO Theme Studio:** Fully customizable dynamic interface. Change background, panel, accent, and text colors on the fly. Settings are automatically saved to `rust_rhythms_theme.cfg`.
* **📊 Real-time Visualizations:** * Poweramp-style 40-band spectrum visualizer with jumping peak caps.
  * "PRO GLOW" reactive LED status strip in the bottom panel that bounces to the beat.
* **📚 Smart Library Management:** Recursively scan folders, search tracks in real-time, and seamlessly load/save `M3U` playlists.
* **⚡ Ultra-Optimized:** Custom release profile (`opt-level = "z"`, `lto = "fat"`, `strip = true`) resulting in a tiny, lightning-fast binary.

## 🚀 Getting Started

### Prerequisites
Ensure you have Rust installed along with the GStreamer development libraries for your operating system.

**Ubuntu / Debian / Linux Mint:**
```bash
sudo apt-get install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
    gstreamer1.0-plugins-base gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav libglib2.0-dev
