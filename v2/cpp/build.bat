@echo off
setlocal
set VCVARS="C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
if not exist %VCVARS% (
  echo vcvars64.bat introuvable, ajuste VCVARS dans build.bat
  exit /b 1
)
set VSWHERE_DIR=C:\Program Files (x86)\Microsoft Visual Studio\Installer
if exist "%VSWHERE_DIR%\vswhere.exe" set PATH=%PATH%;%VSWHERE_DIR%
if "%~1"=="" (
  echo usage: build.bat chemin\vers\fichier.cpp
  exit /b 1
)
call %VCVARS% >nul
if not exist "%~dp0lib" mkdir "%~dp0lib"
if not exist "%~dp0build" mkdir "%~dp0build"
copy /y "%~dp0..\src\target\release\seraplot.dll" "%~dp0lib\seraplot.dll" >nul
copy /y "%~dp0..\src\target\release\seraplot.dll.lib" "%~dp0lib\seraplot.dll.lib" >nul
cl /nologo /EHsc /std:c++17 /I "%~dp0include" "%~1" /Fo"%~dp0build/" /link "%~dp0lib\seraplot.dll.lib" /OUT:"%~dp0build\%~n1.exe"
copy /y "%~dp0lib\seraplot.dll" "%~dp0build\seraplot.dll" >nul
endlocal
