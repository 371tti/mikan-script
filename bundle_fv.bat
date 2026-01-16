@echo off
setlocal

set ROOT=%~dp0
set FV_DIR=%ROOT%fv

if not exist "%FV_DIR%" mkdir "%FV_DIR%"

echo Building fovia-cli (release)...
cargo build --release --manifest-path "%ROOT%fovia-cli\Cargo.toml" || goto :error

echo Building fovia-c (release)...
cargo build --release --manifest-path "%ROOT%fovia-c\Cargo.toml" || goto :error

echo Building fovia-vm (release)...
cargo build --release --manifest-path "%ROOT%fovia-vm\Cargo.toml" || goto :error

echo Copying executables to %FV_DIR%...
copy /Y "%ROOT%fovia-cli\target\release\fovia-cli.exe" "%FV_DIR%\fv.exe" >nul || goto :error
copy /Y "%ROOT%fovia-c\target\release\fovia-c.exe" "%FV_DIR%\fovia-c.exe" >nul || goto :error
copy /Y "%ROOT%fovia-vm\target\release\fovia-vm.exe" "%FV_DIR%\fovia-vm.exe" >nul || goto :error

echo Done.
exit /b 0

:error
echo Failed.
exit /b 1
