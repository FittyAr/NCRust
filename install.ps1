# Pairee Windows PowerShell Installer
# Installs Pairee executable and registers assets into %APPDATA%.

$ErrorActionPreference = "Stop"

$repo = "FittyAr/Pairee"
$installDir = Join-Path $HOME "AppData\Local\Programs\pairee"
$configDir = Join-Path $env:APPDATA "pairee\config"
$exePath = Join-Path $installDir "pairee.exe"

Write-Host "Pairee Installer for Windows" -ForegroundColor Blue
Write-Host "=============================="

# Check for Existing Installation
if ((Test-Path $exePath) -or (Test-Path $configDir)) {
    Write-Host "Warning: Pairee is already installed." -ForegroundColor Yellow
    $overwrite = Read-Host "Do you want to overwrite and update the binary? [y/N]"
    if ($overwrite -notmatch "^[yY](es)?$") {
        Write-Host "Installation cancelled."
        exit 0
    }

    if (Test-Path $configDir) {
        $clearConfig = Read-Host "Do you want to clear old configurations, themes, and history settings? [y/N]"
        if ($clearConfig -match "^[yY](es)?$") {
            Write-Host "Clearing old settings in $configDir..."
            Remove-Item -Recurse -Force $configDir -ErrorAction SilentlyContinue
        } else {
            Write-Host "Keeping existing settings."
        }
    }
}

# 1. Fetch Version
Write-Host "Fetching latest version info..."
$releasesUrl = "https://api.github.com/repos/$repo/releases/latest"
try {
    # Using UseBasicParsing to avoid dependency on Internet Explorer engine
    $release = Invoke-RestMethod -Uri $releasesUrl -UseBasicParsing
    $version = $release.tag_name
} catch {
    Write-Error "Failed to retrieve latest release version from GitHub API: $_"
    exit 1
}

Write-Host "Latest version found: $version" -ForegroundColor Green

# 2. Setup folders
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Force -Path $installDir | Out-Null
}
if (-not (Test-Path (Join-Path $configDir "lang"))) {
    New-Item -ItemType Directory -Force -Path (Join-Path $configDir "lang") | Out-Null
}
if (-not (Test-Path (Join-Path $configDir "help"))) {
    New-Item -ItemType Directory -Force -Path (Join-Path $configDir "help") | Out-Null
}

# 3. Download and Extract ZIP
$tempDir = Join-Path $env:TEMP "pairee_install_$(Get-Date -Format 'yyyyMMddHHmmss')"
New-Item -ItemType Directory -Force -Path $tempDir | Out-Null

$zipName = "pairee-$version-x86_64-pc-windows-msvc.zip"
$downloadUrl = "https://github.com/$repo/releases/download/$version/$zipName"
$zipPath = Join-Path $tempDir $zipName

Write-Host "Downloading $zipName..."
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
} catch {
    Write-Error "Failed to download release file: $_"
    Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
    exit 1
}

Write-Host "Extracting archive..."
try {
    Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force
} catch {
    Write-Error "Failed to extract ZIP file: $_"
    Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue
    exit 1
}

# 4. Copying Files
Write-Host "Installing files..."
$extractedFolder = Join-Path $tempDir "pairee-$version-x86_64-pc-windows-msvc"

# Copy executable
Copy-Item -Path (Join-Path $extractedFolder "pairee.exe") -Destination $installDir -Force

# Copy resources
Copy-Item -Path (Join-Path $extractedFolder "lang\*") -Destination (Join-Path $configDir "lang") -Force -Recurse
Copy-Item -Path (Join-Path $extractedFolder "help\*") -Destination (Join-Path $configDir "help") -Force -Recurse

# Clean up temp
Remove-Item -Recurse -Force $tempDir -ErrorAction SilentlyContinue

Write-Host "==============================" -ForegroundColor Blue
Write-Host "Pairee version $version installed successfully!" -ForegroundColor Green
Write-Host "Executable location: $installDir\pairee.exe" -ForegroundColor Blue
Write-Host "Configuration and resources folder: $configDir" -ForegroundColor Blue
Write-Host ""

# 5. PATH setup
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -split ';' -notcontains $installDir) {
    Write-Host "Adding Pairee directory to User PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$installDir", "User")
    # Update current process PATH
    $env:PATH = "$env:PATH;$installDir"
    Write-Host "PATH updated successfully. You might need to restart your terminal/IDE to refresh PATH changes." -ForegroundColor Green
} else {
    Write-Host "Pairee directory is already in User PATH." -ForegroundColor Green
}

Write-Host "Run Pairee by typing: pairee" -ForegroundColor Green
