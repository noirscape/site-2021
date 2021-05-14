REM cargo build
mkdir deploy
copy "target\debug\xesite.exe" "deploy\xesite.exe"
copy "config.dhall" "deploy\config.dhall"
xcopy /S /Y "blog" "deploy\blog\"
xcopy /S /Y "css" "deploy\css\"
xcopy /S /Y "gallery" "deploy\gallery\"
xcopy /S /Y "signalboost.dhall" "deploy\signalboost.dhall*"
xcopy /S /Y "static" "deploy\static\"
xcopy /S /Y "talks" "deploy\talks\"
