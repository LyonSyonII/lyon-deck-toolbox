![Screenshot](https://github.com/LyonSyonII/lyon-deck-toolbox/blob/main/assets/screenshot.png)

> An installer for a collection of tools and utilities to enhance the experience with the Steam Deck.

## Install
### Easy
[Download the `Lyon's Deck Toolbox.desktop`](https://github.com/LyonSyonII/lyon-deck-toolbox/releases/latest/download/Lyon's%20Deck%20Toolbox.desktop) file and double click it in the file explorer.

It will automatically update itself, so no need to download it again.

### For nerds
Clone the repository and run the `lyon-deck-toolbox` executable.
```bash
git clone https://github.com/LyonSyonII/lyon-deck-toolbox
cd lyon-deck-toolbox
./lyon-deck-toolbox
```

Or, if you want to compile it from source, install it with Cargo.
```bash
git clone https://github.com/LyonSyonII/lyon-deck-toolbox
cd lyon-deck-toolbox
cargo install --path .
```

## Tools
Currently Steam Deck Tools can automatically install:
- ### [Rwfus](https://github.com/ValShaped/rwfus)  
  Like a vinyl couch cover for your filesystem, Rwfus covers your Deck's /usr/ directory (and some others) allowing you to initialize and use pacman (the Arch Linux package manager) on the Steam Deck without losing packages when the next update comes out.  

- ### [CryoUtilities](https://github.com/CryoByte33/steam-deck-utilities)
  Scripts and utilities to enhance the Steam Deck experience, particularly performance.

- ### [Emudeck](https://github.com/dragoonDorise/EmuDeck)
  EmuDeck is a collection of scripts that allows you to autoconfigure your Steam Deck, it creates your roms directory structure and downloads all of the needed Emulators for you along with the best configurations for each of them.

- ### [Decky-loader](https://github.com/SteamDeckHomebrew/decky-loader)
  Decky Loader is a homebrew plugin launcher for the Steam Deck.
