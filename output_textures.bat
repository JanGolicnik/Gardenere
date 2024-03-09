@echo off
setlocal enabledelayedexpansion

set "res_folder=.\res"
set "textures_file=.\res\textures.txt"

if exist "%textures_file%" del "%textures_file%"

for /r "%res_folder%" %%F in (*.png) do (
    set "file_path=%%~pnxF"
    set "file_path=!file_path:%res_folder%=!"
    echo !file_path:~1! >> "%textures_file%"
)