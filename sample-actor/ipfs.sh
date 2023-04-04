#!/bin/bash

cd $(dirname $0)
cd impl

cargo build --target wasm32-unknown-unknown --release

if [ $? -ne 0 ]; then
  exit 1
fi


if ! command -v tas &> /dev/null
then
    cargo install tea-actorx-signer --version 0.2.0-dev.5
fi

tas ../target/wasm32-unknown-unknown/release/sample_actor.wasm
cd ..

echo "upload to ipfs"
ipfs add target/wasm32-unknown-unknown/release/sample_actor.wasm --api /ip4/64.227.49.206/tcp/5001/p2p/12D3KooWScg336x2Rzc97ZnHbYAEd592P3DqkYJFZRQneGopjsyT
