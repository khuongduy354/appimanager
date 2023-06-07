# Todo
<!-- - Todo inside app   -->
<!-- - Rename variable, dir and file  --> 
- test, integration   
- implement Wrapper type that is absolute path 
- icon path option
- extract appname function
<!-- [Desktop Entry] -->
<!-- Version=0.13.23  -->
<!-- Type=Application -->
<!-- Name=appName -->
<!-- Comment=Application Description -->
<!-- TryExec=Path/to/AppImage -->
<!-- Exec=Path/to/AppImage -->
<!-- Actions=Editor -->
### FEATURES 
<!-- - list all .desktop of a path  -->
- remove .desktop  
- custom app name 
<!-- - not just appimage,any executables  -->
<!-- - array of apps  -->


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
2. canoncalize does not include linux ~
3. rust look for (integration) tests dir at top-level (same level as src),
bin crate cant use integration test, only function in lib.rs can be added
4. everything should work in absolute path 
5. implement absolute method for pathbuf  
Implement extensionpathbuf trait for path buf which has a abspath fn  
-> cannot override pathbuf from function or to, so that's the only way 
6. clap global option, that subcommand can take
7. as ref trick 
```rust
    fn parse(&self, icon_default: PathBuf, name_default: String) -> (PathBuf, String) {
// as_ref convert &Option<T> to Option<&T>
        let icon = self.icon.as_ref().unwrap_or(&icon_default);
        let name = self.name.as_ref().unwrap_or(&name_default);
        (icon.clone(), name.clone())
    }
```

