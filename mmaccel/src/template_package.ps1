$dir = Join-Path "package" "MMAccel_{0}"
$mmaccel_dir = Join-Path $dir "MMAccel"

if (!(Test-Path $mmaccel_dir)) {{
    New-Item $mmaccel_dir -ItemType Directory
}}

Copy-Item "target/release/d3d9.dll" $dir
Copy-Item "target/release/mmaccel.dll" $mmaccel_dir
Copy-Item "target/release/key_config.exe" $mmaccel_dir
Copy-Item "mmaccel/src/mmd_map.json" $mmaccel_dir
Copy-Item "key_config/src/order.json" $mmaccel_dir
Copy-Item "README.md" (Join-Path $dir "mmaccel_readme.md")
Copy-Item "README.md" (Join-Path $mmaccel_dir "mmaccel_readme.md")
Copy-Item "LICENSE-DEPENDENCIES"  $mmaccel_dir

Compress-Archive -Force -Path $dir -DestinationPath "package/MMAccel_{0}"