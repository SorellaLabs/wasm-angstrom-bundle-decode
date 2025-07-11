#!/bin/bash

# if directory `rs-bundle-decode` doesn't exist, exit
if [ ! -d "rs-bundle-decode" ]; then
    echo "Error: Directory 'rs-bundle-decode' does not exist"
    exit 1
fi

cp -r rs-bundle-decode/pkg ts-bundle-decode/pkg
rm ts-bundle-decode/pkg/.gitignore

wasm-pack build --target web rs-bundle-decode
# if the above command fails, exit
if [ $? -ne 0 ]; then
    echo "Error: wasm-pack build failed"
    exit 1
fi

# if directory `ts-bundle-decode` doesn't exist, exit
if [ ! -d "ts-bundle-decode" ]; then
    echo "Error: Directory 'ts-bundle-decode' does not exist"
    exit 1
fi

npm i --prefix ts-bundle-decode
npm run build --prefix ts-bundle-decode
# if the above command fails, exit
if [ $? -ne 0 ]; then
    echo "Error: npm start failed"
    exit 1
fi

