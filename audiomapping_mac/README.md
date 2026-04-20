# Audio mapping — macOS

🇬🇧 [English](#-english) | 🇫🇷 [Français](#-français)

---

## 🇬🇧 English

🚧 **Work in progress** — screenshots coming later.

### External resources to check first

- **[Cleveland Software Design — SSF Installing](https://pinball-docs.clevelandsoftwaredesign.com/docs/ssf/installing)**: full YouTube install video + wiring diagram (excellent starting point)
- [VPForums SSF setup guide](https://www.vpforums.org/index.php?app=tutorials&article=163)
- [VPUniverse SSF setup guide](https://vpuniverse.com/tutorials/article/15-ssf-setup-guide/)
- [Pinscape Build Guide — audio](http://mjrnet.org/pinscape/BuildGuideV2/BuildGuide.php?sid=audio)

### VPX Sound3D modes — channel → speaker mapping

The channel-to-speaker mapping is **identical across all operating systems**. The macOS specificity is that you **don't remap** the built-in audio jacks (not supported) — instead you use an external USB multi-channel interface + Audio MIDI Setup.

| VPX mode | Front L/R channels | Rear L/R channels | Side L/R channels | Center/Sub channels | Backglass |
|---|---|---|---|---|---|
| **2ch Front** | Backglass speakers (music) | — | — | — | same speakers |
| **2ch Rear** | Lockbar speakers | — | — | — | separate device recommended |
| **6ch Surround rear lockbar** | Playfield top exciters (BG side) | Lockbar exciters (player side) | — | Subwoofer (optional) | separate device |
| **6ch Surround front lockbar** | Lockbar exciters (player side) | Playfield top exciters (BG side) | — | Subwoofer (optional) | separate device |
| **6ch SSF Legacy** | Top exciters (BG side) | Bottom exciters (lockbar side) | Mid exciters (cabinet sides) | Sub (LFE) | separate device recommended |
| **6ch SSF New** | Top exciters (BG side) | Bottom exciters (lockbar side) | Mid exciters (cabinet sides) | Sub (LFE) | separate device recommended |

**Note**: SSF New uses a better mixing algorithm than Legacy (same physical wiring, improved spatial rendering).

### Multi-channel interface on macOS

Built-in audio on Mac (Apple Silicon or Intel) doesn't support jack retasking like Linux / Windows. Two main options:

#### Option 1 — USB multi-channel interface

A single USB box with 5.1 or 7.1 separate outputs. Examples:

- **Behringer UMC404HD / UMC1820** — pro 4/8 line-level outputs, excellent quality
- **Focusrite Scarlett 4i4 / 8i6 / 18i20** — semi-pro reference
- **Low-cost USB 5.1/7.1 adapter** — works but variable quality (look for CM6206-based ones)

#### Option 2 — Aggregate Device (multiple USB stereo cards)

If you already own several small USB 2ch cards (e.g. Behringer UCA222), you can combine them into a virtual multi-channel device via **Audio MIDI Setup**:

1. Open `Applications → Utilities → Audio MIDI Setup` (or `⌘+Space` → "Audio MIDI Setup")
2. Bottom-left `+` menu → **Create Aggregate Device**
3. Tick the USB cards to combine
4. Enable **Drift Correction** on all but the first (the master clock)
5. The Aggregate Device exposes N×2 channels → appears as a single multi-channel sink in PinReady / VPX

#### Cabinet config screenshot — *(coming soon)*

### Verifying in PinReady / VPX

- Audio MIDI Setup → **Configure Speakers** tab on the device: assign each channel to its logical position (Front L/R, Surround L/R, etc.)
- In PinReady, Audio page: the device should appear in both the **Backglass** and **Playfield** dropdowns
- Pick **6ch SSF New**, assign the multi-channel device to Playfield, and another 2ch device to Backglass
- Test with the built-in Audio page test sequence

---

## 🇫🇷 Français

🚧 **En cours de fabrication** — captures d'écran à venir.

### Ressources externes à consulter avant

- **[Cleveland Software Design — SSF Installing](https://pinball-docs.clevelandsoftwaredesign.com/docs/ssf/installing)** : vidéo YouTube d'installation complète + diagramme de câblage (excellent point de départ)
- [VPForums SSF setup guide](https://www.vpforums.org/index.php?app=tutorials&article=163)
- [VPUniverse SSF setup guide](https://vpuniverse.com/tutorials/article/15-ssf-setup-guide/)
- [Pinscape Build Guide — audio](http://mjrnet.org/pinscape/BuildGuideV2/BuildGuide.php?sid=audio)

### Modes Sound3D VPX — mapping jack → enceinte

Le mapping jack ↔ enceinte est **identique quel que soit l'OS**. Sur macOS la particularité est qu'on **ne remappe pas** les jacks de l'audio intégré (non supporté) : on utilise à la place une interface USB multi-canaux externe + Audio MIDI Setup.

| Mode VPX | Canaux Front L/R | Canaux Rear L/R | Canaux Side L/R | Canaux Center/Sub | Backglass |
|---|---|---|---|---|---|
| **2ch Front** | Enceintes backglass (musique) | — | — | — | mêmes enceintes |
| **2ch Rear** | Enceintes lockbar | — | — | — | device séparé conseillé |
| **6ch Surround rear lockbar** | Exciters playfield top (côté BG) | Exciters lockbar (côté joueur) | — | Sub (optionnel) | device séparé |
| **6ch Surround front lockbar** | Exciters lockbar (côté joueur) | Exciters playfield top (côté BG) | — | Sub (optionnel) | device séparé |
| **6ch SSF Legacy** | Exciters top (côté BG) | Exciters bottom (côté lockbar) | Exciters mid (côtés cabinet) | Sub (LFE) | device séparé recommandé |
| **6ch SSF New** | Exciters top (côté BG) | Exciters bottom (côté lockbar) | Exciters mid (côtés cabinet) | Sub (LFE) | device séparé recommandé |

**Note** : SSF New utilise un meilleur algorithme de mixing que Legacy (même câblage physique, meilleur rendu spatial).

### Interface multi-canaux sur macOS

L'audio intégré des Mac (Apple Silicon ou Intel) ne supporte pas le retasking des jacks comme Linux / Windows. Deux options principales :

#### Option 1 — Interface USB multi-canaux

Une seule box USB avec 5.1 ou 7.1 de sorties séparées. Exemples :

- **Behringer UMC404HD / UMC1820** — interface pro 4/8 sorties line-level, excellente qualité
- **Focusrite Scarlett 4i4 / 8i6 / 18i20** — référence semi-pro
- **Adaptateur USB 5.1/7.1** bas de gamme — fonctionne mais qualité variable (chercher ceux basés sur le chip CM6206)

#### Option 2 — Aggregate Device (plusieurs cartes USB stéréo)

Si vous avez déjà plusieurs petites cartes USB 2ch (type Behringer UCA222), vous pouvez les combiner en un device virtuel multi-canaux via **Audio MIDI Setup** :

1. Ouvrir `Applications → Utilities → Audio MIDI Setup` (ou `⌘+Space` → "Audio MIDI Setup")
2. Menu `+` en bas à gauche → **Create Aggregate Device**
3. Cocher les cartes USB à combiner
4. Activer **Drift Correction** sur toutes sauf la première (l'horloge master)
5. L'Aggregate Device expose N×2 canaux → apparaît comme une seule sink multi-canaux dans PinReady / VPX

#### Capture d'écran de la config cabinet — *(à venir)*

### Vérification côté PinReady / VPX

- Audio MIDI Setup → onglet **Configure Speakers** sur la device : affecter chaque canal à sa position logique (Front L/R, Surround L/R, etc.)
- Dans PinReady, page Audio : la device doit apparaître dans les combos **Backglass** et **Playfield**
- Sélectionner **6ch SSF New**, assigner la device multi-canaux au Playfield, une autre device 2ch au Backglass
- Tester avec le test intégré de la page Audio
