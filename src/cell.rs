use std::net::IpAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum DestroyReason {

}

#[derive(Serialize, Deserialize)]
struct SocketIp {
    ip: IpAddr,
    port: u16

}

#[derive(Serialize, Deserialize)]
enum RelayType {
    Begin,
    Data,
    End,
    Connected,
    SendMe,
    Extend{dh_exchange_stuff_x: Vec<u8>, ip: SocketIp},
    Extended{dh_exchange_stuff_y: Vec<u8>},
    Truncate,
    Drop,
}

#[derive(Serialize, Deserialize)]
enum CellType {
    Padding{padding: Vec<u8>},
    Create{dh_exchange_stuff_x: Vec<u8>},
    Created{dh_exchange_stuff_y: Vec<u8>},
    Relay{recognised: u32, stream_id: u32, digest: u32, data: RelayType, padding: Vec<u8>},
    Destroy{reason: DestroyReason},
    NetInfo{time: u32},
}

#[derive(Serialize, Deserialize)]
struct Cell {
    chain_id: u32,
    data: CellType,
    padding: Vec<u8>
}

