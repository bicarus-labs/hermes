use ibc::core::ics02_client::client_consensus::QueryClientEventRequest;
use ibc::core::ics04_channel::channel::QueryPacketEventDataRequest;
use ibc::core::ics04_channel::packet::Sequence;
use ibc::query::QueryTxHash;
use tendermint_rpc::query::Query;

pub mod abci;
pub mod account;
pub mod balance;
pub mod denom_trace;
pub mod status;
pub mod tx;
pub mod version;

pub fn packet_query(request: &QueryPacketEventDataRequest, seq: Sequence) -> Query {
    Query::eq(
        format!("{}.packet_src_channel", request.event_id.as_str()),
        request.source_channel_id.to_string(),
    )
    .and_eq(
        format!("{}.packet_src_port", request.event_id.as_str()),
        request.source_port_id.to_string(),
    )
    .and_eq(
        format!("{}.packet_dst_channel", request.event_id.as_str()),
        request.destination_channel_id.to_string(),
    )
    .and_eq(
        format!("{}.packet_dst_port", request.event_id.as_str()),
        request.destination_port_id.to_string(),
    )
    .and_eq(
        format!("{}.packet_sequence", request.event_id.as_str()),
        seq.to_string(),
    )
}

pub fn header_query(request: &QueryClientEventRequest) -> Query {
    Query::eq(
        format!("{}.client_id", request.event_id.as_str()),
        request.client_id.to_string(),
    )
    .and_eq(
        format!("{}.consensus_height", request.event_id.as_str()),
        format!(
            "{}-{}",
            request.consensus_height.revision_number, request.consensus_height.revision_height
        ),
    )
}

pub fn tx_hash_query(request: &QueryTxHash) -> Query {
    Query::eq("tx.hash", request.0.to_string())
}
