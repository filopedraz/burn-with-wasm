# Burn with WASM

## Build

```bash
wasm-pack build --out-dir pkg --release --target web --no-typescript
mv ./pkg ./www/
```

## Run the Server

```bash
python3 -m http.server 8000
```

## Template

To create the repo, [this](https://github.com/rustwasm/wasm-pack) template was used.