$vsPath = "C:\Program Files\Microsoft Visual Studio\18\Community"
$cmdOutput = & cmd /c "`"$vsPath\Common7\Tools\VsDevCmd.bat`" -arch=x64 && set"
$cmdOutput | ForEach-Object { 
    if ($_ -match '^(\w+)=(.*)$') { 
        [Environment]::SetEnvironmentVariable($matches[1], $matches[2]) 
    } 
}
& cargo build --target wasm32-wasip1
