echo "Downloading EmuDeck AppImage, please wait" && 
mkdir -p ~/Applications && 
curl -L "$(curl -s https://api.github.com/repos/EmuDeck/emudeck-electron/releases/latest | grep -E 'browser_download_url.*AppImage' | cut -d '"' -f 4)" > ~/Applications/EmuDeck.AppImage && 
chmod +x ~/Applications/EmuDeck.AppImage && 
~/Applications/EmuDeck.AppImage

if [[ $? -eq 0 ]]
then
    echo;
    read -n 1 -r -s -p "Installation completed, press any key to close the window..."
else
    echo;
    touch $HOME/.cache/lyon-deck-toolbox.err
    read -n 1 -r -s -p "Installation failed, press any key to close the window..."
fi

exit