#!/bin/sh

REPOSRC=https://github.com/CryoByte33/steam-deck-utilities
LOCALREPO=$HOME/.local/share/cryo-utilities

LOCALREPO_VC_DIR=$LOCALREPO/.git

if [ ! -d $LOCALREPO_VC_DIR ]
then
    git clone $REPOSRC $LOCALREPO
    cd $LOCALREPO
else
    cd $LOCALREPO
    git pull $REPOSRC
fi

echo 'Installation completed, you can close the window now'

