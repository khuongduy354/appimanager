# AppIm - Create .desktop entry for your AppImage   
![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![git](https://img.shields.io/badge/GIT-E44C30?style=for-the-badge&logo=git&logoColor=white)
![cli](https://img.shields.io/badge/GNU%20Bash-4EAA25?style=for-the-badge&logo=GNU%20Bash&logoColor=white)

A Cli tool to quickly add your AppImage to the Applications menu of your desktop environment

By generating a .desktop file as a Applications menu entry.

Simple, yet has many customizable options for the sake of convenience.


# Installation  
Cargo is required to run this app. After you installed cargo, run: 

`cargo install appim`

# Usage 
```rust 
appim add --help
Usage: appim add [OPTIONS] <APPIMAGE_PATH>

Arguments:
  <APPIMAGE_PATH>  Path of appimage file

Options:
  -d, --dest-dir <DEST_DIR>  Destination of desktop file [default: ~/.local/share/applications]
  -m, --move-dir <MOVE_DIR>  Move appimage file to some location before creating desktop file
  -h, --help                 Print help   

Examples: 
//create myfile.desktop in ~/.local/share/applications by default
appim add myfile.AppImage     
 
//Usually, after downloaded, the AppImage is in downloads folder,
//hence you may want to use the -m flag to move the AppImage to your personal apps folder, before create .desktop entry 
appim add myfile.AppImage -m "~/apps"  
//myfile.AppImage is moved in ~/apps, myfile.desktop also got created with correct exec_path

// create myfile.desktop in your/path/here 
appim add myfile.AppImage -d "your/path/here"  
```
# Build from source   
Steps: 
```rust
git clone https: //github.com/khuongduy354/appim.git 
cd appim  
cargo run //to build and run 
cargo build //to build only
cargo test //to test
``` 
Additional things:
- justfile (Makefile alternatives) to help automate stuffs (you need [just](https://crates.io/crates/just) to run it)  
- personal.md docs my learning stuffs

# Contributing 
- I really need mentoring on: app features, Rust best practices, implementations, or structures of the app.
I'd be very appreciate if anyone could give any sort of guidances. 
- Feels free to raise issues and suggest features.  
- Don't forget to leave a star!




