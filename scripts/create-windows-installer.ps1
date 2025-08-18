# scripts/create-windows-installer.ps1
# Creates NSIS installer for Windows

param(
    [string]$Version = "1.0.0",
    [string]$BinaryPath = "target\release\ballistics-analyzer.exe"
)

$ErrorActionPreference = "Stop"

Write-Host "Creating Windows installer..." -ForegroundColor Yellow

# Check if NSIS is installed
if (!(Get-Command makensis -ErrorAction SilentlyContinue)) {
    Write-Error "NSIS not found. Install with: choco install nsis"
    exit 1
}

# Check if binary exists
if (!(Test-Path $BinaryPath)) {
    Write-Error "Binary not found at $BinaryPath"
    exit 1
}

# Create installer script
$nsisScript = @"
; NSIS Installer Script for Ballistics Analyzer
; Privacy-focused - No telemetry or data collection

!define APP_NAME "Ballistics Analyzer"
!define APP_VERSION "$Version"
!define APP_PUBLISHER "Ballistics Analyzer Contributors"
!define APP_URL "https://github.com/DatilDev/Ballistics-Analyzer"
!define APP_EXE "ballistics-analyzer.exe"
!define UNINSTALL_KEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\{APP_NAME}"

; Modern UI
!include "MUI2.nsh"
!include "FileFunc.nsh"

; General Settings
Name "{APP_NAME} {APP_VERSION}"
OutFile "ballistics-analyzer-setup.exe"
InstallDir "`$PROGRAMFILES64\{APP_NAME}"
InstallDirRegKey HKLM "Software\{APP_NAME}" "InstallDir"
RequestExecutionLevel admin
ShowInstDetails show
ShowUninstDetails show

; Version Information
VIProductVersion "{APP_VERSION}.0"
VIAddVersionKey "ProductName" "{APP_NAME}"
VIAddVersionKey "CompanyName" "{APP_PUBLISHER}"
VIAddVersionKey "LegalCopyright" "MIT License"
VIAddVersionKey "FileDescription" "Ballistics Analyzer Installer"
VIAddVersionKey "FileVersion" "{APP_VERSION}"
VIAddVersionKey "ProductVersion" "{APP_VERSION}"
VIAddVersionKey "OriginalFilename" "ballistics-analyzer-setup.exe"

; UI Configuration
!define MUI_ABORTWARNING
!define MUI_ICON "ballistics-desktop\assets\icon.ico"
!define MUI_UNICON "ballistics-desktop\assets\icon.ico"
!define MUI_WELCOMEFINISHPAGE_BITMAP "installer-banner.bmp"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "installer-header.bmp"
!define MUI_HEADERIMAGE_RIGHT

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "LICENSE"
!define MUI_PAGE_CUSTOMFUNCTION_PRE PrivacyPage
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

!define MUI_FINISHPAGE_RUN "`$INSTDIR\{APP_EXE}"
!define MUI_FINISHPAGE_RUN_TEXT "Launch Ballistics Analyzer"
!define MUI_FINISHPAGE_SHOWREADME "`$INSTDIR\PRIVACY_POLICY.md"
!define MUI_FINISHPAGE_SHOWREADME_TEXT "View Privacy Policy"
!insertmacro MUI_PAGE_FINISH

; Uninstaller Pages
!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

; Languages
!insertmacro MUI_LANGUAGE "English"

; Custom Privacy Page
Function PrivacyPage
    MessageBox MB_ICONINFORMATION|MB_OK "Privacy Notice:`$\n`$\n\
    • All data is stored locally on your computer`$\n\
    • No analytics or telemetry`$\n\
    • No internet connection required`$\n\
    • No data collection of any kind`$\n\
    • Complete privacy protection"
FunctionEnd

; Installer Sections
Section "Main Application" SecMain
    SectionIn RO
    
    ; Set output path
    SetOutPath "`$INSTDIR"
    
    ; Copy files
    File "$BinaryPath"
    File /r "ballistics-desktop\assets"
    File "README.md"
    File "LICENSE"
    File "PRIVACY_POLICY.md"
    
    ; Create uninstaller
    WriteUninstaller "`$INSTDIR\uninstall.exe"
    
    ; Registry entries for uninstaller
    WriteRegStr HKLM "{UNINSTALL_KEY}" "DisplayName" "{APP_NAME}"
    WriteRegStr HKLM "{UNINSTALL_KEY}" "UninstallString" "`"`$INSTDIR\uninstall.exe`""
    WriteRegStr HKLM "{UNINSTALL_KEY}" "QuietUninstallString" "`"`$INSTDIR\uninstall.exe`" /S"
    WriteRegStr HKLM "{UNINSTALL_KEY}" "InstallLocation" "`$INSTDIR"
    WriteRegStr HKLM "{UNINSTALL_KEY}" "DisplayIcon" "`$INSTDIR\{APP_EXE},0"
    WriteRegStr HKLM "{UNINSTALL_KEY}" "Publisher" "{APP_PUBLISHER}"
    WriteRegStr HKLM "{UNINSTALL_KEY}" "DisplayVersion" "{APP_VERSION}"
    WriteRegStr HKLM "{UNINSTALL_KEY}" "URLInfoAbout" "{APP_URL}"
    WriteRegStr HKLM "{UNINSTALL_KEY}" "NoModify" 1
    WriteRegDWORD HKLM "{UNINSTALL_KEY}" "NoRepair" 1
    
    ; Get installed size
    {GetSize} "`$INSTDIR" "/S=0K" `$0 `$1 `$2
    IntFmt `$0 "0x%08X" `$0
    WriteRegDWORD HKLM "{UNINSTALL_KEY}" "EstimatedSize" "`$0"
    
    ; Store install directory
    WriteRegStr HKLM "Software\{APP_NAME}" "InstallDir" "`$INSTDIR"
