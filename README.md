# Ryzone
Ryzone is a linux graphical interface for manually controlling mobile Ryzen processors' power profiles. Utilises `libryzenadj-rs` (Rust bindings for the `libryzenadj` library) under the hood.

> ### ⚠️ WARNING
> *Use of this tool may cause system instability and could possibly break your hardware.*
> *Use of this tool is not sanctioned by AMD*

## OS Support
1. Please note that this app has been designed for any linux distro but has only been tested on Pop OS
2. You will need elevated privileges to run the app. For now, you will have to use `sudo -E`

## Work in progress
This app is a work in progress. Some key pending items:

1.  Power consumption tracking on external power
2.  Background daemon
3.  Pkexec (instead of sudo -E) 
4.  Packaging and release of appimage 
5.  Tray icon
   
## Building & Running the app
To build and run the release version, run 

`cargo build && sudo -E ./target/release/ryzone`

To build and run the release version, run

`cargo build --release && sudo -E ./target/release/ryzone`

## Building an Appimage
To build the appimage, you will need to set up the appimagetool AppImage first on your system. Use the following steps:

### Get and set up Appimagetool
1. Download the appimagetool (https://github.com/AppImage/appimagetool/releases)
2. Create a directory somewhere to store it: `mkdir -p ~/.local/bin`
3. Move file and rename without the suffix `mv /path/to/your/appimagetool-x86_64.AppImage ~/.local/bin/appimagetool`
4. Make appimagetool executable: `chmod +x ~/.local/bin/appimagetool`
5. Add path settings to the `~/.bashrc` file (or your terminal app settings file):  `export PATH="$PATH:$HOME/.local/bin"`
6. Reload settings `source ~/.bashrc`
7. Verify path `which appimagetool`

### Build and run app
1. Run command `cargo appimage`
2. Once done, the app will be in the `target/appimage` folder
3. Convert it to an executable `chmod +x ./ryzone.AppImage`
4. Run with *sudo -E* for example `sudo -E ./ryzone.AppImage`