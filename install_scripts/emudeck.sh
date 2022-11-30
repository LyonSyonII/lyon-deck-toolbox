#!/bin/sh

echo "Downloading EmuDeck AppImage, please wait" && 
mkdir -p ~/Applications && 
curl -L "$(curl -s https://api.github.com/repos/EmuDeck/emudeck-electron/releases/latest | grep -E 'browser_download_url.*AppImage' | cut -d '"' -f 4)" > ~/Applications/EmuDeck.AppImage && 
chmod +x ~/Applications/EmuDeck.AppImage && 
~/Applications/EmuDeck.AppImage && 
exit