# Fix Vikey Registration
# Run this script as Administrator

Write-Host "Fixing Vikey TSF Registration..." -ForegroundColor Yellow

# 1. Remove corrupted registry keys
Write-Host "`n1. Removing old registry keys..." -ForegroundColor Cyan
try {
    Remove-Item "HKLM:\SOFTWARE\Classes\CLSID\{E6B8A6C0-1234-5678-9ABC-DEF012345678}" -Recurse -Force -ErrorAction SilentlyContinue
    Remove-Item "HKLM:\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}" -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "   ✓ Old keys removed" -ForegroundColor Green
} catch {
    Write-Host "   ! Error removing keys (may not exist): $_" -ForegroundColor Yellow
}

# 2. Re-register DLL
Write-Host "`n2. Re-registering DLL..." -ForegroundColor Cyan
$dllPath = Join-Path $PSScriptRoot "target\release\vikey_windows_tsf.dll"

if (Test-Path $dllPath) {
    $result = Start-Process -FilePath "regsvr32.exe" -ArgumentList "/s", $dllPath -Wait -PassThru
    
    if ($result.ExitCode -eq 0) {
        Write-Host "   ✓ DLL registered successfully" -ForegroundColor Green
    } else {
        Write-Host "   ✗ Registration failed with code: $($result.ExitCode)" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "   ✗ DLL not found: $dllPath" -ForegroundColor Red
    exit 1
}

# 3. Verify registration
Write-Host "`n3. Verifying registration..." -ForegroundColor Cyan

$clsidExists = Test-Path "HKLM:\SOFTWARE\Classes\CLSID\{E6B8A6C0-1234-5678-9ABC-DEF012345678}"
$tipExists = Test-Path "HKLM:\SOFTWARE\Microsoft\CTF\TIP\{E6B8A6C0-1234-5678-9ABC-DEF012345678}"

if ($clsidExists -and $tipExists) {
    Write-Host "   ✓ Both registry keys created" -ForegroundColor Green
    
    # Check DLL path
    $dllPathInRegistry = (Get-ItemProperty "HKLM:\SOFTWARE\Classes\CLSID\{E6B8A6C0-1234-5678-9ABC-DEF012345678}\InprocServer32")."(default)"
    Write-Host "   DLL Path: $dllPathInRegistry" -ForegroundColor Gray
    
    if ($dllPathInRegistry -like "*vikey_windows_tsf.dll") {
        Write-Host "   ✓ DLL path is correct" -ForegroundColor Green
    } else {
        Write-Host "   ✗ DLL path is wrong!" -ForegroundColor Red
    }
} else {
    Write-Host "   ✗ Registry keys not created" -ForegroundColor Red
    if (-not $clsidExists) { Write-Host "     Missing: CLSID key" -ForegroundColor Red }
    if (-not $tipExists) { Write-Host "     Missing: TIP key" -ForegroundColor Red }
}

# 4. Instructions
Write-Host "`n4. Next steps:" -ForegroundColor Cyan
Write-Host "   1. Restart Explorer: taskkill /f /im explorer.exe; start explorer.exe" -ForegroundColor White
Write-Host "   2. Or reboot computer" -ForegroundColor White
Write-Host "   3. Check: Settings → Language → Vietnamese → Keyboards" -ForegroundColor White
Write-Host "   4. Add keyboard → Look for 'Vikey - Vietnamese Input'" -ForegroundColor White

Write-Host "`n✓ Done!" -ForegroundColor Green
