param(
    [Parameter(Mandatory = $true)]
    [string]$ExePath,
    [Parameter(Mandatory = $true)]
    [string]$WorkingDirectory,
    [string]$LinkName = "OpenRailWorks.lnk"
)

$ErrorActionPreference = "Stop"

try {

function Resolve-DesktopPath {
    $paths = @(
        [Environment]::GetFolderPath([Environment+SpecialFolder]::Desktop),
        [Environment]::GetFolderPath([Environment+SpecialFolder]::DesktopDirectory)
    )
    if ($env:OneDrive) {
        $paths += Join-Path $env:OneDrive "Desktop"
    }
  if ($env:OneDriveCommercial) {
        $paths += Join-Path $env:OneDriveCommercial "Desktop"
    }
    $paths += @(
        (Join-Path $env:USERPROFILE "OneDrive\Desktop"),
        (Join-Path $env:USERPROFILE "OneDrive - Personal\Desktop"),
        (Join-Path $env:USERPROFILE "Desktop")
    )

    foreach ($p in $paths) {
        if ([string]::IsNullOrWhiteSpace($p)) { continue }
        if (Test-Path -LiteralPath $p) {
            return $p
        }
    }

    $preferred = [Environment]::GetFolderPath([Environment+SpecialFolder]::Desktop)
    if (-not [string]::IsNullOrWhiteSpace($preferred)) {
        New-Item -ItemType Directory -Path $preferred -Force | Out-Null
        return $preferred
    }

    throw "Desktop-Ordner nicht gefunden (OneDrive/Standard)."
}

if (-not (Test-Path -LiteralPath $ExePath)) {
    throw "EXE fehlt: $ExePath"
}

$desktop = Resolve-DesktopPath
$linkPath = Join-Path $desktop $LinkName

$shell = New-Object -ComObject WScript.Shell
$shortcut = $shell.CreateShortcut($linkPath)
$shortcut.TargetPath = $ExePath
$shortcut.WorkingDirectory = $WorkingDirectory
$shortcut.Description = "OpenRailWorks Launcher"
$shortcut.Save()

Write-Output $linkPath

} catch {
    [Console]::Error.WriteLine($_.Exception.Message)
    exit 1
}
