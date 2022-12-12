REPOSRC=https://github.com/ValShaped/rwfus
LOCALREPO=$HOME/.local/share/rwfus

LOCALREPO_VC_DIR=$LOCALREPO/.git

if [ ! -d $LOCALREPO_VC_DIR ]
then
    git clone $REPOSRC $LOCALREPO
    cd $LOCALREPO
else
    cd $LOCALREPO
    git pull $REPOSRC
fi

./rwfus -iI

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
