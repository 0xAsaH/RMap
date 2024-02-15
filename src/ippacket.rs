pub struct IPPacket {
    version: u8, //Should be 4 bits
    header_length: u8, //Should be 4 bits
    type_of_service: u8,
    total_length: u16,
    identification: u16,
    flags: u8, //Should be 3 bits
    fragmentation_offset: u16, //Should be 13 bits
    time_to_live: u8,
    protocol: u8,
    header_checksum: u16,
    source_ip_address: u32,
    destination_ip_address: u32,
    options: u8 //Variable length
}

impl IPPacket {
    pub fn init() -> IPPacket {
        IPPacket {
            version: 0,
            header_length: 1,
            type_of_service: 2,
            total_length: 3,
            identification: 4,
            flags: 5,
            fragmentation_offset: 6,
            time_to_live: 7,
            protocol: 8,
            header_checksum: 9,
            source_ip_address: 10,
            destination_ip_address: 11,
            options: 12
        }
    }

    #[allow(dead_code)]
    pub fn show(self) {
        println!("Version: {0:?}\nHeader length:{1:?}\nType of service: {2:?}\nTotal length: {3:?}\nIdentification: {4:?}\nFlags: {5:?}\nFragmentation offset: {6:?}\nTime to live: {7:?}\nProtocol: {8:?}\nHeader checksum: {9:?}\nSource ip address: {10:?}\nDestination ip address: {11:?}\nOptions: {12:?}", self.version, self.header_length, self.type_of_service, self.total_length, self.identification, self.flags, self.fragmentation_offset, self.time_to_live, self.protocol, self.header_checksum, self.source_ip_address, self.destination_ip_address, self.options)
    }
}