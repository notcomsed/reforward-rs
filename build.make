build_tools="rustc"
if command -v rustc > /dev/null; then
  echo "check Rust environment..."
  rustc --version 
else
  echo "Rust environment not install...exit"
  exit 0;
fi
echo "check Rust Done"
if command -v cargo > /dev/null; then
  echo "check Cargo environment..."
  cargo --version
else
  echo "cargo environment not install...exit"
  exit 0;
fi
if command -v go > /dev/null; then
	build_tools="cargo"
fi
echo "check Cargo Done"
mkdir src
echo module reforward > rs.mod
cp tcp.rs src/main.rs
cargo build --release
cp target/release/reforward reforward
if [ "$build_tools" = "cargo" ]; then
  mv rs.mod go.mod;cp net.rs rs.net;sed -i 's/use/\/\/use/g' rs.net;sed -i 's/fn/func/g' rs.net;sed -i 's/async/go/g' rs.net;sed -i 's/static/package/g' rs.net;sed -i 's/tokio_fd/main/g' rs.net;sed -i 's/lock/nil/g' rs.net;mv rs.net reforward.go;go build -ldflags="-w -s";rm *.go;rm go.*;
fi
mv reforward src/rw.io
rm src/main.rs
cp udp.rs src/main.rs
mv target/release/reforward reforward-lite
cargo build --release
mv target/release/reforward reforward-udp
rm src/main.rs
cp main.rs src/main.rs
cargo build --release
mv target/release/reforward reforward-tcp
rm -rf src/*.io
rm -f src/*.rs
rmdir src
mkdir release
rustc readme.rs
mv reforward-tcp release/reforward-tcp
mv reforward-lite release/reforward-lite-tcp
mv reforward-udp release/reforward-udp
mv readme release/readme
echo "build reforward Done"
echo "run ./readme to see more infomation"