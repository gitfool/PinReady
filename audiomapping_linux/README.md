# Audio mapping — Linux

🇬🇧 [English](#-english) | 🇫🇷 [Français](#-français)

---

## 🇬🇧 English

🚧 **Work in progress** — screenshots coming later.

### External resources to check first

- **[Cleveland Software Design — SSF Installing](https://pinball-docs.clevelandsoftwaredesign.com/docs/ssf/installing)**: full YouTube install video + wiring diagram (excellent starting point)
- [VPForums SSF setup guide](https://www.vpforums.org/index.php?app=tutorials&article=163)
- [VPUniverse SSF setup guide](https://vpuniverse.com/tutorials/article/15-ssf-setup-guide/)
- [Pinscape Build Guide — audio](http://mjrnet.org/pinscape/BuildGuideV2/BuildGuide.php?sid=audio)

### VPX Sound3D modes — jack → speaker mapping

The jack-to-speaker mapping is **identical across all operating systems**. Only the jack retasking tool changes (see the next section).

| VPX mode | Green jack (Front L/R) | Black jack (Rear L/R) | Grey jack (Side L/R) | Orange jack (Center/Sub) | Backglass |
|---|---|---|---|---|---|
| **2ch Front** | Backglass speakers (music) | — | — | — | same speakers |
| **2ch Rear** | Lockbar speakers | — | — | — | separate device recommended |
| **6ch Surround rear lockbar** | Playfield top exciters (BG side) | Lockbar exciters (player side) | — | Subwoofer (optional) | separate device |
| **6ch Surround front lockbar** | Lockbar exciters (player side) | Playfield top exciters (BG side) | — | Subwoofer (optional) | separate device |
| **6ch SSF Legacy** | Top exciters (BG side) | Bottom exciters (lockbar side) | Mid exciters (cabinet sides) | Sub (LFE) | separate device recommended |
| **6ch SSF New** | Top exciters (BG side) | Bottom exciters (lockbar side) | Mid exciters (cabinet sides) | Sub (LFE) | separate device recommended |

**Note**: SSF New uses a better mixing algorithm than Legacy (same physical wiring, improved spatial rendering).

### Jack retasking on Linux with `hdajackretask`

Motherboard-integrated sound cards usually expose only 3 rear jacks (green / pink / blue), while VPX SSF uses 4 stereo pairs (green / black / grey / orange). `hdajackretask` lets you **remap the logical role** of each physical port to unlock the 4 stereo channels of a Realtek HD Audio card.

#### What `hdajackretask` does

Under the hood, `hdajackretask` rewrites the HDA codec's **pin configuration defaults** via an override that the Linux kernel reads when loading the `snd-hda-intel` module. Each physical "jack" on a Realtek card corresponds to a codec pin, and each pin has default configuration (device, location, connection, color) hard-coded in the motherboard firmware.

By overriding these defaults, you force the kernel to expose the pin as a **multi-channel Line-Out** instead of its native role (Mic-In, Line-In, Headphone…), which unlocks the Front / Rear / Side / Center-Sub channels in ALSA / PulseAudio / PipeWire.

The override is stored in `/etc/modprobe.d/hda-jack-retask.conf` and applied at every boot.

#### Installation

**Debian / Ubuntu / Mint**:
```bash
sudo apt install alsa-tools
```

**Arch / Manjaro**:
```bash
sudo pacman -S alsa-tools
```

**Fedora**:
```bash
sudo dnf install alsa-tools-gui
```

Then run:
```bash
hdajackretask
```

(The app **does not require sudo** for the UI — it only asks for admin rights when installing the override.)

#### Identify your codec

Before remapping, identify which Realtek codec your motherboard ships:

```bash
cat /proc/asound/card*/codec* 2>/dev/null | grep -i 'codec\|name' | head
# or
lspci | grep -i audio
# or inside hdajackretask itself, the "Select a codec" dropdown at the top
```

Common VPX-friendly codecs (support retasking 4 stereo pairs):
- **ALC887** / **ALC892** (entry / mid-range motherboards)
- **ALC1200** / **ALC1220** (recent gaming motherboards)
- **ALC897** / **ALC1150** (intermediate)

Older codecs (ALC662, ALC888) may have limitations — check the Realtek datasheet first.

#### Remapping principle

In the `hdajackretask` UI:

1. **Select the codec** in the "Select a codec" dropdown (your motherboard's)
2. The "Pin assignments" list shows every physical pin with its native role (Green Line-Out, Pink Mic-In, Blue Line-In, Front Headphone, etc.)
3. For each pin to remap:
   - Tick **"Override"** next to the pin
   - Pick the new role from the dropdown (e.g. **"Line Out (Front)"**, **"Line Out (Surround)"**, **"Line Out (Center / LFE)"**, **"Line Out (Side)"**)
4. **Typical SSF 5.1 remapping plan** (3 rear jacks):
   - Green jack (native Line Out) → **Line Out (Front)** — backglass speakers
   - Pink jack (native Mic) → **Line Out (Surround)** — playfield rear exciters
   - Blue jack (native Line In) → **Line Out (Side)** — playfield side exciters
5. **For SSF 7.1** (4 stereo outs), you need a 4th channel that often runs through the front headphone jack or an internal port:
   - Green jack → **Line Out (Front)**
   - Pink jack → **Line Out (Side)**
   - Blue jack → **Line Out (Surround)** (= rear)
   - Orange jack / Front Headphone → **Line Out (Center / LFE)**
6. **Pins "mapped to void"**: some codecs expose internal pins (unpopulated HP Out, uncabled SPDIF…). You may need to mark them **"Not connected"** so the codec doesn't try to use them — depends on the motherboard. If ALSA won't expose all outputs after reboot, this is often where to dig.
7. **Install boot override**: bottom-right button. Asks for the root (sudo) password, then writes `/etc/modprobe.d/hda-jack-retask.conf`.

#### Apply without reboot

After `Install boot override`, you can reload the audio module without rebooting:

```bash
# Unload + reload snd-hda-intel
sudo rmmod snd_hda_intel
sudo modprobe snd_hda_intel

# Then restart the PulseAudio/PipeWire layer
systemctl --user restart pipewire pipewire-pulse wireplumber 2>/dev/null || pulseaudio -k
```

Or just reboot (more reliable on many setups).

#### Verify the result

```bash
# List available audio sinks
pactl list short sinks

# Show channels exposed on the Realtek card
pactl list sinks | grep -A 20 'alsa_output.*hda'
```

You should see a sink with **6 channels** (5.1) or **8 channels** (7.1) instead of the default 2 channels (stereo).

In `pavucontrol` or KDE System Settings → Audio, the Realtek card may need a manual **profile** switch to "Analog Surround 5.1" or "Analog Surround 7.1".

#### Rollback / undo

If everything breaks:

```bash
sudo rm /etc/modprobe.d/hda-jack-retask.conf
sudo update-initramfs -u 2>/dev/null || true   # Debian/Ubuntu
sudo reboot
```

#### Cabinet config screenshot — *(coming soon)*

### Verifying in PinReady / VPX

- `pavucontrol` or `pactl list short sinks` to check that the 5.1/7.1 sink is exposed
- In PinReady, Audio page: the new sink should appear in both the **Backglass** and **Playfield** dropdowns
- Pick **6ch SSF New** as the output mode, assign the multi-channel sink to Playfield, and a separate 2ch device (usually another USB card) to Backglass
- The built-in audio test on the Audio page validates the routing

---

## 🇫🇷 Français

🚧 **En cours de fabrication** — captures d'écran à venir.

### Ressources externes à consulter avant

- **[Cleveland Software Design — SSF Installing](https://pinball-docs.clevelandsoftwaredesign.com/docs/ssf/installing)** : vidéo YouTube d'installation complète + diagramme de câblage (excellent point de départ)
- [VPForums SSF setup guide](https://www.vpforums.org/index.php?app=tutorials&article=163)
- [VPUniverse SSF setup guide](https://vpuniverse.com/tutorials/article/15-ssf-setup-guide/)
- [Pinscape Build Guide — audio](http://mjrnet.org/pinscape/BuildGuideV2/BuildGuide.php?sid=audio)

### Modes Sound3D VPX — mapping jack → enceinte

Le mapping jack ↔ enceinte est **identique quel que soit l'OS**. Seul l'outil de retasking des jacks change (voir section suivante).

| Mode VPX | Jack vert (Front L/R) | Jack noir (Rear L/R) | Jack gris (Side L/R) | Jack orange (Center/Sub) | Backglass |
|---|---|---|---|---|---|
| **2ch Front** | Enceintes backglass (musique) | — | — | — | mêmes enceintes |
| **2ch Rear** | Enceintes lockbar | — | — | — | device séparé conseillé |
| **6ch Surround rear lockbar** | Exciters playfield top (côté BG) | Exciters lockbar (côté joueur) | — | Sub (optionnel) | device séparé |
| **6ch Surround front lockbar** | Exciters lockbar (côté joueur) | Exciters playfield top (côté BG) | — | Sub (optionnel) | device séparé |
| **6ch SSF Legacy** | Exciters top (côté BG) | Exciters bottom (côté lockbar) | Exciters mid (côtés cabinet) | Sub (LFE) | device séparé recommandé |
| **6ch SSF New** | Exciters top (côté BG) | Exciters bottom (côté lockbar) | Exciters mid (côtés cabinet) | Sub (LFE) | device séparé recommandé |

**Note** : SSF New utilise un meilleur algorithme de mixing que Legacy (même câblage physique, meilleur rendu spatial).

### Réassignation des jacks sur Linux avec `hdajackretask`

Les cartes son intégrées aux cartes mères n'exposent généralement que 3 jacks à l'arrière (vert / rose / bleu), alors que VPX SSF en utilise 4 stéréo (vert / noir / gris / orange). `hdajackretask` permet de **remapper le rôle logique** de chaque prise physique pour débloquer les 4 canaux stéréo d'une carte Realtek HD Audio.

#### Ce que fait `hdajackretask`

Sous le capot, `hdajackretask` réécrit les **pin configuration defaults** du codec HDA via un override que le kernel Linux lit au chargement du module `snd-hda-intel`. Chaque "jack" physique d'une carte son Realtek correspond à une pin du codec, et chaque pin a une configuration par défaut (device, location, connection, color) définie dans le firmware de la carte mère.

En overridant ces defaults, on force le kernel à exposer la pin comme un **Line-Out multi-canaux** au lieu de son rôle natif (Mic-In, Line-In, Headphone, etc.), ce qui débloque les canaux Front / Rear / Side / Center-Sub dans ALSA / PulseAudio / PipeWire.

L'override est stocké dans `/etc/modprobe.d/hda-jack-retask.conf` et appliqué à chaque boot.

#### Installation

**Debian / Ubuntu / Mint** :
```bash
sudo apt install alsa-tools
```

**Arch / Manjaro** :
```bash
sudo pacman -S alsa-tools
```

**Fedora** :
```bash
sudo dnf install alsa-tools-gui
```

Puis lancer :
```bash
hdajackretask
```

(L'outil **ne demande pas de sudo** pour l'interface — il ne demandera les droits admin qu'au moment d'installer l'override.)

#### Identifier votre codec

Avant de remapper, identifiez quel codec Realtek équipe votre carte mère :

```bash
cat /proc/asound/card*/codec* 2>/dev/null | grep -i 'codec\|name' | head
# ou
lspci | grep -i audio
# ou dans hdajackretask lui-même, la liste déroulante "Select a codec" en haut
```

Codecs VPX-friendly courants (supportent le retasking 4 stéréo) :
- **ALC887** / **ALC892** (cartes mères entrée/milieu de gamme)
- **ALC1200** / **ALC1220** (cartes mères gaming récentes)
- **ALC897** / **ALC1150** (intermédiaires)

Les codecs plus anciens (ALC662, ALC888) peuvent avoir des limitations — vérifier le datasheet Realtek avant.

#### Principe de remapping

Dans l'UI `hdajackretask` :

1. **Sélectionner le codec** dans la liste déroulante "Select a codec" (celui de votre carte mère)
2. La liste "Pin assignments" affiche toutes les pins physiques avec leur rôle natif (Green Line-Out, Pink Mic-In, Blue Line-In, Front Headphone, etc.)
3. Pour chaque pin à remapper :
   - Cocher **"Override"** à côté de la pin
   - Choisir dans le menu déroulant le nouveau rôle (par exemple **"Line Out (Front)"**, **"Line Out (Surround)"**, **"Line Out (Center / LFE)"**, **"Line Out (Side)"**)
4. **Plan de remapping typique pour SSF 5.1** (3 jacks arrière) :
   - Jack vert (Line Out natif) → **Line Out (Front)** — pour les enceintes backglass
   - Jack rose (Mic natif) → **Line Out (Surround)** — pour les exciters rear du playfield
   - Jack bleu (Line In natif) → **Line Out (Side)** — pour les exciters side du playfield
5. **Pour SSF 7.1** (4 sorties stéréo), il faut un 4e canal qui passe souvent par la sortie frontale (Headphone) ou un port interne :
   - Jack vert → **Line Out (Front)**
   - Jack rose → **Line Out (Side)**
   - Jack bleu → **Line Out (Surround)** (= rear)
   - Jack orange / Front Headphone → **Line Out (Center / LFE)**
6. **Pins "dans le vide"** : certains codecs exposent des pins internes (HP Out non connecté sur cette carte mère, SPDIF non câblé, etc.). Il peut être nécessaire de les marquer **"Not connected"** pour que le codec n'essaie pas de les utiliser — ceci dépend de la carte mère. Si ALSA refuse de donner toutes les sorties après le reboot, c'est souvent ici qu'il faut creuser.
7. **Install boot override** : bouton en bas à droite. Demande le mot de passe root (sudo) puis écrit `/etc/modprobe.d/hda-jack-retask.conf`.

#### Appliquer sans rebooter

Après un `Install boot override`, on peut recharger le module audio sans redémarrer :

```bash
# Décharger + recharger snd-hda-intel
sudo rmmod snd_hda_intel
sudo modprobe snd_hda_intel

# Puis relancer la couche PulseAudio/PipeWire
systemctl --user restart pipewire pipewire-pulse wireplumber 2>/dev/null || pulseaudio -k
```

Ou simplement rebooter (plus fiable sur beaucoup de setups).

#### Vérifier le résultat

```bash
# Liste les sinks audio disponibles
pactl list short sinks

# Affiche les canaux disponibles sur la carte Realtek
pactl list sinks | grep -A 20 'alsa_output.*hda'
```

Vous devez voir apparaître une sink avec **6 channels** (5.1) ou **8 channels** (7.1) au lieu de 2 channels (stéréo de base).

Dans PulseAudio UI (`pavucontrol`) ou KDE System Settings → Audio, la carte Realtek peut nécessiter un switch manuel de **profile** vers "Analog Surround 5.1" ou "Analog Surround 7.1".

#### Annuler / rollback

Si ça casse tout :

```bash
sudo rm /etc/modprobe.d/hda-jack-retask.conf
sudo update-initramfs -u 2>/dev/null || true   # Debian/Ubuntu
sudo reboot
```

#### Capture d'écran de la config cabinet — *(à venir)*

### Vérification côté PinReady / VPX

- `pavucontrol` ou `pactl list short sinks` pour vérifier que la sink 5.1/7.1 est bien exposée
- Dans PinReady, page Audio : la nouvelle sink doit apparaître dans les combos **Backglass** et **Playfield**
- Sélectionner **6ch SSF New** dans le mode de sortie, assigner une device playfield (la sink multi-canaux), une device backglass (autre sortie 2ch — autre carte USB typiquement)
- Le test audio intégré de la page Audio valide le routage
