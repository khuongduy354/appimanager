# Todo
- OSstr and str difference, is it ok to convert it one on one  
- Error Handling   
- Different file writing methods (bytes, buffers, str) and its advanteages   
- Todo inside app  
- Rename variable, dir and file 

# To learn 
1. why like this, or else moved borrowed, error from compiler
```rust 
let app_name = PathBuf::from(app_file.clone());
let app_name = app_name.file_stem().unwrap().to_str().unwrap();
```    
2. throw error when unwrap option, not result

# Learned 
1. canoncalize absolute path


