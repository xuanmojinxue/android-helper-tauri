@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

:menu
cls
echo ========================================
echo   Android Helper Build Tool
echo ========================================
echo.
echo   [1] Run Dev
echo   [2] Build Installer (NSIS/MSI)
echo   [3] Build Portable
echo   [0] Exit
echo.
echo ========================================
set /p choice=Select: 

if "%choice%"=="1" goto run_dev
if "%choice%"=="2" goto build_installer
if "%choice%"=="3" goto build_portable
if "%choice%"=="0" exit /b 0
goto menu

:run_dev
cls
echo [Run Dev]
echo.
call :check_env
if %errorlevel% neq 0 goto end

echo Starting dev server...
echo Press Ctrl+C to stop
echo.
call npm run tauri dev
goto end

:build_installer
cls
echo [Build Installer]
echo.
call :check_env
if %errorlevel% neq 0 goto end

call :install_deps
if %errorlevel% neq 0 goto end

echo [2/3] Building...
echo.
call npm run tauri build
if %errorlevel% neq 0 (
    echo.
    echo [Error] Build failed
    goto end
)

echo.
echo [3/3] Build complete!
echo.
echo ========================================
echo   Output:
echo ========================================
if exist "src-tauri\target\release\bundle\nsis\*.exe" (
    echo   NSIS: src-tauri\target\release\bundle\nsis\
    explorer "src-tauri\target\release\bundle\nsis"
)
if exist "src-tauri\target\release\bundle\msi\*.msi" (
    echo   MSI:  src-tauri\target\release\bundle\msi\
)
echo.
goto end

:build_portable
cls
echo [Build Portable]
echo.
call :check_env
if %errorlevel% neq 0 goto end

call :install_deps
if %errorlevel% neq 0 goto end

echo [2/4] Compiling...
echo.
call npm run tauri build -- --no-bundle
if %errorlevel% neq 0 (
    echo.
    echo [Error] Compile failed
    goto end
)

echo.
echo [3/4] Creating portable folder...
set "OUTPUT_DIR=dist\AndroidHelper-Portable"
if exist "%OUTPUT_DIR%" rmdir /s /q "%OUTPUT_DIR%"
mkdir "%OUTPUT_DIR%"

copy "src-tauri\target\release\Android玩机助手.exe" "%OUTPUT_DIR%\" >nul
echo   - Copy exe

if exist "src-tauri\tools" (
    xcopy "src-tauri\tools" "%OUTPUT_DIR%\tools\" /e /i /q >nul
    echo   - Copy tools
)

mkdir "%OUTPUT_DIR%\data"
mkdir "%OUTPUT_DIR%\data\APK"
mkdir "%OUTPUT_DIR%\data\Backup"
mkdir "%OUTPUT_DIR%\data\Screenshot"
mkdir "%OUTPUT_DIR%\data\Record"
mkdir "%OUTPUT_DIR%\data\ROM"
mkdir "%OUTPUT_DIR%\data\Module"
mkdir "%OUTPUT_DIR%\data\Log"
echo   - Create data folder

if exist "src-tauri\target\release\WebView2Loader.dll" (
    copy "src-tauri\target\release\WebView2Loader.dll" "%OUTPUT_DIR%\" >nul
    echo   - Copy WebView2Loader.dll
)

echo.
echo [4/4] Portable build complete!
echo.
echo ========================================
echo   Output: %CD%\%OUTPUT_DIR%\
echo ========================================
echo.
explorer "%OUTPUT_DIR%"
goto end

:check_env
if not exist "package.json" (
    echo [Error] Run this script in project root
    exit /b 1
)

where node >nul 2>&1
if %errorlevel% neq 0 (
    echo [Error] Node.js not found
    exit /b 1
)

where rustc >nul 2>&1
if %errorlevel% neq 0 (
    echo [Error] Rust not found
    exit /b 1
)

if exist "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" (
    call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" -arch=x64 >nul 2>&1
)

exit /b 0

:install_deps
echo [1/4] Checking dependencies...
if not exist "node_modules" (
    echo Installing npm dependencies...
    call npm install
    if %errorlevel% neq 0 (
        echo [Error] npm install failed
        exit /b 1
    )
)
exit /b 0

:end
echo.
echo Press any key to return...
pause >nul
goto menu
