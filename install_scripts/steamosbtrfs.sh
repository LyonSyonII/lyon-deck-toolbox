# Script extracted from the official .desktop installer

if [[ -f /usr/share/steamos-btrfs/install.sh ]] ; 
then 
    /usr/share/steamos-btrfs/install.sh ; 
else 
    t=\\"\\$(mktemp -d)\\" ; 
    curl -sSL https://gitlab.com/popsulfr/steamos-btrfs/-/archive/main/steamos-btrfs-main.tar.gz | tar -xzf - -C \\"\\$t\\" --strip-components=1 ; 
    \\"\\$t/install.sh\\" ; 
    rm -rf \\"\\$t\\" ; 
fi

echo;
read -n 1 -r -s -p "Installation completed, press any key to close the window..."

exit 0;