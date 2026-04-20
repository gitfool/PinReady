# Audio mapping — Windows

🇬🇧 [English](#-english) | 🇫🇷 [Français](#-français)

---

## 🇬🇧 English

🚧 **Work in progress** — screenshots coming later.

### External resources to check first

- **[Cleveland Software Design — SSF Installing](https://pinball-docs.clevelandsoftwaredesign.com/docs/ssf/installing)**: full YouTube install video + wiring diagram (excellent starting point)
- **[Cleveland Software Design — SSF Configuring the Computer](https://pinball-docs.clevelandsoftwaredesign.com/docs/ssf/configuring-computer)**: Windows PC configuration (sound card, 5.1/7.1 output, channel assignment)
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

### Jack retasking on Windows with Realtek HD Audio Manager

Motherboard-integrated sound cards on Windows usually expose only 3 rear jacks (green / pink / blue), while VPX SSF uses 4 stereo pairs. The **Realtek HD Audio Manager** (or its OEM equivalent: Dolby Access, Sound Blaster Cinema, ASUS Armoury Crate, MSI Audio, Gigabyte App Center…) lets you **remap the logical role** of each physical port.

#### Accessing the panel

Windows 10/11: `Start → Realtek Audio Console` (or `Control Panel → Sound → Realtek HD Audio Manager` on older versions).

If the app isn't installed, grab it from the [Microsoft Store](https://apps.microsoft.com/detail/9P2B8MCSVPLN) (free, published by Realtek Semiconductor).

#### Principle

1. Tab **Jacks** / **Connector Settings** / similar depending on OEM
2. For each physical jack at the back of the PC:
   - Green jack → assign **Front**
   - Pink jack → assign **Side**
   - Blue jack → assign **Rear**
   - (possibly a 4th orange / front connector → **Center/Sub**)
3. Disable automatic device-type detection (otherwise Windows will reset the pink jack to "Microphone" as soon as something is plugged in)
4. Apply. The config persists at boot.

#### Alternative: Equalizer APO + virtual routing

For advanced cases (Voicemeeter, multi-card routes), [Equalizer APO](https://sourceforge.net/projects/equalizerapo/) lets you create virtual routes at the Windows Audio layer.

#### Cabinet config screenshot — *(coming soon)*

### Verifying in PinReady / VPX

- `Control Panel → Sound → Playback`: the 5.1 or 7.1 device should appear. Right-click → **Configure Speakers** → pick 5.1 or 7.1
- In PinReady, Audio page: the device should appear in both the **Backglass** and **Playfield** dropdowns
- Pick **6ch SSF New**, assign the multi-channel device to Playfield, and another 2ch device to Backglass
- Test with the built-in Audio page test sequence

---

## 🇫🇷 Français

🚧 **En cours de fabrication** — captures d'écran à venir.

### Ressources externes à consulter avant

- **[Cleveland Software Design — SSF Installing](https://pinball-docs.clevelandsoftwaredesign.com/docs/ssf/installing)** : vidéo YouTube d'installation complète + diagramme de câblage (excellent point de départ)
- **[Cleveland Software Design — SSF Configuring the Computer](https://pinball-docs.clevelandsoftwaredesign.com/docs/ssf/configuring-computer)** : configuration du PC Windows (carte son, sortie 5.1/7.1, assignation des canaux)
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

### Réassignation des jacks sur Windows avec Realtek HD Audio Manager

Les cartes son intégrées aux cartes mères Windows n'exposent généralement que 3 jacks (vert / rose / bleu), alors que VPX SSF en utilise 4. Le **Realtek HD Audio Manager** (ou son équivalent OEM : Dolby Access, Sound Blaster Cinema, ASUS Armoury Crate, MSI Audio, Gigabyte App Center…) permet de **remapper le rôle logique** de chaque prise physique.

#### Accès au panneau

Windows 10/11 : `Start → Realtek Audio Console` (ou `Control Panel → Sound → Realtek HD Audio Manager` sur les versions plus anciennes).

Si l'app n'est pas installée, elle est disponible sur le [Microsoft Store](https://apps.microsoft.com/detail/9P2B8MCSVPLN) (gratuite, éditeur Realtek Semiconductor).

#### Principe

1. Onglet **Jacks** / **Connector Settings** / similaire selon l'OEM
2. Pour chaque jack physique à l'arrière du PC :
   - Jack vert → assigner **Front**
   - Jack rose → assigner **Side**
   - Jack bleu → assigner **Rear**
   - (éventuellement un 4e connecteur orange/avant pour **Center/Sub**)
3. Désactiver la détection automatique de type de périphérique (sinon Windows peut remettre le jack rose en "Microphone" dès qu'on y branche quelque chose)
4. Appliquer. La configuration persiste au boot.

#### Alternative : Equalizer APO + route virtuelle

Pour des cas plus avancés (Voicemeeter, routes multi-cartes), [Equalizer APO](https://sourceforge.net/projects/equalizerapo/) permet de créer des routes virtuelles au niveau Windows Audio.

#### Capture d'écran de la config cabinet — *(à venir)*

### Vérification côté PinReady / VPX

- `Control Panel → Sound → Playback` : la device 5.1 ou 7.1 doit apparaître, clic droit → **Configure Speakers** → choisir 5.1 ou 7.1
- Dans PinReady, page Audio : la device doit apparaître dans les combos **Backglass** et **Playfield**
- Sélectionner **6ch SSF New**, assigner la device multi-canaux au Playfield, une autre device 2ch au Backglass
- Tester avec le test intégré de la page Audio
