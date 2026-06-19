# CrossPlatform C App — V2 (raylib statique)

## Architecture du projet

```
crossplatform-c/
├── src/
│   ├── core.h                ← Interface commune (types, contrat)
│   ├── app.c                 ← Logique de l'app (100% portable)
│   ├── main.c                ← Boucle principale (Windows + Linux)
│   ├── platform_windows.c    ← Implémentation Win32 via raylib
│   ├── platform_linux.c      ← Implémentation X11 via raylib
│   └── platform_android.c    ← Implémentation NDK via raylib
├── libs/
│   ├── include/raylib.h
│   ├── linux/libraylib.a
│   ├── windows/libraylib.a
│   └── android_arm64/libraylib.a
└── build/
    ├── build_linux.sh
    ├── build_windows.sh
    └── build_android.sh
```

---

## Points que V2 DOIT respecter

### 1. Un seul exécutable sans DLL externe
- Raylib est linké statiquement (-l:libraylib.a)
- Le binaire final embarque tout le code de raylib
- Les DLLs système (gdi32, X11, opengl32...) sont TOUJOURS présentes
  sur un OS graphique normal, l'utilisateur n'installe rien

### 2. Même code métier sur tous les OS
- app.c ne contient AUCUN #ifdef d'OS
- La logique (physique, rendu, règles) est écrite une seule fois
- Si tu corriges un bug dans app.c, il est corrigé partout

### 3. Interface abstraite claire (core.h)
- Toute communication OS ↔ app passe par les fonctions platform_*
- Remplacer raylib par une autre lib = toucher UNIQUEMENT les platform_*.c
- Ajouter une nouvelle plateforme = créer platform_nouvelle.c

### 4. Delta-time obligatoire
- Toute vitesse est multipliée par delta (secondes/frame)
- L'app tourne à la même vitesse à 30fps et 144fps
- Le delta est plafonné à 0.1s pour éviter les sauts après un freeze

### 5. Séparation update / draw
- app_update() modifie l'état, ne dessine jamais
- app_draw() lit l'état, ne le modifie jamais
- Cette règle permet : replays, interpolation, tests unitaires

### 6. Compilation standard C99 minimum
- Pas de C++ (pas de classes, exceptions, STL)
- Pas d'extensions compilateur (__declspec, __attribute__ évités)
- -Wall activé : zéro warning toléré

### 7. Taille de l'exécutable raisonnable
- Linux : ~1-3 Mo (raylib statique + ton code)
- Windows : ~1-3 Mo (idem)
- Android : ~2-5 Mo (APK compressé)

---

## Obtenir raylib statique

```bash
# Cloner raylib
git clone --depth 1 https://github.com/raysan5/raylib
cd raylib/src

# --- Linux ---
make PLATFORM=PLATFORM_DESKTOP
cp libraylib.a ../../libs/linux/
cp raylib.h ../../libs/include/

# --- Windows (cross depuis Linux) ---
make CC=x86_64-w64-mingw32-gcc \
     AR=x86_64-w64-mingw32-ar  \
     PLATFORM=PLATFORM_DESKTOP  \
     OS=Windows_NT
cp libraylib.a ../../libs/windows/

# --- Android ARM64 ---
make PLATFORM=PLATFORM_ANDROID         \
     ANDROID_NDK=$NDK                   \
     ANDROID_ARCH=arm64                 \
     ANDROID_API_VERSION=33
cp libraylib.a ../../libs/android_arm64/
```

---

## Compiler

```bash
# Linux
chmod +x build/build_linux.sh
./build/build_linux.sh
./build/linux/app

# Windows (depuis Linux avec mingw-w64)
sudo apt install mingw-w64
./build/build_windows.sh
# → build/windows/app.exe (à copier sur Windows)

# Android
export NDK=~/Android/Sdk/ndk/25.2.9519653
export ANDROID_SDK=~/Android/Sdk
./build/build_android.sh
adb install build/android/CrossPlatformApp.apk
```

---

## Sources et documentation

- raylib officiel        : https://www.raylib.com
- raylib GitHub          : https://github.com/raysan5/raylib
- raylib cheatsheet      : https://www.raylib.com/cheatsheet/cheatsheet.html
- Exemples Android raylib: https://github.com/raysan5/raylib/tree/master/projects/AndroidStudio
- NDK Android            : https://developer.android.com/ndk/guides
- MinGW-w64              : https://www.mingw-w64.org
- Android NativeActivity : https://developer.android.com/reference/android/app/NativeActivity
