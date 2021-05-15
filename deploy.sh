mkdir deploy
cp "target/debug/techpriestsite" "deploy/techpriestsite"
cp "config.dhall" "deploy/config.dhall"
cp "blog" "deploy/blog"
cp "css" "deploy/css"
cp "gallery" "deploy/gallery"
cp "static" "deploy/static"
cd deploy
./techpriestsite
