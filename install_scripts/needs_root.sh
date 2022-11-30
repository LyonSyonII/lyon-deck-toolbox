if [[ $(passwd -S) == *" NP "* ]]; then
    echo 'This utility REQUIRES a password set on the Steam Deck.'
    echo 'A password will be set now:'
    echo;
    passwd
fi
