# PinReady

[English](#english) | [Français](#français)

---

## English

Cross-platform configurator and launcher for [VPinballX](https://github.com/vpinball/vpinball) standalone (10.8.1).

PinReady replaces the non-existent native configuration tools for VPX standalone builds (SDL3/bgfx). It guides you through setting up a virtual pinball cabinet from scratch: screens, inputs, tilt, audio, then lets you browse and launch tables from a single interface.

### Features

**Configuration wizard (first run)**

- **VPinballX auto-install** -- Automatically download and install the correct VPinballX build for your platform (Linux/macOS/Windows, x64/arm64/SBC)
- **Screen assignment** -- Detect displays via SDL3, auto-assign roles (Playfield, Backglass, DMD, Topper) by size, configure multi-screen positioning and cabinet physical dimensions
- **Rendering** -- Anti-aliasing, FXAA, sharpening, reflections, texture limits, sync mode, max framerate
- **Input mapping** -- Capture keyboard and joystick bindings for all VPX actions, auto-detect Pinscape/KL25Z controllers, conflict warnings
- **Tilt & nudge** -- Configure accelerometer sensitivity with simplified or advanced controls
- **Audio routing** -- Assign playfield and backglass audio devices, configure SSF surround modes (6 modes), test speaker wiring with built-in audio sequences (music, ball sounds, knocker)
- **Tables directory** -- Select the root folder containing your tables (folder-per-table layout)
- **Internationalization** -- 20+ languages, including CJK, Arabic, Thai, Hindi

**Table launcher (subsequent runs)**

- **Table browser** -- Scan folder-per-table directories, display backglass thumbnails extracted from `.directb2s` files
- **Multi-screen layout** -- Table selector on DMD, backglass preview on BG display
- **VPX integration** -- Launch tables with loading progress overlay, parse VPX stdout for real-time status
- **Auto-update** -- Checks for new VPinballX releases on startup, one-click update from the launcher
- **Input navigation** -- Browse and launch tables with joystick (flippers, start) or keyboard

### Target

- **VPinballX 10.8.1** -- Uses the folder-per-table layout
- **Cross-platform** -- Linux, macOS, Windows. SDL3 only, no platform-specific APIs
- **No system dependencies** -- SDL3 and SQLite are statically linked

### Build

**Linux:**

```bash
sudo apt install build-essential cmake pkg-config \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev

cargo build --release
```

**macOS / Windows:**

```bash
cargo build --release
```

SDL3 and SQLite compile from source automatically -- no manual installation needed.

### Usage

**First run** (no existing database) launches the configuration wizard.
**Subsequent runs** go directly to the table launcher. The wizard can be re-launched at any time.

```bash
# Run with debug logging
RUST_LOG=info cargo run

# Or run the release binary directly
./target/release/pinready
```

**Requirements:**

- **VPinballX** executable (10.8.1+) -- auto-installed or path configured in the wizard
- **Tables directory** -- folder-per-table layout as described in VPX docs
- **Internet connection** -- required for auto-install and update checks (optional for manual install)

**Launcher controls:**

| Action | Keyboard | Joystick |
|---|---|---|
| Navigate tables | Arrow keys | Left/Right flipper (prev/next), Staged flippers (row up/down) |
| Launch table | Enter | Start button |
| Open config | -- | Launch Ball button |
| Quit launcher | Escape | ExitGame button |

---

## Français

Configurateur et lanceur multiplateforme pour [VPinballX](https://github.com/vpinball/vpinball) standalone (10.8.1).

PinReady remplace les outils de configuration natifs inexistants pour les builds VPX standalone (SDL3/bgfx). Il vous guide dans la mise en place d'un flipper virtuel depuis zero : ecrans, controles, tilt, audio, puis permet de parcourir et lancer vos tables depuis une interface unique.

### Fonctionnalites

**Assistant de configuration (premier lancement)**

- **Installation automatique de VPinballX** -- Telecharge et installe automatiquement le bon build VPinballX pour votre plateforme (Linux/macOS/Windows, x64/arm64/SBC)
- **Affectation des ecrans** -- Detection des ecrans via SDL3, affectation automatique des roles (Playfield, Backglass, DMD, Topper) par taille, configuration du positionnement multi-ecran et des dimensions physiques du cabinet
- **Rendu** -- Anti-aliasing, FXAA, nettete, reflets, limites de texture, mode sync, framerate max
- **Mapping des controles** -- Capture des touches clavier et boutons joystick pour toutes les actions VPX, detection automatique des controleurs Pinscape/KL25Z, avertissements de conflits
- **Tilt & nudge** -- Configuration de la sensibilite de l'accelerometre en mode simplifie ou avance
- **Routage audio** -- Affectation des peripheriques audio playfield et backglass, configuration des modes surround SSF (6 modes), test du cablage des enceintes avec sequences audio integrees (musique, bruits de bille, knocker)
- **Repertoire des tables** -- Selection du dossier racine contenant vos tables (format dossier-par-table)
- **Internationalisation** -- 20+ langues, dont CJK, arabe, thai, hindi

**Lanceur de tables (lancements suivants)**

- **Navigateur de tables** -- Scan des repertoires dossier-par-table, affichage des miniatures backglass extraites des fichiers `.directb2s`
- **Affichage multi-ecran** -- Selecteur de table sur le DMD, apercu du backglass sur l'ecran BG
- **Integration VPX** -- Lancement des tables avec overlay de progression, lecture du stdout VPX pour le statut en temps reel
- **Mise a jour automatique** -- Verifie les nouvelles releases VPinballX au demarrage, mise a jour en un clic depuis le lanceur
- **Navigation aux controles** -- Parcourir et lancer les tables au joystick (flippers, start) ou au clavier

### Cible

- **VPinballX 10.8.1** -- Utilise le format dossier-par-table
- **Multiplateforme** -- Linux, macOS, Windows. SDL3 uniquement, aucune API specifique a une plateforme
- **Aucune dependance systeme** -- SDL3 et SQLite sont lies statiquement

### Compilation

**Linux :**

```bash
sudo apt install build-essential cmake pkg-config \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev

cargo build --release
```

**macOS / Windows :**

```bash
cargo build --release
```

SDL3 et SQLite se compilent depuis les sources automatiquement -- aucune installation manuelle necessaire.

### Utilisation

**Premier lancement** (pas de base de donnees existante) : lance l'assistant de configuration.
**Lancements suivants** : acces direct au lanceur de tables. L'assistant peut etre relance a tout moment.

```bash
# Lancer avec les logs de debug
RUST_LOG=info cargo run

# Ou lancer directement le binaire release
./target/release/pinready
```

**Prerequis :**

- **VPinballX** executable (10.8.1+) -- installe automatiquement ou chemin configure dans l'assistant
- **Repertoire de tables** -- format dossier-par-table tel que decrit dans la doc VPX
- **Connexion internet** -- necessaire pour l'installation automatique et la verification des mises a jour (optionnel pour l'installation manuelle)

**Controles du lanceur :**

| Action | Clavier | Joystick |
|---|---|---|
| Naviguer les tables | Fleches | Flippers gauche/droit (precedent/suivant), Staged flippers (ligne haut/bas) |
| Lancer une table | Entree | Bouton Start |
| Ouvrir la config | -- | Bouton Launch Ball |
| Quitter le lanceur | Echap | Bouton ExitGame |

---

## Architecture

```
src/
  main.rs       Entry point, first-run detection, eframe launch
  app/          Main App struct, page routing, wizard & launcher UI
  screens.rs    SDL3 display enumeration + role assignment
  inputs.rs     Input mapping with SDL3 event loop on dedicated thread
  tilt.rs       Tilt/nudge sensitivity configuration
  audio.rs      Audio device detection + routing + test sequences
  assets.rs     Backglass extraction from directb2s files
  config.rs     VPinballX.ini read/write (format-preserving)
  db.rs         SQLite catalog
  updater.rs    VPinballX release check, download, install
```

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
| Archive | `zip` + `flate2` + `tar` | Release extraction |
| i18n | `rust-i18n` + `noto-fonts-dl` | 20+ languages with font support |

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

## License

GPL-3.0-or-later
