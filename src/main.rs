#![feature(core_intrinsics)]

use std::env;
use std::io::prelude::*;
use std::path::Path;
use core::intrinsics::*;

extern crate apollo;
extern crate hex;
extern crate csv;


use apollo::{parameters::{TOTAL_MESSAGE_LENGTH_BYTES, CALLSIGN, START_END_HEADER, BARE_MESSAGE_LENGTH_BYTES}, telemetry::{decode_packet, values_from_packet, DecodedDataPacket}};
use serde::{Serialize, Deserialize};

mod sensors;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    timestamp: u64,
    packetdata: DecodedDataPacket,
    original_packet: String,
    decoded_packet: String,
}

#[cfg(test)]
mod tests {
    extern crate apollo;
    extern crate hex;
    
    use apollo::parameters::{TOTAL_MESSAGE_LENGTH_BYTES, BARE_MESSAGE_LENGTH_BYTES};

    use crate::sensors;

    pub fn get_random_packet() -> [u8; TOTAL_MESSAGE_LENGTH_BYTES] {
        unsafe { crate::sensors::init() };
        apollo::generate_packet(sensors::get_temperature(), sensors::get_altitude(), sensors::get_voltage(), sensors::get_location())
    }
    
    #[test]
    #[no_mangle]
    fn try_corrupt_decode() {
        // debug the similarity bitmask:
        // let (mask, _) = find_packet_similarities();
        // println!("");
        // for i in mask {
        //     print!("{:08b} ", i);
        // }

        println!("");
        const CORRUPT_BYTES: usize = 18;
        let mut _packet: [u8; TOTAL_MESSAGE_LENGTH_BYTES] = get_random_packet();
        let mut _corrupt_packet = _packet.clone();
        for i in 0..CORRUPT_BYTES {
            _corrupt_packet[i] = 0x00;
        }
        for i in _corrupt_packet {
            print!("{:02x?} ", i);
        }
        println!("");
        let mut _packet_data: [u8; BARE_MESSAGE_LENGTH_BYTES] = [0u8; BARE_MESSAGE_LENGTH_BYTES];
        _packet_data.copy_from_slice(&_packet[0..BARE_MESSAGE_LENGTH_BYTES]);

        for i in _packet_data {
            print!("{:02x?} ", i);
        }
        
        assert_eq!(_packet_data, crate::try_decode_packet(_corrupt_packet))
    }
}

#[inline]
pub fn try_decode_packet(_packet: [u8; TOTAL_MESSAGE_LENGTH_BYTES]) -> [u8; BARE_MESSAGE_LENGTH_BYTES] {
     // for i in 0..14 {
    //     _packet[i] = 0x00;
    // }
    // for i in _packet {
    //     print!("{:02x?} ", i);
    // } println!();

    let decoded_packet = decode_packet(_packet, &[0u8]);
    
    // println!();
    // for i in decoded_packet {
    //     print!("{:02x?} ", i);
    // }

    for i in 0..CALLSIGN.len() {
        assert_eq!(decoded_packet[core::mem::size_of_val(&START_END_HEADER) + i], CALLSIGN[i]);
    }
    
    decoded_packet

}

#[inline]
pub fn get_random_packet() -> [u8; TOTAL_MESSAGE_LENGTH_BYTES] {
    unsafe { crate::sensors::init() };
    apollo::generate_packet(sensors::get_temperature(), sensors::get_altitude(), sensors::get_voltage(), sensors::get_location())
}

pub unsafe fn _generate_decode_loop() {
    let mut i: u128 = 0;
    loop {
        let _packet = get_random_packet();
        let decoded_packet = try_decode_packet(_packet);
        let packet_values = values_from_packet(decoded_packet);
        let _altitude = packet_values.data_arr[0];
        let _voltage = packet_values.data_arr[1];
        let _temperature = packet_values.data_arr[2];
        let _latitude = packet_values.data_arr[3];
        let _longitude = packet_values.data_arr[4];
    
        let packet_data = crate::Data {
            timestamp: std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            packetdata: DecodedDataPacket {
                altitude: _altitude,
                voltage: _voltage,
                temperature: _temperature,
                latitude: _latitude,
                longitude: _longitude
            },
            original_packet: format!("{:02x?}", _packet),
            decoded_packet: format!("{:02x?}", decoded_packet),
        };
        debug_assert_eq!(packet_data.decoded_packet[0..decoded_packet.len()], packet_data.original_packet[0..decoded_packet.len()]);
    
        i = unchecked_add(i, 1);
        
        if unlikely(i % 100_000 == 0) {
            println!("{}", i);
            println!("{:#?}", packet_data);
        }
    }
}

#[inline]
fn cli_main() {
    let args: Vec<String> = env::args().collect();
    if &args.len() == &1usize {
        eprintln!("No input data provided.");
        return
    }
    debug_assert_ne!(&args.len(), &1usize, "\nNo input data");
    let hex_input = hex::decode(&args[1]).unwrap();
    assert_eq!(hex_input.len(), TOTAL_MESSAGE_LENGTH_BYTES, "\nIncorrect length input array. Expected {}, found {}.", TOTAL_MESSAGE_LENGTH_BYTES, hex_input.len());
    let mut _packet: [u8; TOTAL_MESSAGE_LENGTH_BYTES] = [0u8; TOTAL_MESSAGE_LENGTH_BYTES];
    
    
    _packet.copy_from_slice(&hex_input[..TOTAL_MESSAGE_LENGTH_BYTES]);

    let decoded_packet = try_decode_packet(_packet);
    
    let packet_values = values_from_packet(decoded_packet);

    // for i in packet_values {
    //     println!("{:02x?} ", i.to_be_bytes());
    // } println!();

    let _altitude = packet_values.data_arr[0];
    let _voltage = packet_values.data_arr[1];
    let _temperature = packet_values.data_arr[2];
    let _latitude = packet_values.data_arr[3];
    let _longitude = packet_values.data_arr[4];

    println!("ALTITUDE    :  {:.4} METERS ASL", _altitude);
    println!("VOLTAGE     :  {:.4} VOLTS", 12.0f32+_voltage);
    println!("TEMPERATURE :  {:.4} DEGREES CELCIUS", _temperature);
    println!("LATITUDE    : {}{:.4} DEGREES", ( if unlikely(_longitude > 0.0) {""} else {" "} ), _latitude);
    println!("LONGITUDE   : {}{:.4} DEGREES", ( if likely(_latitude > 0.0) {""} else {" "} ), _longitude);
    println!("CURRENT LOCATION:");
    println!("https://www.google.com/maps/@{:.},{:.},19z?entry=ttu", _latitude, _longitude);

    let packet_data = crate::Data {
        timestamp: std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs(),
        packetdata: DecodedDataPacket {
            altitude: _altitude,
            voltage: _voltage,
            temperature: _temperature,
            latitude: _latitude,
            longitude: _longitude
        },
        original_packet: format!("{:02x?}", _packet),
        decoded_packet: format!("{:02x?}", decoded_packet),
    };

    let path = Path::new("data.yml");
    if let Ok(exists) = path.try_exists() {
        if !exists {
            std::fs::File::create(path).unwrap();
        }
    }

    let mut data_file = std::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .expect("cannot open file");

    let needs_headers = data_file.seek(std::io::SeekFrom::End(0)).unwrap() == 0;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(needs_headers)
        .from_writer(&mut data_file);
        

    wtr.serialize(&packet_data).unwrap();


}


fn main() {
    cli_main();
    unsafe { _generate_decode_loop() };
}
