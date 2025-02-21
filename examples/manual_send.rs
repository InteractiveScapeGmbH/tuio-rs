use std::{net::{SocketAddr, IpAddr, Ipv4Addr}};

use rosc::OscPacket;
use tuio_rs::tuio11::osc_encode_decode::{EncodeOsc, OscEncoder};
use tuio_rs::tuio11::server::{SendOsc, UdpSender};
use tuio_rs::tuio11::Tuio11Cursor;
use tuio_rs::common::vector_2d::Vector2D;
fn main() {
    let source = "test".to_string();

    let sender = UdpSender::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 3333)).unwrap();

    let cursors = vec![
        Tuio11Cursor::new(0, Vector2D { x: 0., y: 0. }),
        Tuio11Cursor::new(1, Vector2D { x: 0.5, y: 0.5 }),
    ];

    let cursor_bundle = OscEncoder::encode_cursor_bundle(
        &cursors,
        source,
        0
    );

    sender.send_osc_packet(&OscPacket::Bundle(cursor_bundle)).expect("Sending OSC packet");
}
