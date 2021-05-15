# site

Fork of [xe/site](https://github.com/Xe/site/). Mostly just retrofitted to not rely on nix and using my own branding. Many credits go to the original dev, don't be afraid to give 'em some love.

The original version of this site is build on the zlib license. This fork more or less retains that license, barring published work.

The changes to the fork are listed below, as well as how to deploy it.

My main aim in publishing this repo is to make it easy for others to do something similar: document what you need to change to make it usable.

## Fork changes

### Things needed for windows deployment

* Removed socket listener: this thing can just be served over port 4095 and I at times need to cross-platform dev. UnixListener doesn't exist on windows sooooo yeah. Not everyone has access to Unix only.
* Added special check for `\r` in the frontmatter: it is a newline format on windows.
* Removed nix dev files.
* Removed vscode settings.

### Branding related stuff

* Took out a ton of external site refs.
* Localized prism.css
* Disabled poking external services.
* Removed signalboost.
* Removed mi.
* Greatly simplify config.
* Took out prometheus. (Restored fire to humanity as well, zeus disposed).
* Removed unused crates.
* Renamed executable to just site.
* Changed avatar and favicons
* Removed talks - I don't give them.
* Slimmed down JSON parser.
