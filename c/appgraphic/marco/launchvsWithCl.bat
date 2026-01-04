@echo off
REM === Charger l'environnement de compilation de Visual Studio ===
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"

REM === Lancer Visual Studio Code dans le dossier courant ===
code .
