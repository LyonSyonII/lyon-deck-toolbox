#!/bin/sh

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

echo -n "Installation completed! Press any key to close the window." && read