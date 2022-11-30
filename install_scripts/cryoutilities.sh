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

chmod u+x cryo_utilities.sh
./cryo_utilities.sh

echo;
read -n 1 -r -s -p "Installation completed, press any key to close the window..."

exit 0;

