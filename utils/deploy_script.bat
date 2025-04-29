cargo build -p my-first-game --release --target wasm32-unknown-unknown
mkdir -p ./deploy
cp ./my-first-game/index.html ./deploy/
cp ./target/wasm32-unknown-unknown/release/my-first-game.wasm ./deploy/
cp -r ./my-first-game/web/* ./deploy/ || true
mkdir -p ./deploy/my-first-game/assets/
cp -r ./my-first-game/assets/* ./deploy/my-first-game/assets/