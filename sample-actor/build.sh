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

echo "copy to dev-runner"
cd ..
cp -r target/wasm32-unknown-unknown/release/sample_actor.wasm ../../dev-runner/local/b-node/