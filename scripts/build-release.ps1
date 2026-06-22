# Aether Editor Build Script
# 一键构建发布版本

param(
    [switch]$Release,
    [switch]$Run
)

$ErrorActionPreference = "Stop"

Write-Host "=== Aether Editor Build ===" -ForegroundColor Cyan

# 确保Rust环境
$env:Path += ";$env:USERPROFILE\.cargo\bin"

# 构建Rust项目
Write-Host "Building Rust project..." -ForegroundColor Yellow
if ($Release) {
    cargo build --release
} else {
    cargo build
}

if ($LASTEXITCODE -ne 0) {
    Write-Host "Rust build failed!" -ForegroundColor Red
    exit 1
}

# 构建AI面板前端
Write-Host "Building AI panel frontend..." -ForegroundColor Yellow
Set-Location ai-panel
npm install
npm run build
Set-Location ..

# 复制前端产物到输出目录
$targetDir = if ($Release) { "target\release" } else { "target\debug" }
$aiPanelDir = "$targetDir\aether-ai-panel"

if (Test-Path $aiPanelDir) {
    Remove-Item -Recurse -Force $aiPanelDir
}
Copy-Item -Recurse ai-panel\dist $aiPanelDir

Write-Host "Build complete!" -ForegroundColor Green
Write-Host "Output: $targetDir\aether-app.exe" -ForegroundColor Green

if ($Run) {
    & "$targetDir\aether-app.exe"
}
