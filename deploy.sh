rm -rf deploy
mkdir deploy
cp -r "target/debug/techpriestsite" "deploy/techpriestsite"
cp -r "config.dhall" "deploy/config.dhall"
cp -r "blog" "deploy/blog"
cp -r "css" "deploy/css"
cp -r "gallery" "deploy/gallery"
cp -r "static" "deploy/static"
cd deploy
./techpriestsite
