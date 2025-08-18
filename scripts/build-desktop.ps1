# scripts/build-desktop.ps1 - Windows build script for Ballistics Analyzer

param(
    [string]$BuildType = "release",
    [switch]$Clean,
    [switch]$InstallDeps,
    [switch]$CreateInstaller
)

# Colors for output
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

Write-Host "Building Ballistics Analyzer Desktop for Windows" -ForegroundColor Blue
Write-Host "Architecture: $env:PROCESSOR_ARCHITECTURE" -ForegroundColor Green

# Check for required tools
function Test-Command($Command) {
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    } catch {
        return $false
    }
}

# Install dependencies
if ($InstallDeps) {
    Write-Host "Installing dependencies..." -ForegroundColor Yellow
    
    # Check for Chocolatey
    if (!(Test-Command choco)) {
        Write-Host "Installing Chocolatey..." -ForegroundColor Yellow
        Set-ExecutionPolicy Bypass -Scope Process -Force
        [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
        iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    }
    
    # Install build tools
    choco install -y visualstudio2022-workload-vctools
    choco install -y cmake
    choco install -y nsis  # For creating installer
    
    # Install Rust if not present
    if (!(Test-Command rustc)) {
        Write-Host "Installing Rust..." -ForegroundColor Yellow
        Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
        .\rustup-init.exe -y
        Remove-Item rustup-init.exe
        $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
    }
}

# Check for Rust
if (!(Test-Command rustc)) {
    Write-Error "Rust not found. Please install from https://rustup.rs or run with -InstallDeps"
    exit 1
}

# Update Rust
Write-Host "Updating Rust toolchain..." -ForegroundColor Yellow
rustup update stable

# Determine target
$target = ""
switch ($env:PROCESSOR_ARCHITECTURE) {
    "AMD64" { $target = "x86_64-pc-windows-msvc" }
    "ARM64" { $target = "aarch64-pc-windows-msvc" }
    default { $target = "x86_64-pc-windows-msvc" }
}

Write-Host "Target: $target" -ForegroundColor Yellow
rustup target add $target

# Set build flags
$buildFlags = @()
if ($BuildType -eq "release") {
    $buildFlags += "--release"
    $outputDir = "release"
} else {
    $outputDir = "debug"
}
$buildFlags += "--target"
$buildFlags += $target

# Clean if requested
if ($Clean) {
    Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
    cargo clean
}

# Set environment variables for Windows build
$env:RUSTFLAGS = "-C target-feature=+crt-static"

# Build ballistics_core
Write-Host "Building ballistics_core..." -ForegroundColor Blue
Push-Location ballistics_core
try {
    cargo build @buildFlags
    if ($LASTEXITCODE -ne 0) { throw "Build failed" }
} finally {
    Pop-Location
}

# Build ballistics-desktop
Write-Host "Building ballistics-desktop..." -ForegroundColor Blue
Push-Location ballistics-desktop
try {
    cargo build @buildFlags
    if ($LASTEXITCODE -ne 0) { throw "Build failed" }
} finally {
    Pop-Location
}

# Find output binary
$binaryPath = "target\$target\$outputDir\ballistics-analyzer.exe"

if (!(Test-Path $binaryPath)) {
    Write-Error "Binary not found at $binaryPath"
    exit 1
}

# Create output directory
New-Item -ItemType Directory -Force -Path "build\windows" | Out-Null

# Copy binary and assets
Copy-Item $binaryPath "build\windows\"
Copy-Item -Recurse "ballistics-desktop\assets" "build\windows\"

# Copy Visual C++ redistributables if in release mode
if ($BuildType -eq "release") {
    Write-Host "Copying Visual C++ redistributables..." -ForegroundColor Yellow
    
    $vcRedistPath = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools\VC\Redist\MSVC"
    if (Test-Path $vcRedistPath) {
        $latestRedist = Get-ChildItem $vcRedistPath | Sort-Object Name -Descending | Select-Object -First 1
        if ($target -match "x86_64") {
            Copy-Item "$($latestRedist.FullName)\x64\Microsoft.VC*.CRT\*.dll" "build\windows\" -ErrorAction SilentlyContinue
        }
    }
}

# Create installer if requested
if ($CreateInstaller -and $BuildType -eq "release") {
    Write-Host "Creating Windows installer..." -ForegroundColor Yellow
    
    if (Test-Command makensis) {
        # Create NSIS script
        $nsisScript = @"
!define APP_NAME "Ballistics Analyzer"
!define APP_VERSION "1.0.0"
!define APP_PUBLISHER "Ballistics Analyzer Contributors"
!define APP_URL "https://github.com/DatilDev/Ballistics-Analyzer"
!define APP_EXE "ballistics-analyzer.exe"

Name "`${APP_NAME}"
OutFile "build\windows\ballistics-analyzer-setup.exe"
InstallDir "`$PROGRAMFILES64\`${APP_NAME}"
RequestExecutionLevel admin

!include "MUI2.nsh"

!define MUI_ICON "ballistics-desktop\assets\icon.ico"
!define MUI_UNICON "ballistics-desktop\assets\icon.ico"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath `$INSTDIR
    File "build\windows\ballistics-analyzer.exe"
    File /r "build\windows\assets"
    File "README.md"
    File "LICENSE"
    File "PRIVACY_POLICY.md"
    
    WriteUninstaller "`$INSTDIR\uninstall.exe"
    
    # Create Start Menu shortcuts
    CreateDirectory "`$SMPROGRAMS\`${APP_NAME}"
    CreateShortcut "`$SMPROGRAMS\`${APP_NAME}\`${APP_NAME}.lnk" "`$INSTDIR\`${APP_EXE}"
    CreateShortcut "`$SMPROGRAMS\`${APP_NAME}\Uninstall.lnk" "`$INSTDIR\uninstall.exe"
    
    # Create Desktop shortcut
    CreateShortcut "`$DESKTOP\`${APP_NAME}.lnk" "`$INSTDIR\`${APP_EXE}"
    
    # Registry entries
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APP_NAME}" "DisplayName" "`${APP_NAME}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APP_NAME}" "UninstallString" "`$INSTDIR\uninstall.exe"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APP_NAME}" "Publisher" "`${APP_PUBLISHER}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APP_NAME}" "DisplayVersion" "`${APP_VERSION}"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APP_NAME}" "URLInfoAbout" "`${APP_URL}"
SectionEnd

Section "Uninstall"
    Delete "`$INSTDIR\*.*"
    RMDir /r "`$INSTDIR"
    Delete "`$SMPROGRAMS\`${APP_NAME}\*.*"
    RMDir "`$SMPROGRAMS\`${APP_NAME}"
    Delete "`$DESKTOP\`${APP_NAME}.lnk"
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\`${APP_NAME}"
SectionEnd
"@
        $nsisScript | Out-File -Encoding UTF8 "build\windows\installer.nsi"
        
        makensis "build\windows\installer.nsi"
        Write-Host "Installer created: build\windows\ballistics-analyzer-setup.exe" -ForegroundColor Green
    } else {
        Write-Warning "NSIS not found. Skipping installer creation."
    }
}

# Create ZIP package
if ($BuildType -eq "release") {
    Write-Host "Creating distribution package..." -ForegroundColor Yellow
    
    $packageName = "ballistics-analyzer-windows-$env:PROCESSOR_ARCHITECTURE"
    $packageDir = "build\$packageName"
    
    New-Item -ItemType Directory -Force -Path $packageDir | Out-Null
    
    Copy-Item "build\windows\ballistics-analyzer.exe" $packageDir
    Copy-Item -Recurse "build\windows\assets" $packageDir
    Copy-Item "README.md" $packageDir
    Copy-Item "LICENSE" $packageDir
    Copy-Item "PRIVACY_POLICY.md" $packageDir
    
    Compress-Archive -Path $packageDir -DestinationPath "build\$packageName.zip" -Force
    
    Write-Host "Distribution package: build\$packageName.zip" -ForegroundColor Green
}

# Display binary info
Write-Host "`nBuild Information:" -ForegroundColor Blue
$binary = Get-Item $binaryPath
Write-Host "Binary: $($binary.FullName)"
Write-Host "Size: $([math]::Round($binary.Length / 1MB, 2)) MB"
Write-Host "Version: $((Get-Command $binary.FullName).FileVersionInfo.FileVersion)"

Write-Host "`n✓ Build complete!" -ForegroundColor Green
Write-Host "Binary location: build\windows\ballistics-analyzer.exe" -ForegroundColor Green