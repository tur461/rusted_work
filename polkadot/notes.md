—— versions of para-chain and relay-chain code must be in sync (means same)

—— setup relay chain 
(2 validator nodes for 1 para collator)

— Alice will act as validator

./bin/polkadot --alice \
--validator \
--base-path /tmp/relay/alice \
--chain specs/relay-rawSpec.json \
--port 30333 \
--ws-port 9944

— Bob will act as validator

./bin/polkadot --bob \
--validator \
--base-path /tmp/relay/bob \
--chain specs/relay-rawSpec.json \
--port 30334 \
--ws-port 9945

—— setup parachain

 ../bin/para build-spec --chain para-plainSpec.json --disable-default-bootnode --raw > para-rawSpec.json

../bin/para build-spec --chain para-plainSpec.json --disable-default-bootnode --raw > para-rawSpec.json

../bin/para export-genesis-state --chain ../specs/para-rawSpec.json para-2000-genesis-state
../bin/para export-genesis-wasm --chain ../specs/para-rawSpec.json para-2000-wasm

— Alice will act as collator

./bin/para \
--alice \
--collator \
--force-authoring \
--chain specs/para-rawSpec.json \
--base-path tmp/parachain/alice \
--port 40333 \
--ws-port 8844 \
-- \
--execution wasm \
--chain rawSpec.json \
--port 30343 \
--ws-port 9977

