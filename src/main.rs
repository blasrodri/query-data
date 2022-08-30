use ibc::{
    core::{
        ics04_channel::packet::Packet,
        ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
    },
    events::IbcEvent,
};
use ibc_proto::ibc::core::{
    channel::v1::{
        Packet as RawPacket, QueryChannelResponse, QueryChannelsResponse,
        QueryNextSequenceReceiveResponse, QueryPacketAcknowledgementResponse,
        QueryPacketCommitmentResponse, QueryPacketReceiptResponse,
    },
    client::v1::{IdentifiedClientState, QueryClientStateResponse, QueryConsensusStateResponse},
    connection::v1::{QueryConnectionResponse, QueryConnectionsResponse},
};
use ibc_rpc::IbcApiClient;
use ibc_rpc::{BlockNumberOrHash, ConnHandshakeProof};
use sp_core::H256;
use std::{collections::hash_map::HashMap, str::FromStr};
use subxt::DefaultConfig;

const HEIGHT: u32 = 57;

#[tokio::main]
async fn main() {
    let client = subxt::ClientBuilder::new()
        .set_url("ws://127.0.0.1:9988")
        .build::<subxt::DefaultConfig>()
        .await
        .unwrap();
    let client_id = ClientId::from_str("11-beefy-1").unwrap();
    let connection_id = ConnectionId::new(0);
    let channel_id = ChannelId::new(0);
    let port_id = PortId::from_str("ping").unwrap();

    let client_consensus_state = query_client_consensus(&client, HEIGHT, client_id.clone()).await;
    let client_state = query_client_state(&client, 30, client_id.clone()).await;
    let connection_end = query_connection_end(&client, HEIGHT, connection_id.clone()).await;
    let channel_end = query_channel_end(&client, HEIGHT, channel_id, port_id.clone()).await;
    let packets = query_packets(&client, &port_id, &channel_id, vec![1]).await;
    let commitment = query_packet_commitment(&client, HEIGHT, &port_id, &channel_id, 1).await;
    let acknowledgement =
        query_packet_acknowledgement(&client, HEIGHT, &port_id, &channel_id, 1).await;
    let next_seq_recv = query_next_sequence_recv(&client, HEIGHT, &port_id, &channel_id).await;
    let packet_reciept = query_packet_receipt(&client, HEIGHT, &port_id, &channel_id, 1).await;
    let block_numbers = (1..HEIGHT)
        .into_iter()
        .map(|i| BlockNumberOrHash::<H256>::Number(i))
        .collect();
    let events = query_events_at(&client, block_numbers).await;
    let clients = query_clients(&client).await;
    let connections = query_connections(&client).await;
    let channels = query_channels(&client).await;
    let identified_client =
        query_channel_client(&client, HEIGHT, channel_id, port_id.clone()).await;
    let conn_handshake_proof = generate_connection_handshake_proof(
        &client,
        HEIGHT,
        client_id.clone(),
        connection_id.clone(),
    )
    .await;
    let identified_connection = query_connections_using_client(&client, HEIGHT, client_id).await;
    let connection_channels = query_channels_using_connection(&client, HEIGHT, connection_id).await;
    dbg!(events);
}

async fn query_client_consensus(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    client_id: ClientId,
) -> QueryConsensusStateResponse {
    IbcApiClient::<u32, H256>::query_client_consensus_state(
        &*client.rpc().client,
        Some(at),
        client_id.to_string(),
        0,
        0,
        true,
    )
    .await
    .unwrap()
}

async fn query_client_state(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    client_id: ClientId,
) -> QueryClientStateResponse {
    IbcApiClient::<u32, H256>::query_client_state(&*client.rpc().client, at, client_id.to_string())
        .await
        .unwrap()
}

async fn query_connection_end(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    connection_id: ConnectionId,
) -> QueryConnectionResponse {
    IbcApiClient::<u32, H256>::query_connection(
        &*client.rpc().client,
        at,
        connection_id.to_string(),
    )
    .await
    .unwrap()
}

async fn query_channel_end(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    channel_id: ChannelId,
    port_id: PortId,
) -> QueryChannelResponse {
    IbcApiClient::<u32, H256>::query_channel(
        &*client.rpc().client,
        at,
        channel_id.to_string(),
        port_id.to_string(),
    )
    .await
    .unwrap()
}

async fn query_connections(client: &subxt::Client<DefaultConfig>) -> QueryConnectionsResponse {
    IbcApiClient::<u32, H256>::query_connections(&*client.rpc().client)
        .await
        .unwrap()
}

