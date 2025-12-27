# ZeroTier MCP Server 安装脚本 (Windows)
# 用法: irm https://raw.githubusercontent.com/fromsko/zerotier-sdk-rust-mcp/main/scripts/install.ps1 | iex

$ErrorActionPreference = "Stop"

$Repo = "fromsko/zerotier-sdk-rust-mcp"
$BinaryName = "zerotier-mcp"
$InstallDir = "$env:LOCALAPPDATA\zerotier-mcp"

function Write-Info { param($msg) Write-Host "[INFO] $msg" -ForegroundColor Green }
function Write-Warn { param($msg) Write-Host "[WARN] $msg" -ForegroundColor Yellow }
function Write-Err { param($msg) Write-Host "[ERROR] $msg" -ForegroundColor Red; exit 1 }

# 获取最新版本
function Get-LatestVersion {
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
    return $release.tag_name
}

# 下载并安装
function Install-Binary {
    param($Version)
    
    $Platform = "x86_64-pc-windows-msvc"
    $DownloadUrl = "https://github.com/$Repo/releases/download/$Version/$BinaryName-$Platform.zip"
    
    Write-Info "下载: $DownloadUrl"
    
    $TmpDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
    $ZipPath = Join-Path $TmpDir "archive.zip"
    
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath
    
    Write-Info "解压..."
    Expand-Archive -Path $ZipPath -DestinationPath $TmpDir -Force
    
    Write-Info "安装到 $InstallDir"
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Move-Item -Path (Join-Path $TmpDir "$BinaryName.exe") -Destination $InstallDir -Force
    
    Remove-Item -Path $TmpDir -Recurse -Force
    
    Write-Info "安装完成!"
}

# 添加到 PATH
function Add-ToPath {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -notlike "*$InstallDir*") {
        Write-Warn "$InstallDir 不在 PATH 中，正在添加..."
        [Environment]::SetEnvironmentVariable("Path", "$currentPath;$InstallDir", "User")
        $env:Path = "$env:Path;$InstallDir"
        Write-Info "已添加到 PATH (需要重启终端生效)"
    }
}

# 主函数
function Main {
    Write-Info "ZeroTier MCP Server 安装脚本"
    Write-Host ""
    
    $Version = Get-LatestVersion
    Write-Info "最新版本: $Version"
    
    Install-Binary -Version $Version
    Add-ToPath
    
    Write-Host ""
    Write-Host "可执行文件位置: $InstallDir\$BinaryName.exe"
    Write-Host ""
    Write-Info "MCP 配置示例:"
    Write-Host @"
{
  "mcpServers": {
    "zerotier": {
      "command": "$InstallDir\$BinaryName.exe",
      "env": {
        "ZEROTIER_CENTRAL_TOKEN": "your_token"
      }
    }
  }
}
"@
}

Main
