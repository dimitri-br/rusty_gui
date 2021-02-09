@echo off
echo Started: %date% %time%
rem compile vertex shaders
for %%f in (*.vert) do (
    if "%%~xf"==".vert" glslc.exe %%f -o %%f.spv
    if errorlevel 1 (
        echo Failed to compile shader %%f. Please read the error message[s] above
        exit /b 1
    )
    if "%%~xf"==".vert" echo %date% %time%: Compiled vertex shader: %%f
)
rem compile fragment shaders
for %%f in (*.frag) do (
    if "%%~xf"==".frag" glslc.exe %%f -o %%f.spv
    if errorlevel 1 (
        echo Failed to compile shader %%f. Please read the error message[s] above
        exit /b 1
    )
    if "%%~xf"==".frag" echo %date% %time%: Compiled fragment shader: %%f

)
echo Completed: %date% %time%. All shaders compiled without error
exit /b 0