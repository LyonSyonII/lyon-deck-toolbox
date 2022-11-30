if [[ $(passwd -S) == *" P "* ]]; then
    echo 'This utility REQUIRES a password set on the Steam Deck.'
    echo 'A password will be set now:'
    passwd
fi