async fn query_channels(client: &subxt::Client<DefaultConfig>) -> QueryChannelsResponse {
    IbcApiClient::<u32, H256>::query_channels(&*client.rpc().client)
        .await
        .unwrap()
}

async fn query_clients(client: &subxt::Client<DefaultConfig>) -> Vec<IdentifiedClientState> {
    IbcApiClient::<u32, H256>::query_clients(&*client.rpc().client)
        .await
        .unwrap()
}

async fn generate_connection_handshake_proof(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    client_id: ClientId,
    connection_id: ConnectionId,
) -> ConnHandshakeProof {
    IbcApiClient::<u32, H256>::generate_conn_handshake_proof(
        &*client.rpc().client,
        at,
        client_id.to_string(),
        connection_id.to_string(),
    )
    .await
    .unwrap()
}

async fn query_connections_using_client(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    client_id: ClientId,
) -> Vec<ibc_proto::ibc::core::connection::v1::IdentifiedConnection> {
    IbcApiClient::<u32, H256>::query_connection_using_client(
        &*client.rpc().client,
        at,
        client_id.to_string(),
    )
    .await
    .unwrap()
}

async fn query_channels_using_connection(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    connection_id: ConnectionId,
) -> QueryChannelsResponse {
    IbcApiClient::<u32, H256>::query_connection_channels(
        &*client.rpc().client,
        at,
        connection_id.to_string(),
    )
    .await
    .unwrap()
}

async fn query_channel_client(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    channel_id: ChannelId,
    port_id: PortId,
) -> IdentifiedClientState {
    IbcApiClient::<u32, H256>::query_channel_client(
        &*client.rpc().client,
        at,
        channel_id.to_string(),
        port_id.to_string(),
    )
    .await
    .unwrap()
}

async fn query_proof(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    keys: Vec<Vec<u8>>,
) -> Vec<u8> {
    IbcApiClient::<u32, H256>::query_proof(&*client.rpc().client, at, keys)
        .await
        .unwrap()
        .proof
}

async fn query_packets(
    client: &subxt::Client<DefaultConfig>,
    port_id: &PortId,
    channel_id: &ChannelId,
    seqs: Vec<u64>,
) -> Vec<Packet> {
    let packets: Vec<RawPacket> = IbcApiClient::<u32, H256>::query_packets(
        &*client.rpc().client,
        channel_id.to_string(),
        port_id.to_string(),
        seqs.clone(),
    )
    .await
    .expect("lol");

    packets
        .into_iter()
        .map(|raw_packet| raw_packet.try_into().unwrap())
        .collect::<Vec<Packet>>()
}

async fn query_packet_commitment(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    port_id: &PortId,
    channel_id: &ChannelId,
    seq: u64,
) -> QueryPacketCommitmentResponse {
    IbcApiClient::<u32, H256>::query_packet_commitment(
        &*client.rpc().client,
        at,
        channel_id.to_string(),
        port_id.to_string(),
        seq,
    )
    .await
    .unwrap()
}

async fn query_packet_acknowledgement(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    port_id: &PortId,
    channel_id: &ChannelId,
    seq: u64,
) -> QueryPacketAcknowledgementResponse {
    IbcApiClient::<u32, H256>::query_packet_acknowledgement(
        &*client.rpc().client,
        at,
        channel_id.to_string(),
        port_id.to_string(),
        seq,
    )
    .await
    .unwrap()
}

async fn query_next_sequence_recv(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    port_id: &PortId,
    channel_id: &ChannelId,
) -> QueryNextSequenceReceiveResponse {
    IbcApiClient::<u32, H256>::query_next_seq_recv(
        &*client.rpc().client,
        at,
        channel_id.to_string(),
        port_id.to_string(),
    )
    .await
    .unwrap()
}

async fn query_packet_receipt(
    client: &subxt::Client<DefaultConfig>,
    at: u32,
    port_id: &PortId,
    channel_id: &ChannelId,
    seq: u64,
) -> QueryPacketReceiptResponse {
    IbcApiClient::<u32, H256>::query_packet_receipt(
        &*client.rpc().client,
        at,
        channel_id.to_string(),
        port_id.to_string(),
        seq,
    )
    .await
    .unwrap()
}

/// Get all ibc events deposited in finalized blocks
async fn query_events_at(
    client: &subxt::Client<DefaultConfig>,
    block_numbers: Vec<BlockNumberOrHash<H256>>,
) -> Vec<IbcEvent> {
    let events: HashMap<String, Vec<IbcEvent>> =
        IbcApiClient::<u32, H256>::query_events(&*client.rpc().client, block_numbers)
            .await
            .unwrap();

    events.into_values().flatten().collect()
}
