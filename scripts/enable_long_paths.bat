:: Enable long paths on Windows

REG ADD "HKLM\SYSTEM\CurrentControlSet\Control\FileSystem" /v LongPathsEnabled /t REG_DWORD /d 1 /f
git config --system core.longpaths true