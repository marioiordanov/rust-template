#!/bin/bash
./build.sh
near delete t.marioyordanov.testnet marioyordanov.testnet
near create-account t.marioyordanov.testnet --masterAccount marioyordanov.testnet
near deploy t.marioyordanov.testnet --wasmFile res/mario_first_contract.wasm --initFunction 'new' --initArgs '{"solution": "mario"}'