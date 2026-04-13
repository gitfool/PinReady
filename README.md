# 🎯 PinReady

🇬🇧 [English](#-english) | 🇫🇷 [Français](#-français)

---

## 🇬🇧 English

Cross-platform configurator and launcher for [Visual Pinball](https://github.com/vpinball/vpinball) standalone (10.8.1).

PinReady replaces the non-existent native configuration tools for VPX standalone builds (SDL3/bgfx). It guides you through setting up a virtual pinball cabinet from scratch: screens, inputs, tilt, audio, then lets you browse and launch tables from a single interface. 🕹️

### ✨ Features

**🧙 Configuration wizard (first run)**

- 📥 **Visual Pinball auto-install** -- Automatically download and install the correct Visual Pinball build for your platform (Linux/macOS/Windows, x64/arm64/SBC)
- 🖥️ **Screen assignment** -- Detect displays via SDL3, auto-assign roles (Playfield, Backglass, DMD, Topper) by size, configure multi-screen positioning and cabinet physical dimensions
- 🎨 **Rendering** -- Anti-aliasing, FXAA, sharpening, reflections, texture limits, sync mode, max framerate
- 🎮 **Input mapping** -- Capture keyboard and joystick bindings for all VPX actions, auto-detect Pinscape/KL25Z controllers, conflict warnings
- 📐 **Tilt & nudge** -- Configure accelerometer sensitivity with simplified or advanced controls
- 🔊 **Audio routing** -- Assign playfield and backglass audio devices, configure SSF surround modes (6 modes), test speaker wiring with built-in audio sequences (music, ball sounds, knocker)
- 📁 **Tables directory** -- Select the root folder containing your tables (folder-per-table layout)
- 🌍 **Internationalization** -- 20+ languages: 🇬🇧 🇫🇷 🇩🇪 🇪🇸 🇮🇹 🇵🇹 🇳🇱 🇸🇪 🇫🇮 🇵🇱 🇨🇿 🇸🇰 🇷🇺 🇹🇷 🇸🇦 🇮🇳 🇧🇩 🇹🇭 🇻🇳 🇮🇩 🇰🇪 🇨🇳 🇹🇼 🇯🇵 🇰🇷

**🚀 Table launcher (subsequent runs)**

- 🗂️ **Table browser** -- Scan folder-per-table directories, display backglass thumbnails extracted from `.directb2s` files
- 📺 **Multi-screen layout** -- Table selector on DMD, backglass preview on BG display
- ⚡ **VPX integration** -- Launch tables with loading progress overlay, parse VPX stdout for real-time status
- 🔄 **Auto-update** -- Checks for new Visual Pinball releases on startup, one-click update from the launcher
- 🕹️ **Input navigation** -- Browse and launch tables with joystick (flippers, start) or keyboard

### 🎯 Target

- 🎰 **Visual Pinball 10.8.1** -- Uses the folder-per-table layout
- 💻 **Cross-platform** -- Linux, macOS, Windows. SDL3 only, no platform-specific APIs
- 📦 **No system dependencies** -- SDL3 and SQLite are statically linked

### 📥 Download

Grab the latest release for your platform -- no install needed, just download and run:

👉 **[Download PinReady](https://github.com/Le-Syl21/PinReady/releases/latest)** (Linux, macOS, Windows)

### 🔨 Build from source

If you prefer to compile it yourself:

**🐧 Linux:**

```bash
sudo apt install build-essential cmake pkg-config \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev

cargo build --release
```

**🍎 macOS / 🪟 Windows:**

```bash
cargo build --release
```

SDL3 and SQLite compile from source automatically -- no manual installation needed. ✅

### 🚀 Usage

**First run** (no existing database) launches the configuration wizard.
**Subsequent runs** go directly to the table launcher. The wizard can be re-launched at any time.

```bash
# Run with debug logging
RUST_LOG=info cargo run

# Or run the release binary directly
./target/release/pinready
```

**📋 Requirements:**

- 🎰 **Visual Pinball** executable (10.8.1+) -- auto-installed or path configured in the wizard
- 📁 **Tables directory** -- folder-per-table layout as described in VPX docs
- 🌐 **Internet connection** -- required for auto-install and update checks (optional for manual install)

**🎮 Launcher controls:**

| Action | ⌨️ Keyboard | 🕹️ Joystick |
|---|---|---|
| Navigate tables | Arrow keys | Left/Right flipper (prev/next), Staged flippers (row up/down) |
| Launch table | Enter | Start button |
| Open config | -- | Launch Ball button |
| Quit launcher | Escape | ExitGame button |

---

## 🇫🇷 Français

Configurateur et lanceur multiplateforme pour [Visual Pinball](https://github.com/vpinball/vpinball) standalone (10.8.1).

PinReady remplace les outils de configuration natifs inexistants pour les builds VPX standalone (SDL3/bgfx). Il vous guide dans la mise en place d'un flipper virtuel depuis zéro : écrans, contrôles, tilt, audio, puis permet de parcourir et lancer vos tables depuis une interface unique. 🕹️

### ✨ Fonctionnalités

**🧙 Assistant de configuration (premier lancement)**

- 📥 **Installation automatique de Visual Pinball** -- Télécharge et installe automatiquement le bon build Visual Pinball pour votre plateforme (Linux/macOS/Windows, x64/arm64/SBC)
- 🖥️ **Affectation des écrans** -- Détection des écrans via SDL3, affectation automatique des rôles (Playfield, Backglass, DMD, Topper) par taille, configuration du positionnement multi-écran et des dimensions physiques du cabinet
- 🎨 **Rendu** -- Anti-aliasing, FXAA, netteté, reflets, limites de texture, mode sync, framerate max
- 🎮 **Mapping des contrôles** -- Capture des touches clavier et boutons joystick pour toutes les actions VPX, détection automatique des contrôleurs Pinscape/KL25Z, avertissements de conflits
- 📐 **Tilt & nudge** -- Configuration de la sensibilité de l'accéléromètre en mode simplifié ou avancé
- 🔊 **Routage audio** -- Affectation des périphériques audio playfield et backglass, configuration des modes surround SSF (6 modes), test du câblage des enceintes avec séquences audio intégrées (musique, bruits de bille, knocker)
- 📁 **Répertoire des tables** -- Sélection du dossier racine contenant vos tables (format dossier-par-table)
- 🌍 **Internationalisation** -- 20+ langues : 🇬🇧 🇫🇷 🇩🇪 🇪🇸 🇮🇹 🇵🇹 🇳🇱 🇸🇪 🇫🇮 🇵🇱 🇨🇿 🇸🇰 🇷🇺 🇹🇷 🇸🇦 🇮🇳 🇧🇩 🇹🇭 🇻🇳 🇮🇩 🇰🇪 🇨🇳 🇹🇼 🇯🇵 🇰🇷

**🚀 Lanceur de tables (lancements suivants)**

- 🗂️ **Navigateur de tables** -- Scan des répertoires dossier-par-table, affichage des miniatures backglass extraites des fichiers `.directb2s`
- 📺 **Affichage multi-écran** -- Sélecteur de table sur le DMD, aperçu du backglass sur l'écran BG
- ⚡ **Intégration VPX** -- Lancement des tables avec overlay de progression, lecture du stdout VPX pour le statut en temps réel
- 🔄 **Mise à jour automatique** -- Vérifie les nouvelles releases Visual Pinball au démarrage, mise à jour en un clic depuis le lanceur
- 🕹️ **Navigation aux contrôles** -- Parcourir et lancer les tables au joystick (flippers, start) ou au clavier

### 🎯 Cible

- 🎰 **Visual Pinball 10.8.1** -- Utilise le format dossier-par-table
- 💻 **Multiplateforme** -- Linux, macOS, Windows. SDL3 uniquement, aucune API spécifique à une plateforme
- 📦 **Aucune dépendance système** -- SDL3 et SQLite sont liés statiquement

### 📥 Téléchargement

Téléchargez la dernière version pour votre plateforme -- pas d'installation, il suffit de lancer :

👉 **[Télécharger PinReady](https://github.com/Le-Syl21/PinReady/releases/latest)** (Linux, macOS, Windows)

### 🔨 Compilation depuis les sources

Si vous préférez compiler vous-même :

**🐧 Linux :**

```bash
sudo apt install build-essential cmake pkg-config \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev

cargo build --release
```

**🍎 macOS / 🪟 Windows :**

```bash
cargo build --release
```

SDL3 et SQLite se compilent depuis les sources automatiquement -- aucune installation manuelle nécessaire. ✅

### 🚀 Utilisation

**Premier lancement** (pas de base de données existante) : lance l'assistant de configuration.
**Lancements suivants** : accès direct au lanceur de tables. L'assistant peut être relancé à tout moment.

```bash
# Lancer avec les logs de debug
RUST_LOG=info cargo run

# Ou lancer directement le binaire release
./target/release/pinready
```

**📋 Prérequis :**

- 🎰 **Visual Pinball** exécutable (10.8.1+) -- installé automatiquement ou chemin configuré dans l'assistant
- 📁 **Répertoire de tables** -- format dossier-par-table tel que décrit dans la doc VPX
- 🌐 **Connexion internet** -- nécessaire pour l'installation automatique et la vérification des mises à jour (optionnel pour l'installation manuelle)

**🎮 Contrôles du lanceur :**

| Action | ⌨️ Clavier | 🕹️ Joystick |
|---|---|---|
| Naviguer les tables | Flèches | Flippers gauche/droit (précédent/suivant), Staged flippers (ligne haut/bas) |
| Lancer une table | Entrée | Bouton Start |
| Ouvrir la config | -- | Bouton Launch Ball |
| Quitter le lanceur | Échap | Bouton ExitGame |

---

## 🏗️ Architecture

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
  updater.rs    Visual Pinball release check, download, install
```

## 🧰 Stack

| Layer | Crate | Role |
|---|---|---|
| 🖼️ UI | `eframe` + `egui` | Immediate mode GUI |
| 🖥️ Display/Input | `sdl3-sys` (build-from-source-static) | Screen enumeration, input capture |
| ⚙️ Config | `ini-preserve` | Read/write VPinballX.ini |
| 🗄️ Database | `rusqlite` (bundled) | Local table catalog |
| 🖼️ Images | `image` + `directb2s` | Backglass thumbnail extraction |
| 🔊 Audio | `symphonia` | OGG/Vorbis decode for SDL3 playback |
| 🌐 HTTP | `ureq` | GitHub API + release download |
| 📦 Archive | `zip` + `flate2` + `tar` | Release extraction |
| 🌍 i18n | `rust-i18n` + `noto-fonts-dl` | 20+ languages with font support |

## 🔧 Visual Pinball fork management

The `vpinball-fork.sh` script manages a personal fork of [vpinball/vpinball](https://github.com/vpinball/vpinball) for building Visual Pinball. It keeps CI workflows set to manual dispatch so builds only run when you decide.

Releases created by this script are automatically detected by PinReady clients, which can download and install the correct build for their platform. 🎉

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

## 📄 License

GPL-3.0-or-later
