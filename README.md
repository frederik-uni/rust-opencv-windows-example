```
OPENCV_LINK_LIBS=
OPENCV_LINK_PATHS=
OPENCV_INCLUDE_PATHS="+C:\tools\opencv\build\include"
```

## find available libs
`tree C:\tools\opencv /F` and search output for .lib

## Variable values
OPENCV_LINK_LIBS={filename}
OPENCV_LINK_PATHS={path_to_folder}

## script for opencv_world
opencv_world is the only lib in opencv right now
```
$files = Get-ChildItem -Path "C:\tools\opencv" -Recurse -Filter "opencv_world*.lib" |
 Where-Object { $_.Name -notmatch "d\.lib$" }
if ($files.Count -eq 0) {
  Write-Error "No matching OpenCV release libraries found (excluding debug builds)."
  exit 1
}

$libFile = $files | Sort-Object Name -Descending | Select-Object -First 1

$libName = [System.IO.Path]::GetFileNameWithoutExtension($libFile.Name)
$libPath = $libFile.DirectoryName

echo "Found library: $libName in $libPath"

echo "OPENCV_LINK_LIBS=$libName" >> $env:GITHUB_ENV
echo "OPENCV_LINK_PATHS=+$libPath" >> $env:GITHUB_ENV
          ``
