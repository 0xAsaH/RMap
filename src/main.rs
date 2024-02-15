mod tcppacket;
mod ippacket;

use tcppacket::TCPPacketHeader;
use ippacket::IPPacket;

fn main() {
    let mut tcp_packet = TCPPacketHeader::init();
    let ip_packet = IPPacket::init();

    tcp_packet.set_dest(80);
    tcp_packet.set_source(80);
    tcp_packet.show();

    println!("\n\n");

    ip_packet.show();
}

//When implementing source and destination addresses, use u32 and split into 4 sets of hex for instance 192.168 would be c0 a8 