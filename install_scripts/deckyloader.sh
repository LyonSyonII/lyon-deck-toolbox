curl -L https://github.com/SteamDeckHomebrew/decky-loader/raw/main/dist/install_release.sh | sh

if [[ $? -eq 0 ]]
then
    echo;
    read -n 1 -r -s -p "Installation completed, press any key to close the window..."
else
    echo;
    touch $HOME/.cache/lyon-deck-toolbox.err
    read -n 1 -r -s -p "Installation failed, press any key to close the window..."
fi

exit 0;