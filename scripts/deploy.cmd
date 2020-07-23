@setlocal && pushd "%~dp0.."

if EXIST E:\APPS\package.opk copy target\package.opk E:\APPS\package.opk

@endlocal && popd && exit /b %ERRORLEVEL%
