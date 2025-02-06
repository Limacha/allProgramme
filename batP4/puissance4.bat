@echo off
title puissance4

:menu
echo ----------Menu----------
echo 1) Notification
echo 2) msg Box
echo 3) msg Box yes no
echo 4) input Box
echo 5) window
echo 6) puissance4
echo 7) try

set /p input=">>"

if %input% EQU 1 goto noti
if %input% EQU 2 goto msgbox
if %input% EQU 3 goto msgboxYN
if %input% EQU 4 goto inputbox
if %input% EQU 5 goto window
if %input% EQU 6 goto puissance4
if %input% EQU 7 goto try

:error
cls
powershell -Command "& {Add-Type -AssemblyName System.Windows.Forms; Add-Type -AssemblyName System.Drawing; $notify = New-Object System.Windows.Forms.NotifyIcon; $notify.Icon = [System.Drawing.SystemIcons]::Error; $notify.Visible = $true; $notify.ShowBalloonTip(0, 'Hello world', 'This is called from a batch script.', [System.Windows.Forms.ToolTipIcon]::None)}"
goto menu

:noti
cls
powershell -Command "& {Add-Type -AssemblyName System.Windows.Forms; Add-Type -AssemblyName System.Drawing; $notify = New-Object System.Windows.Forms.NotifyIcon; $notify.Icon = [System.Drawing.SystemIcons]::Information; $notify.Visible = $true; $notify.ShowBalloonTip(0, 'Hello world', 'This is called from a batch script.', [System.Windows.Forms.ToolTipIcon]::None)}"
goto menu

:msgbox
cls
powershell -Command "& {Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.MessageBox]::Show('Hello World', 'This is an example', 'OK', [System.Windows.Forms.MessageBoxIcon]::Information);}"
goto menu

:msgBoxYN
cls
powershell -Command "& {Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.MessageBox]::Show('Hello', 'Hey', 'YesNo', [System.Windows.Forms.MessageBoxIcon]::Warning);}" > %TEMP%\out.tmp
set /p OUT=<%TEMP%\out.tmp
if %OUT%==Yes (echo Clicked Yes)
goto menu

:inputbox
cls
powershell -Command "& {Add-Type -AssemblyName Microsoft.VisualBasic; [Microsoft.VisualBasic.Interaction]::InputBox('Enter your name:', 'Input box example')}" > %TEMP%\out.tmp
set /p OUT=<%TEMP%\out.tmp
set msgBoxArgs="& {Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.MessageBox]::Show('You have entered: %OUT%', 'Hello');}"
powershell -Command %msgBoxArgs%
goto menu

:window
cls
powershell -Command "& {Add-Type -AssemblyName System.Windows.Forms; $mainForm = New-Object System.Windows.Forms.Form; $mainForm.Text = 'Main Window'; $lbl = New-Object System.Windows.Forms.Label; $lbl.Text = 'Hello World'; $mainForm.Controls.Add($lbl); $mainForm.StartPosition = [System.Windows.Forms.FormStartPosition]::CenterScreen; $mainForm.ShowDialog()}"
goto menu

:puissance4
cls
powershell -Command "& { Add-Type -AssemblyName System.Windows.Forms; $mainForm = New-Object System.Windows.Forms.Form; $mainForm.Text = 'puissance 4'; $lbl = New-Object System.Windows.Forms.Label; $lbl.Text = 'fuckkkkkk?'; $mainForm.Controls.Add($lbl); $mainForm.StartPosition = [System.Windows.Forms.FormStartPosition]::CenterScreen; $mainForm.ShowDialog();}"
goto menu

:try
cls  
powershell -Command ^
 Add-Type -AssemblyName System.Windows.Forms;^
 $mainForm = New-Object System.Windows.Forms.Form;^
 $mainForm.Text = 'puissance 4';^
 $lbl = New-Object System.Windows.Forms.Label;^
 $lbl.Text = 'fuckkkkkk?'; $mainForm.Controls.Add($lbl);^
 $mainForm.StartPosition = [System.Windows.Forms.FormStartPosition]::CenterScreen; $mainForm.ShowDialog();
 
powershell -Command ^
  Add-Type -AssemblyName System.Windows.Forms;^
  Add-Type -AssemblyName System.Drawing;^
  $notify = New-Object System.Windows.Forms.NotifyIcon;^
  $notify.Icon = [System.Drawing.SystemIcons]::Error;^
  $notify.BalloonTipTitle = 'Try';^
  $notify.BalloonTipText = 'This is called from a batch script.';^
  $notify.Visible = $true;^
  $notify.ShowBalloonTip(0)
goto menu

pause