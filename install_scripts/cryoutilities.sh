curl https://raw.githubusercontent.com/CryoByte33/steam-deck-utilities/main/install.sh | bash -s --;

if [[ $? -eq 0 ]]
then
    echo;
    read -n 1 -r -s -p "Installation completed, press any key to close the window..."
else
    echo;
    touch $HOME/.cache/lyon-deck-toolbox.err
    read -n 1 -r -s -p "Installation failed, press any key to close the window..."
fi

exit;
