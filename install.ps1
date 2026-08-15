$ErrorActionPreference = "Stop"

$Repo = "fkm-X3/Minz-CLI"
$InstallDir = "$env:LocalAppData\Programs\Minz-CLI"

# Fetch Latest Release Info from GitHub API
Write-Host "Fetching latest release for $Repo..." -ForegroundColor Cyan
$ApiUrl = "https://api.github.com/repos/$Repo/releases/latest"
$Release = Invoke-RestMethod -Uri $ApiUrl -Headers @{ "User-Agent" = "PowerShell" }

# Find asset matching Windows / x64 / exe
$Asset = $Release.assets | Where-Object { $_.name -like "*win*" -or $_.name -like "*.exe" -or $_.name -like "*.zip" } | Select-Object -First 1

if (-not $Asset) {
    # Fallback to first available download asset
    $Asset = $Release.assets | Select-Object -First 1
}

if (-not $Asset) {
    Write-Error "Could not find suitable release asset for Windows."
    exit 1
}

$DownloadUrl = $Asset.browser_download_url
$FileName = $Asset.name

# Create Install Directory
if (-not (Test-Path -Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

$TempFile = Join-Path $env:TEMP $FileName
Write-Host "Downloading $FileName from $DownloadUrl..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempFile

# Handle ZIP archives vs Direct EXEs
$TargetExePath = Join-Path $InstallDir "minz.exe"

if ($FileName.EndsWith(".zip")) {
    Write-Host "Extracting archive..." -ForegroundColor Cyan
    Expand-Archive -Path $TempFile -DestinationPath $InstallDir -Force
    Remove-Item $TempFile -Force
} else {
    Move-Item -Path $TempFile -Destination $TargetExePath -Force
}

# Add to User PATH if not already present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "Adding $InstallDir to User PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path += ";$InstallDir"
}

Write-Host " Minz-CLI installed successfully!" -ForegroundColor Green
Write-Host "Run 'minz --help' in a new terminal session to get started (THIS COMMAND DOESN'T EXIST YET)." -ForegroundColor Green