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

./rwfus -iI &&

echo &&
read -n 1 -r -s -p "Installation completed, press any key to close the window..." &&

exit 0;
