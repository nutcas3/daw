# My DAW - A Digital Audio Workstation in Rust

A professional-grade DAW built entirely in Rust, following the phased roadmap from the TODO list.

## 🎯 Project Status

Currently implementing **Phase 1: Foundation & Audio Playback**

### Completed
- ✅ Workspace structure with modular crates
- ✅ Core data models (Project, Track, Clip, MIDI)
- ✅ Real-time audio engine foundation
- ✅ Lock-free UI-to-Engine command queue
- ✅ Basic egui GUI framework
- ✅ Project serialization/deserialization

### In Progress
- 🔨 Audio playback from files
- 🔨 Transport controls (Play/Pause/Stop)

## 📂 Architecture

```
my_daw/
├── crates/
│   ├── daw-core/       # Pure data models (Project, Track, Clip, MIDI)
│   ├── daw-engine/     # Real-time audio processing (CRITICAL: no allocations!)
│   ├── daw-ui/         # GUI using egui (runs in main thread)
│   ├── daw-vst/        # Plugin hosting (Phase 7)
│   └── daw-state/      # Project save/load (Phase 8)
└── src/
    └── main.rs         # Binary entry point
```

### Key Design Principles

1. **Thread Separation**: Real-time audio thread is separate from GUI thread
2. **Lock-Free Communication**: Command queue using `ringbuf` for UI → Engine messages
3. **Real-Time Safety**: Audio callback has NO allocations, locks, or syscalls
4. **Modular Design**: Each crate is independently compilable

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Audio drivers (ASIO on Windows, CoreAudio on macOS, ALSA on Linux)

### Building

```bash
# Clone the repository
git clone <your-repo>
cd my_daw

# Build in debug mode
cargo build

# Build optimized release (MUCH better audio performance)
cargo build --release

# Run the DAW
cargo run --release
```

### Development

```bash
# Run with logging
RUST_LOG=debug cargo run

# Check all crates
cargo check --workspace

# Run tests
cargo test --workspace

# Build documentation
cargo doc --no-deps --open
```

## 🧩 Crate Overview

### `daw-core` - Data Models
Pure data structures with no dependencies on audio or GUI.

**Key types:**
- `Project` - Top-level project container
- `Track` - Audio or MIDI track
- `Clip` - Audio region or MIDI pattern
- `MidiNote` / `MidiEvent` - MIDI data
- `TimeSignature` / `TempoMap` - Timing

### `daw-engine` - Audio Engine
Real-time audio processing. **CRITICAL:** This code runs in a high-priority thread.

**Rules:**
- ❌ NO `Vec::new()`, `Box::new()`, `String`
- ❌ NO locks (use `try_lock` only)
- ❌ NO file I/O or network calls
- ✅ Use `Atomic` types for thread-safe parameters
- ✅ Pre-allocate all buffers

**Key components:**
- `AudioEngine` - Main engine state
- `Mixer` - Track summing with real-time safe controls
- `CommandQueue` - Lock-free UI → Engine messaging
- `AudioDevice` - `cpal` abstraction

### `daw-ui` - User Interface
GUI using `egui`. Runs in main thread, sends commands to engine.

**Key components:**
- `DawApp` - Main application window
- `widgets/` - Custom UI widgets (piano roll, waveform, etc.)

### `daw-vst` - Plugin Hosting
VST3/AU plugin hosting using the `rack` crate (Phase 7).

### `daw-state` - Persistence
Project save/load, autosave, crash recovery.

## 📋 TODO Phases

Following the comprehensive TODO list:

- [x] **Phase 1**: Foundation & Audio Playback
- [ ] **Phase 2**: Multi-Track Mixer
- [ ] **Phase 3**: MIDI Sequencing & Synthesis
- [ ] **Phase 4**: GUI with Piano Roll
- [ ] **Phase 5**: Audio Recording & Disk Streaming
- [ ] **Phase 6**: Effects & Signal Processing
- [ ] **Phase 7**: Plugin Hosting (VST/AU)
- [ ] **Phase 8**: Project Management & Export
- [ ] **Phase 9**: Polish & Performance

See `docs/TODO.md` for the complete checklist.

## 🔧 Dependencies

### Audio
- `cpal` - Cross-platform audio I/O
- `symphonium` - Audio file loading
- `creek` - Disk streaming (Phase 5)
- `wmidi` - MIDI handling

### GUI
- `egui` / `eframe` - Immediate-mode GUI

### Concurrency
- `parking_lot` - Fast locks (used sparingly)
- `crossbeam` - Channel utilities
- `ringbuf` - Lock-free ring buffer

### Serialization
- `serde` / `serde_json` - Project file format

## 🎵 Real-Time Safety

The audio callback is the most critical code in the DAW. It must:

1. **Never allocate memory**: All buffers pre-allocated at startup
2. **Never block**: Use `try_lock()`, never `lock()`
3. **Be deterministic**: No syscalls, no unpredictable operations
4. **Handle failures gracefully**: Return silence, log errors asynchronously

Example of **correct** real-time code:
```rust
pub fn process(&mut self, output: &mut [f32]) {
    // Process commands (non-blocking)
    while let Some(cmd) = self.command_queue.try_pop() {
        self.execute_command(cmd);
    }
    
    // Generate audio
    for sample in output.iter_mut() {
        *sample = self.generate_sample();  // Pure computation
    }
}
```

Example of **incorrect** real-time code:
```rust
pub fn process(&mut self, output: &mut [f32]) {
    let buffer = Vec::new();  // ❌ ALLOCATION!
    
    let state = self.state.lock();  // ❌ CAN BLOCK!
    
    std::fs::read("file.wav");  // ❌ SYSCALL!
}
```

## 📖 Resources

- [Rust Audio Discord](https://discord.gg/rust-audio)
- [Meadowlark DAW](https://github.com/MeadowlarkDAW/Meadowlark) - Reference implementation
- [cpal documentation](https://docs.rs/cpal)
- Original roadmap documents (in repository)

## 📄 License

(Add your chosen license here)

## 🤝 Contributing

Contributions welcome! Please:

1. Follow the phased TODO list
2. Maintain real-time safety in audio code
3. Add tests for new functionality
4. Document public APIs

## 🙏 Acknowledgments

Built using the Rust audio ecosystem:
- `cpal` team for cross-platform audio
- Meadowlark DAW for inspiration and creek/symphonium crates
- Rust Audio community
