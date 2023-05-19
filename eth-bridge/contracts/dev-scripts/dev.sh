#!/bin/bash

set -exuo pipefail

anvil &
sleep 1
forge script script/Deploy.s.sol --rpc-url http://localhost:8545 --private-key=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast
cast send 0x5FC8d32690cc91D4c39d9d3abcBD16989F875707 --private-key=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 'grantRole(bytes32,address)' 0xa49807205ce4d355092ef5a8a18f56e8913cf4a201fbe287825b095693c21775 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
cast send 0x5FC8d32690cc91D4c39d9d3abcBD16989F875707 --private-key=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 'grantRole(bytes32,address)' 0x077a1d526a4ce8a773632ab13b4fbbf1fcc954c3dab26cd27ea0e2a6750da5d7 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
cast send 0x5FC8d32690cc91D4c39d9d3abcBD16989F875707 --private-key=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 'setActive(bool)' 1
cast rpc evm_setAutomine false
cast rpc evm_setIntervalMining 5
wait
