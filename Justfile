test_simple_add: 
  cargo build
  sudo target/debug/appim add ./test.AppImage

move:
  cargo build
  sudo target/debug/appim add ./test.AppImage -m ..

here:
  cargo build
  sudo target/debug/appim add ./test.AppImage -d "."
