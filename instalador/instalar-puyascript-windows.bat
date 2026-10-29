@echo off
title Instalador de PuyaScript
color 0A

echo ========================================
echo      Instalador de PuyaScript          
echo ========================================
echo.

set "RUTA_DESTINO=%LOCALAPPDATA%\Programs\PuyaScript"
set "BINARIO=puyascript.exe"
set "DIR_SCRIPT=%~dp0"

REM Comprobar si el ejecutable está en la misma carpeta que el script
if not exist "%DIR_SCRIPT%%BINARIO%" (
    color 0C
    echo [ERROR] No se encuentra %BINARIO%.
    echo Asegurate de que el instalador y el ejecutable esten juntos.
    echo.
    pause
    exit /b 1
)

echo [+] Creando carpeta de instalacion...
if not exist "%RUTA_DESTINO%" mkdir "%RUTA_DESTINO%"

echo [+] Copiando archivos...
copy /Y "%DIR_SCRIPT%%BINARIO%" "%RUTA_DESTINO%\%BINARIO%" >nul

echo [+] Configurando el sistema (PATH)...
powershell -NoProfile -ExecutionPolicy Bypass -Command ^
    "$path = [Environment]::GetEnvironmentVariable('Path', 'User'); " ^
    "if (-not ($path -split ';' -contains '%RUTA_DESTINO%')) { " ^
    "   [Environment]::SetEnvironmentVariable('Path', $path + ';%RUTA_DESTINO%', 'User'); " ^
    "} "

echo.
echo ========================================
echo  ¡Instalacion completada con exito!
echo ========================================
echo Ya puedes usar 'puyascript' desde cualquier terminal.
echo (Cierra y abre tu terminal si es la primera vez).
echo.
pause