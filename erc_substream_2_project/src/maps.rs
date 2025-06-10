use crate::abi::{self};
use crate::pb::erc20::types::v1::{TransferEvents, TransferEvent};
use abi::erc20::events::Transfer;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

// substreams build 
// Generates the necessary Protobufs specified in the substreams.yaml file.
// Compiles the Rust code.
// Creates a Substreams package file (.spkg).


// The substreams CLI is used to generate the associated Rust code for the protobuf.
// substreams protogen ./substreams.yaml --exclude-paths="sf/ethereum,sf/substreams,google" <<<<<<<<<<<<
// ^^^ The pairing code is generated and saved into the src/pb/eth.erc721.v1.rsRust file.


// substreams gui -e mainnet.eth.streamingfast.io:443 substreams.yaml map_my_data -s 12369621 -t +1
// substreams gui -e mainnet.eth.streamingfast.io:443 substreams.yaml map_transfers -s 12369621 -t +1

#[substreams::handlers::map]
pub fn map_transfers(block: Block) -> Result<TransferEvents, Error> {
    let  transfers = map_events(&block);

    Ok(TransferEvents {
        transfers
    })
}

pub fn map_events(block: &Block) -> Vec<TransferEvent> {
    let mut transfers = vec![];

    for log in block.logs() {
        
        if let Some(transfer) = Transfer::match_and_decode(log.log) {
            transfers.push(decode_transfer(transfer, log));
            continue;
        }

        // no data
    }

    return transfers;
}

fn decode_transfer(event: Transfer, log: LogView) -> TransferEvent {
    TransferEvent {
        // contract address
        contract: Hex::encode(log.address()),

        // event payload
        from: Hex::encode(event.from),
        to: Hex::encode(event.to),
        value: event.value.to_string(),

        // trace information
        tx_id: Hex::encode(&log.receipt.transaction.hash),
        block_index: log.log.block_index.into(),
        index: log.index()
    }
}
