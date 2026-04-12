# PinReady

Cross-platform configurator and launcher for [VPinballX](https://github.com/vpinball/vpinball) standalone (10.8.1).

PinReady replaces the non-existent native configuration tools for VPX standalone builds (SDL3/bgfx). It handles screen assignment, input mapping, tilt/nudge sensitivity, audio routing, table browsing and asset management.

## Features

### Configuration wizard

- **VPinballX auto-install** -- Automatically download and install the correct VPinballX build for your platform (Linux/macOS/Windows, x64/arm64/SBC)
- **Screen assignment** -- Detect displays via SDL3, auto-assign roles (Playfield, Backglass, DMD, Topper) by size, configure multi-screen positioning
- **Input mapping** -- Capture keyboard and joystick bindings for all VPX actions, auto-detect Pinscape/KL25Z controllers
- **Tilt & nudge** -- Configure accelerometer sensitivity with simplified or advanced controls
- **Audio routing** -- Assign playfield and backglass audio devices, configure SSF surround modes, test speaker wiring with built-in audio sequences

### Table launcher

- **Table browser** -- Scan folder-per-table directories, display backglass thumbnails extracted from `.directb2s` files
- **Multi-screen layout** -- Table selector on DMD, backglass preview on BG display, cover screens on unused displays
- **VPX integration** -- Launch tables with loading progress overlay, parse VPX stdout for real-time status
- **Auto-update** -- Checks for new VPinballX releases on startup, one-click update from the launcher
- **Input navigation** -- Browse and launch tables with joystick (flippers, start) or keyboard

## Target

- **VPinballX 10.8.1** -- Uses the folder-per-table layout
- **Cross-platform** -- Linux, macOS, Windows. SDL3 only, no platform-specific APIs
- **No system dependencies** -- SDL3 and SQLite are statically linked

## Stack

| Layer | Crate | Role |
|---|---|---|
| UI | `eframe` + `egui` | Immediate mode GUI |
| Display/Input | `sdl3-sys` (build-from-source-static) | Screen enumeration, input capture |
| Config | `ini-preserve` | Read/write VPinballX.ini |
| Database | `rusqlite` (bundled) | Local table catalog |
| Images | `image` + `directb2s` | Backglass thumbnail extraction |
| Audio | `symphonia` | OGG/Vorbis decode for SDL3 playback |
| HTTP | `ureq` | GitHub API + release download |
| Archive | `zip` | Release extraction |

## Build

### Linux

```bash
sudo apt install build-essential cmake pkg-config \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev

cargo build --release
```

### macOS / Windows

```bash
cargo build --release
```

SDL3 and SQLite compile from source automatically -- no manual installation needed.

## Usage

**First run** (no existing database) launches the configuration wizard.
**Subsequent runs** go directly to the table launcher. The wizard can be re-launched at any time.

```bash
# Run with debug logging
RUST_LOG=info cargo run

# Or run the release binary directly
./target/release/pinready
```

### Requirements

- **VPinballX** executable (10.8.1+) -- auto-installed or path configured in the wizard
- **Tables directory** -- folder-per-table layout as described in VPX docs
- **Internet connection** -- required for auto-install and update checks (optional for manual install)

### Launcher controls

| Action | Keyboard | Joystick |
|---|---|---|
| Navigate tables | Arrow keys | Left/Right flipper (prev/next), Staged flippers (row up/down) |
| Launch table | Enter | Start button |
| Open config | -- | Launch Ball button |
| Quit launcher | Escape | ExitGame button |

## Architecture

```
src/
  main.rs       Entry point, first-run detection, eframe launch
  app.rs        Main App struct, page routing, launcher
  screens.rs    SDL3 display enumeration + role assignment
  inputs.rs     Input mapping with SDL3 event loop on dedicated thread
  tilt.rs       Tilt/nudge sensitivity configuration
  audio.rs      Audio device detection + routing + test sequences
  assets.rs     Backglass extraction from directb2s files
  config.rs     VPinballX.ini read/write (format-preserving)
  db.rs         SQLite catalog
  updater.rs    VPinballX release check, download, install
```

## VPinballX fork management

The `vpinball-fork.sh` script manages a personal fork of [vpinball/vpinball](https://github.com/vpinball/vpinball) for building VPinballX. It keeps CI workflows set to manual dispatch so builds only run when you decide.

Releases created by this script are automatically detected by PinReady clients, which can download and install the correct build for their platform.

### Prerequisites

- [gh CLI](https://cli.github.com) installed and authenticated (`gh auth login`)
- `jq` installed (`sudo apt install jq`)
- A fork of `vpinball/vpinball` on your GitHub account

### Workflow

```bash
# 1. Sync fork with upstream + patch CI + trigger builds
./vpinball-fork.sh sync

# 2. Monitor build progress
./vpinball-fork.sh status

# 3. Test the build manually on your pincab

# 4. When validated, create a GitHub Release (clients will auto-detect it)
./vpinball-fork.sh release
```

### Commands

| Command | Action |
|---|---|
| `sync` | Force-reset fork to upstream HEAD, patch workflows to `workflow_dispatch`, trigger `vpinball` + `vpinball-sbc` builds |
| `release` | Wait for both builds to succeed, run `prerelease` workflow to create a GitHub Release, upload SBC artifacts |
| `status` | Show recent workflow runs and latest release info |

### How it works

1. **sync** resets the fork's master branch to match upstream exactly, then commits two patches that change the CI trigger from `push` to `workflow_dispatch`. This prevents builds from running on every upstream sync. Finally, it dispatches both build workflows.

2. **release** waits for the `vpinball` and `vpinball-sbc` workflows to complete successfully, then triggers the `prerelease` workflow which creates a GitHub Release with all PC artifacts (Windows, macOS, Linux). SBC artifacts (RPi, RK3588) are then uploaded to the same release.

3. PinReady clients check this release on startup and offer one-click updates.

### Validated configuration

Builds are tested on: **Ubuntu 24.04 LTS, X11, 3-screen PinCab**. Other platforms are built but not validated.

## License

GPL-3.0-or-later
