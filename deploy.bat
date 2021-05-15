REM cargo build
mkdir deploy
copy "target\debug\techpriestsite.exe" "deploy\techpriestsite.exe"
copy "config.dhall" "deploy\config.dhall"
xcopy /S /Y "blog" "deploy\blog\"
xcopy /S /Y "css" "deploy\css\"
xcopy /S /Y "gallery" "deploy\gallery\"
xcopy /S /Y "static" "deploy\static\"
cd deploy
techpriestsite.exe
