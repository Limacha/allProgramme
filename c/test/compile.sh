#!/bin/bash
set -e  # Stop on first error

echo "=============================="
echo "1) Détection de l'OS"
echo "=============================="

OS="$(uname -s)"
# Détection WSL
if [[ -f /proc/version ]] && grep -qi microsoft /proc/version; then
    OS="WSL"
fi
echo "OS détecté : $OS"

# Détermination de la target
case "$OS" in
    Linux) TARGET_OS=linux ;;
    Darwin) TARGET_OS=macos ;;
    WSL|*) TARGET_OS=windows ;;
esac
echo "Target OS : $TARGET_OS"

echo "=============================="
echo "2) Création des dossiers de build"
echo "=============================="
[ ! -d build ] && mkdir build
[ ! -d build/$TARGET_OS ] && mkdir build/$TARGET_OS

# Dossiers pour objets Debug et Release
OBJ_DEBUG_DIR="build/$TARGET_OS/obj_debug"
OBJ_RELEASE_DIR="build/$TARGET_OS/obj_release"
[ ! -d "$OBJ_DEBUG_DIR" ] && mkdir -p "$OBJ_DEBUG_DIR"
[ ! -d "$OBJ_RELEASE_DIR" ] && mkdir -p "$OBJ_RELEASE_DIR"

# Dossiers pour exécutables
[ ! -d build/$TARGET_OS/debug ] && mkdir -p build/$TARGET_OS/debug
[ ! -d build/$TARGET_OS/release ] && mkdir -p build/$TARGET_OS/release


echo "=============================="
echo "3) Définition des fichiers sources"
echo "=============================="
SOURCES="main.c logic.c render.c input.c files.c memory.c string.c log/log.c dateTime.c fonction.c path.c platform/platform.c"

# Fichier spécifique à la plateforme
case "$TARGET_OS" in
    windows) PLATFORM_SRC="platform/platform_win.c" ;;
    linux) PLATFORM_SRC="platform/platform_linux.c" ;;
    macos) PLATFORM_SRC="" ;;
    android) PLATFORM_SRC="" ;;
    *) PLATFORM_SRC="" ;;
esac

ALL_SOURCES="$SOURCES $PLATFORM_SRC"

echo "=============================="
echo "4) Compilation GCC - DEBUG"
echo "=============================="

if command -v gcc &> /dev/null; then
    echo "gcc détecté - Debug build"
    gcc --version

    for src in $ALL_SOURCES; do
        obj="$OBJ_DEBUG_DIR/$(basename $src .c).o"
        echo "compil $src"
        gcc -c "$src" -o "$obj" -g -Wall -Wextra -Wpedantic
    done

    echo "for fini"

    gcc "$OBJ_DEBUG_DIR"/*.o -o build/$TARGET_OS/debug/app.exe -lgdi32 -luser32 -mwindows -Wall -Wextra -Wpedantic
else
    echo "❌ Aucun compilateur GCC trouvé"
    exit 1
fi

echo "=============================="
echo "5) Compilation GCC - RELEASE"
echo "=============================="

for src in $ALL_SOURCES; do
    obj="$OBJ_RELEASE_DIR/$(basename $src .c).o"
    gcc -c "$src" -o "$obj" -O2 -Wall -Wextra -Wpedantic
done

gcc "$OBJ_RELEASE_DIR"/*.o -o build/$TARGET_OS/release/app.exe -lgdi32 -luser32 -mwindows -Wall -Wextra -Wpedantic


echo "=============================="
echo "✅ Compilation terminée pour $TARGET_OS"
echo "=============================="

# echo "=============================="
# echo "4) Détection du compilateur"
# echo "=============================="

# # # Priorité MSVC sur Windows
# # if [[ "$TARGET_OS" == "windows" ]] && command -v cl.exe &> /dev/null; then
# #     echo "✅ cl.exe détecté"
    
# #     # Charger l'environnement MSVC automatiquement
# #     VCVARS="C:\\Progra~1\\Microsoft Visual Studio\\2022\\Community\\VC\\Auxiliary\\Build\\vcvars64.bat"
    
# #     if [[ -f "/mnt/c/Progra~1/Microsoft Visual Studio/2022/Community/VC/Auxiliary/Build/vcvars64.bat" ]] || [[ -f "$VCVARS" ]]; then
# #         echo "🔧 Chargement de vcvars64.bat..."
# #         # On appelle cl.exe via cmd.exe pour Windows
# #         cmd.exe /c "\"$VCVARS\" && cl.exe $ALL_SOURCES /Fo:build/obj/ /Fe:build/windows/app.exe /EHsc /Zi /nologo /link user32.lib gdi32.lib /SUBSYSTEM:WINDOWS"
# #     else
# #         echo "❌ vcvars64.bat introuvable, configure MSVC manuellement"
# #         exit 1
# #     fi

# # el
# if command -v gcc &> /dev/null; then
#     echo "✅ gcc détecté"
#     if [[ "$TARGET_OS" == "windows" ]]; then
#         gcc $ALL_SOURCES -o build/windows/app.exe -lgdi32 -luser32 -mwindows
#     elif [[ "$TARGET_OS" == "linux" ]]; then
#         gcc $ALL_SOURCES -o build/linux/app -lX11
#     elif [[ "$TARGET_OS" == "android" ]]; then
#         echo "📱 Compilation Android non encore implémentée"
#         exit 1
#     fi
# elif command -v clang &> /dev/null && [[ "$TARGET_OS" == "macos" ]]; then
#     echo "✅ clang détecté sur macOS"
#     clang $ALL_SOURCES -o build/macos/app
# else
#     echo "❌ Aucun compilateur trouvé"
#     exit 1
# fi

# echo "=============================="
# echo "✅ Compilation terminée pour $TARGET_OS"
# echo "=============================="
