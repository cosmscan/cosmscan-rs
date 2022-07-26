#!/bin/sh
MONIKER=novatest
CHAIN_ID=nova
KEYRING=test
HOME_PATH=/data/chain/.gaiad

INDEX=1
SEQUENCE=1

while :;
do
        echo "generating keys"
        /bin/gaiad keys add validator$INDEX --keyring-backend test --home $HOME_PATH
        ADDR=$(/bin/gaiad keys show validator$INDEX -a --keyring-backend test --home $HOME_PATH)

        echo $ADDR

        echo ""
        echo "sent 100unova from validator to the $ADDR"
        /bin/gaiad tx bank send validator $ADDR 100unova \
                --chain-id $CHAIN_ID \
                --keyring-backend test \
                --home $HOME_PATH \
                --sequence $SEQUENCE \
                --node http://novachain:26657 \
                -y

        INDEX=$(( INDEX + 1))
        SEQUENCE=$(( SEQUENCE + 1))
        sleep 1
done