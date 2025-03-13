@echo off
title multi tool
chcp 65001 >nul
:start
cls
cd
cd C:\Users\Nico\desktop
cd
call :banner


:menu
echo.
echo [38;2;255;204;0m╔═(0) reset
echo [38;2;255;204;0m║
echo [38;2;255;204;0m╠═(1) archive with copy
echo [38;2;255;204;0m║
echo [38;2;255;204;0m╠═(2) unarchive with 7z
echo [38;2;255;204;0m║
echo [38;2;255;204;0m╠═(3) hide file
echo [38;2;255;204;0m║
echo [38;2;255;204;0m╠═(4) hide write file
echo [38;2;255;204;0m║
echo [38;2;255;204;0m╠═(5) show file
echo [38;2;255;204;0m║
echo [38;2;255;204;0m╠═(6) wlan profil
echo [38;2;255;204;0m║
echo [38;2;255;204;0m╠═(7) wlan profil key clear
echo [38;2;255;204;0m║
echo [38;2;255;204;0m╠═(8) weather
echo [38;2;255;204;0m║
echo [38;2;255;204;0m╠═(9) show all file
echo [38;2;255;204;0m║
set /p input=[38;2;255;204;0m╚══════^>
if /I %input% EQU 0 cls & goto menu
if /I %input% EQU 1 goto copyArch
if /I %input% EQU 2 goto unarch7zip
if /I %input% EQU 3 goto hideFile
if /I %input% EQU 4 goto hideWriteFile
if /I %input% EQU 5 goto showFile
if /I %input% EQU 6 goto wlanProfile
if /I %input% EQU 7 goto wlanKeyClear
if /I %input% EQU 8 goto weather
if /I %input% EQU 9 goto showAllFile

goto start


:copyArch
cls
echo couverture
set /p couv=">>"

echo archive
set /p arch=">>"

echo nouveau fichier
set /p nouv=">>"
cls
if not exist %couv% echo %couv% not found & goto menu
if not exist %arch% echo %arch% not found & goto menu
if %nouv% EQU "" echo no nouv & goto menu
rem seulement la couverture + un fichier zip pour voir le contenu
rem si un fichie deja trafiquer dedans alors reste comme il est mais nouvre pas lecteur img si image mais le contenu
rem c:\directory\file ou directory\file
copy /b %couv%+%arch% %nouv%
echo copy complete
echo with %couv% and %arch%
echo to %nouv%
goto menu

:unarch7zip
cls
REM Path to the archive and destination folder
echo archive
set /p archive=">>"
echo sortie
set /p output=">>"
cls
if not exist "C:\Program Files\7-Zip\7z.exe" echo C:\Program Files\7-Zip\7z.exe not found & goto menu
if not exist %archive% echo %archive% not found & goto menu
if %output% EQU "" echo no output & goto menu
"C:\Program Files\7-Zip\7z.exe" x %archive% -o%output% -y
echo Extraction complete.
echo from %archive%
echo to %output%
goto menu


:hideFile
cls
echo file
set /p file=">>"
cls
if not exist %file% echo %file% not found & goto menu
attrib +h +s +r %file%
echo %file% hide
goto menu

:hideWriteFile
cls
echo file
set /p file=">>"
cls
if not exist %file% echo %file% not found & goto menu
attrib +h +s -r %file%
echo %file% hide
goto menu

:showFile
cls
echo file
set /p file=">>"
cls
if not exist %file% echo %file% not found & goto menu
attrib -h -s -r %file%
echo %file% show
goto menu

:wlanProfile
cls
netsh wlan show profile
goto menu

:wlanKeyClear
cls
echo "name"
set /p name=">>"
if %name% EQU "" echo no input & goto menu
netsh wlan show profile %name% key=clear
goto menu

:weather
cls
echo "contry"
set /p contry=">>"
if %contry% EQU "" echo no input & goto menu
set url=wttr.in/%contry%
echo %url%
curl %url%
goto menu

:showAllFile
cls
attrib /D
goto menu

:banner
echo.
echo.
echo                     [38;2;255;0;0m███╗   ███╗██╗   ██╗██╗  ████████╗██╗    ████████╗ ██████╗  ██████╗ ██╗[0m     
echo                     [38;2;255;51;0m████╗ ████║██║   ██║██║  ╚══██╔══╝██║    ╚══██╔══╝██╔═══██╗██╔═══██╗██║     [0m
echo                     [38;2;255;102;0m██╔████╔██║██║   ██║██║     ██║   ██║       ██║   ██║   ██║██║   ██║██║    [0m 
echo                     [38;2;255;153;0m██║╚██╔╝██║██║   ██║██║     ██║   ██║       ██║   ██║   ██║██║   ██║██║     [0m
echo                     [38;2;255;204;0m██║ ╚═╝ ██║╚██████╔╝███████╗██║   ██║       ██║   ╚██████╔╝╚██████╔╝███████╗[0m
echo                     [38;2;255;255;0m╚═╝     ╚═╝ ╚═════╝ ╚══════╝╚═╝   ╚═╝       ╚═╝    ╚═════╝  ╚═════╝ ╚══════╝[0m
echo.
goto menu

pause