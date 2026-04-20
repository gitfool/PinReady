# Audio jack mapping — Linux

🚧 **En cours de fabrication** — Work in progress.

Ce guide expliquera comment réassigner les 3 jacks d'une carte son intégrée (typiquement Realtek ALC sur carte mère) pour piloter un cabinet en 5.1 ou 7.1, sans racheter de carte son, via **`hdajackretask`**.

À venir :
- Installation d'`alsa-tools` (paquet Debian/Ubuntu/Arch)
- Capture d'écran de `hdajackretask` avec la config cabinet recommandée
- Exemple de mapping : Line-Out → Front, Mic-In → Side, Line-In → Rear
- Précisions sur les ports à mapper "dans le vide" pour débloquer certaines sorties
- Vérification côté SDL3 / PinReady / VPX
