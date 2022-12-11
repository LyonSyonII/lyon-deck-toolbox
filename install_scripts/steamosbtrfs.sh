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

echo;
read -n 1 -r -s -p "Installation completed, press any key to close the window..."

exit 0;