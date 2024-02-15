pub struct TCPPacketHeader {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgement_number: u32,
    data_offset: u8, //Supposed to be 4 bits
    control_flags: u8, //Supposed to be 9 bits
    window_size: u16,
    check_sum: u16,
    urgent_pointer: u16,
    options: u8, //Variable length
    data: u8 //Can be much larger
}

impl TCPPacketHeader {
    pub fn init() -> TCPPacketHeader {
        TCPPacketHeader {
            source_port: 0,
            destination_port: 1,
            sequence_number: 2,
            acknowledgement_number: 3,
            data_offset: 4,
            control_flags: 5,
            window_size: 6,
            check_sum: 7,
            urgent_pointer: 8,
            options: 9,
            data: 10
        }
    }

    #[allow(dead_code)]
    pub fn set_source(&mut self, value: u16) {
        self.source_port = value;
    }

    #[allow(dead_code)]
    pub fn set_dest(&mut self, value: u16) {
        self.destination_port = value;
    }

    #[allow(dead_code)]
    pub fn set_data_offset(&mut self, value: u8) {
        if value <= 4 {
            self.data_offset = value;
        } else {
            println!("Data offset must be lower than or equal to 4");
        }
    }

    #[allow(dead_code)]
    pub fn set_control_flags(&mut self, value: u8) {
        if value <= 9 {
            self.control_flags = value;
        } else {
            println!("Control flags must be lower than or equal to 9");
        }
    }

    #[allow(dead_code)]
    pub fn show(self) {
        println!("Source port: {0:?}\nDestination port: {1:?}\nSequence number: {2:?}\nAcknowledgment number: {3:?}\nData offset: {4:?}\nControl flags: {5:?}\nWindow size: {6:?}\nCheck sum: {7:?}\nUrgent pointer: {8:?}\nOptions: {9:?}\nData: {10:?}", self.source_port, self.destination_port, self.sequence_number, self.acknowledgement_number, self.data_offset, self.control_flags, self.window_size, self.check_sum, self.urgent_pointer, self.options, self.data);
    }
}