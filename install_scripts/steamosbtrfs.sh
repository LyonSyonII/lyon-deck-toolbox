REPOSRC=https://github.com/popsUlfr/steamos-btrfs
LOCALREPO=$HOME/.local/share/steamosbtrfs

LOCALREPO_VC_DIR=$LOCALREPO/.git

if [ ! -d $LOCALREPO_VC_DIR ]
then
    git clone $REPOSRC $LOCALREPO
    cd $LOCALREPO
else
    cd $LOCALREPO
    git pull $REPOSRC
fi

chmod u+x install.sh
./install.sh

EXIT=$?
if [[ $EXIT -eq 0 ]]
then
    read -n 1 -r -s -p "Installation completed, press any key to close the window..."
else
    read -n 1 -r -s -p "Installation failed, press any key to close the window..."
fi
exit $EXIT;