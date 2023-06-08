# AppImanager - Create and manage .desktop entry <br> for your AppImages or any Executables.    
![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![git](https://img.shields.io/badge/GIT-E44C30?style=for-the-badge&logo=git&logoColor=white)
![cli](https://img.shields.io/badge/GNU%20Bash-4EAA25?style=for-the-badge&logo=GNU%20Bash&logoColor=white)

A Cli tool to quickly add your AppImage (or anything checked as executables) to the Applications menu of your desktop environment
by generating a .desktop file as an entry.

Simple, yet has many customizable options for the sake of convenience.


# Installation  
Cargo is required to run this app. After installed cargo, run: 

`cargo install appimanager`

# Usage 
```rust 
A simple cli to create and manage desktop entries from executables

Usage: appimanager [OPTIONS] <COMMAND>

Commands:
  add     Generate .desktop file
  list    List .desktop files 
  delete  Delete .desktop file by index (displayed by list subcommand) 
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --dest-dir <DEST_DIR>  Destination path that store all .desktop files  (default=~/.local/share/applications)
  -h, --help                 Print help
  -V, --version              Print version

Examples: 
1. appimanager add myfile.AppImage 

-m: move executable to a path before generating .desktop file   
-n: name property of to be generated .desktop file 
-i: icon (path) property of to be generated .desktop file 
 

2. appimanager list 

3. appimanager delete 0 // note: .desktop index starts from 0 

4. set custom directory of desktop files instead of ~/.local/share/applications (not recommended) for subcommands  

appimanager -d /path/here/ add myfile.AppImage //create myfile.desktop in /path/here/
appimanager -d /path/here/ list //list .desktop in /path/here
appimanager -d /path/here/ delete 0 //delete index 0 .desktop in /path/here


```
# Build from source   
Steps: 
```rust
git clone https: //github.com/khuongduy354/appimanager.git 
cd appimanager  
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

# Notes 
- For cargo: i published a crate called appim before, which is also the previous name of this repo, then i realized that not only AppImage is executable :) , this crate changes a bit, more features and has test coverage.



