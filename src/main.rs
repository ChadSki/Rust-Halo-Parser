mod halo_map;
use halo_map::HaloMap;

use std::str;
use std::env;
use std::fs::File;
use std::io::Read;

extern crate byteorder;
use byteorder::{BigEndian, ByteOrder};

fn main() {
    let mut st : Vec<u8> = Vec::new();
    let arguments : Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        println!("rust_halo_parser <path_to_map>");
        return;
    }
    let path = arguments[1].clone();
    match File::open(path) {
        Err(_) => { println!("Couldn't open..."); },
        Ok(mut m) => {
            match m.read_to_end(&mut st) {
                Err(_) => { println!("Couldn't read..."); },
                Ok(l) => {
                    println!("Read {} bytes.",l);
                    match HaloMap::from_buffer(&st[..]) {
                        None => println!("Failed parse."),
                        Some(m) => {
                            println!("Succeeded parse.");
                            println!("Map name: {}. Build: {}",m.name, m.build);
                            println!("Scenario Tag: {}",m.tags[m.scenario_tag as usize].path);
                            let tag_count = m.tags.len();

                            let mut q = 0;
                            for i in m.tags {
                                q = q + 1;
                                println!("\nTag {} of {}...", q, tag_count);
                                println!("{}", i.path);

                                let mut classa : [u8; 4] = [0; 4];
                                BigEndian::write_u32(&mut classa, i.class_a);
                                println!("Class: {}", str::from_utf8(&classa).unwrap_or("????"));

                                if i.indexed {
                                    println!("Map: {:?}; Index: {:?}\n",i.resource_map,i.resource_map_index);
                                }
                                else {
                                    println!("Address: {:?}; Offset: {:?}",i.data_address,i.data_offset);
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
