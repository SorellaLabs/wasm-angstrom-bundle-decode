# bundle-decode

### Prequisites
Install rust, node (npm), and wasm-pack.

For wasm-pack install:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```


### Steps

1. Build the rust code:
```bash
cd rs-bundle-decode
wasm-pack build --target bundler
```

2. Install deps in the typescript code:
```bash
cd ts-bundle-decode
npm i
```