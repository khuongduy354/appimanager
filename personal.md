# Todo
<!-- - Todo inside app   -->
<!-- - Rename variable, dir and file  --> 
- test, integration  
- icon path option
<!-- [Desktop Entry] -->
<!-- Version=0.13.23  -->
<!-- Type=Application -->
<!-- Name=appName -->
<!-- Comment=Application Description -->
<!-- TryExec=Path/to/AppImage -->
<!-- Exec=Path/to/AppImage -->
Icon=Path/to/AppImage.icon
<!-- Actions=Editor -->

# To learn 
1. why like this, or else moved borrowed, error from compiler
```rust 
let app_name = PathBuf::from(app_file.clone());
let app_name = app_name.file_stem().unwrap().to_str().unwrap();
```    
2. throw error when unwrap option, not result 
3. Different file writing methods (bytes, buffers, str) and its advanteages   
4. OSstr and str difference, is it ok to convert it one on one   
5. Error Handling

# Learned 
1. canoncalize absolute path, which verify file exist