SectionEnd

Section "Start Menu Shortcuts" SecShortcuts
    CreateDirectory "`$SMPROGRAMS\{APP_NAME}"
    CreateShortcut "`$SMPROGRAMS\{APP_NAME}\{APP_NAME}.lnk" "`$INSTDIR\{APP_EXE}" "" "`$INSTDIR\{APP_EXE}" 0
    CreateShortcut "`$SMPROGRAMS\{APP_NAME}\Privacy Policy.lnk" "`$INSTDIR\PRIVACY_POLICY.md"
    CreateShortcut "`$SMPROGRAMS\{APP_NAME}\Uninstall.lnk" "`$INSTDIR\uninstall.exe"
SectionEnd

Section "Desktop Shortcut" SecDesktop
    CreateShortcut "`$DESKTOP\{APP_NAME}.lnk" "`$INSTDIR\{APP_EXE}" "" "`$INSTDIR\{APP_EXE}" 0
SectionEnd

; Descriptions
!insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
    !insertmacro MUI_DESCRIPTION_TEXT {SecMain} "Core application files (required)"
    !insertmacro MUI_DESCRIPTION_TEXT {SecShortcuts} "Add shortcuts to Start Menu"
    !insertmacro MUI_DESCRIPTION_TEXT {SecDesktop} "Add shortcut to Desktop"
!insertmacro MUI_FUNCTION_DESCRIPTION_END

; Uninstaller Section
Section "Uninstall"
    ; Privacy notice
    MessageBox MB_YESNO|MB_ICONQUESTION "This will remove Ballistics Analyzer and all locally stored data. Continue?" IDYES +2
    Abort
    
    ; Remove files
    Delete "`$INSTDIR\{APP_EXE}"
    Delete "`$INSTDIR\uninstall.exe"
    Delete "`$INSTDIR\README.md"
    Delete "`$INSTDIR\LICENSE"
    Delete "`$INSTDIR\PRIVACY_POLICY.md"
    RMDir /r "`$INSTDIR\assets"
    RMDir "`$INSTDIR"
    
    ; Remove shortcuts
    Delete "`$SMPROGRAMS\{APP_NAME}\*.lnk"
    RMDir "`$SMPROGRAMS\{APP_NAME}"
    Delete "`$DESKTOP\{APP_NAME}.lnk"
    
    ; Remove registry entries
    DeleteRegKey HKLM "{UNINSTALL_KEY}"
    DeleteRegKey HKLM "Software\{APP_NAME}"
    
    ; Remove user data (with confirmation)
    MessageBox MB_YESNO|MB_ICONQUESTION "Remove all user data and settings?" IDYES +2
    Goto SkipUserData
    RMDir /r "`$APPDATA\ballistics-analyzer"
    RMDir /r "`$LOCALAPPDATA\ballistics-analyzer"
    
    SkipUserData:
    MessageBox MB_ICONINFORMATION|MB_OK "Uninstallation complete. All data has been removed from your computer."
SectionEnd

; Installation initialization
Function .onInit
    ; Check for previous installation
    ReadRegStr `$0 HKLM "Software\{APP_NAME}" "InstallDir"
    StrCmp `$0 "" +2
    StrCpy `$INSTDIR `$0
FunctionEnd
"@

# Save NSIS script
$nsisScript | Out-File -Encoding UTF8 "installer.nsi"

# Create installer
Write-Host "Compiling installer..." -ForegroundColor Yellow
makensis installer.nsi

if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Installer created: ballistics-analyzer-setup.exe" -ForegroundColor Green
    
    # Sign installer if certificate is available
    if ($env:WINDOWS_CERTIFICATE) {
        Write-Host "Signing installer..." -ForegroundColor Yellow
        signtool sign /f cert.pfx /p $env:WINDOWS_CERTIFICATE_PWD /t http://timestamp.sectigo.com ballistics-analyzer-setup.exe
    }
} else {
    Write-Error "Failed to create installer"
}

# Clean up
Remove-Item installer.nsi