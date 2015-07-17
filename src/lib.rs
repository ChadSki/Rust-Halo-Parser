mod halo_tag;
pub use halo_tag::HaloTag;

extern crate byteorder;
use byteorder::{LittleEndian, ByteOrder};

use std::ffi::CStr;
use std::str;

pub struct HaloMap {
    pub name : String,
    pub build : String,
    pub size : u32,
    pub scenario_tag : usize,
    pub tags : Vec<HaloTag>
}

trait GetString {
    unsafe fn get_string(&self) -> Option<String>;
}

impl GetString for [u8] {
    unsafe fn get_string(&self) -> Option<String> {
        return match str::from_utf8(CStr::from_ptr(self.as_ptr() as *const i8).to_bytes() as &[u8]) {
            Err(_) => None,
            Ok(k) => Some(k.to_string())
        }
    }
}


impl HaloMap {
    fn new() -> HaloMap {
        HaloMap { name: String::new(), build: String::new(), size: 0, scenario_tag: 0, tags: Vec::new() }
    }

    pub fn from_buffer(data: &[u8]) -> Option<HaloMap> {
        let mut map = HaloMap::new();
        if data.len() < 2048 {
            return None
        }
        let map_size = LittleEndian::read_u32(&data[8..]) as usize;
        if data.len() != map_size {
            return None
        }
        if (LittleEndian::read_u32(&data[0..]) != 1751474532) || (LittleEndian::read_u32(&data[0x7FC..]) != 1718579060) {
            return None
        }

        unsafe {
            map.name = match (&data[0x20..0x40]).get_string() {
                Some(k) => k,
                None => String::new()
            };
            map.build = match (&data[0x40..0x60]).get_string() {
                Some(k) => k,
                None => String::new()
            };
        }

        let index_offset = LittleEndian::read_u32(&data[0x10..]) as usize;
        if index_offset + 0x28 > map_size {
            return None
        }

        let meta_size : usize = LittleEndian::read_u32(&data[0x14..]) as usize;
        if index_offset + meta_size > map_size {
            return None
        }
        let magic : usize = 0x40440000 - index_offset;

        let tag_array = LittleEndian::read_u32(&data[index_offset..]) as usize - magic;
        map.scenario_tag = LittleEndian::read_u16(&data[index_offset + 0x4..]) as usize;
        let tag_count = LittleEndian::read_u32(&data[index_offset + 0xC..]) as usize;

        if (tag_array + 0x20 * tag_count) > map_size {
            return None
        }

        let meta_end = index_offset + meta_size;

        for i in 0..tag_count {
            let tag_location = tag_array + 0x20 * i;
            let mut tag = HaloTag {
                path : String::new(),
                class_a : 0xFFFFFFFF,
                class_b : 0xFFFFFFFF,
                class_c : 0xFFFFFFFF,
                identity : 0xFFFFFFFF,
                indexed : false,
                data_offset : None,
                data_address : None,
                resource_map : None,
                resource_map_index : None,
            };
            tag.class_a = LittleEndian::read_u32(&data[tag_location..]);
            tag.class_b = LittleEndian::read_u32(&data[tag_location + 4..]);
            tag.class_c = LittleEndian::read_u32(&data[tag_location + 8..]);
            tag.indexed = LittleEndian::read_u32(&data[tag_location + 0x18..]) == 1;
            let tag_data = LittleEndian::read_u32(&data[tag_location + 0x14..]);
            if tag.indexed {
                tag.resource_map_index = Some(tag_data);
                tag.resource_map = match tag.class_a {
                    1651078253 => Some("bitmaps".to_string()),
                    1936614433 => Some("sounds".to_string()),
                    _ => Some("loc".to_string())
                }
            }
            else if tag.class_a == 1935831920 { //sbsp
                let scenario_tag_location = tag_array + 0x20 * map.scenario_tag;
                if scenario_tag_location < meta_end {
                    let scenario_tag_data = LittleEndian::read_u32(&data[scenario_tag_location + 0x14..]) as usize - magic;
                    if scenario_tag_data + 0x5AC < meta_end {
                        let sbsp_count = LittleEndian::read_u32(&data[scenario_tag_data + 0x5A4..]) as usize;
                        let sbsp_location = LittleEndian::read_u32(&data[scenario_tag_data + 0x5A8..]) as usize - magic;
                        if sbsp_location + sbsp_count * 32 < meta_end {
                            for k in 0..sbsp_count {
                                let sbsp_loc = sbsp_location + k * 32;
                                if LittleEndian::read_u16(&data[sbsp_loc + 0x1C..]) as usize == i {
                                    tag.data_address = Some(LittleEndian::read_u32(&data[sbsp_loc + 0x8..]));
                                    tag.data_offset = Some(LittleEndian::read_u32(&data[sbsp_loc..]) as usize);
                                }
                            }
                        }
                    }
                }
            }
            else {
                tag.data_address = Some(tag_data);
                tag.data_offset = Some(tag_data as usize - magic);
            }
            let tag_name_offset = LittleEndian::read_u32(&data[tag_location + 0x10..]) as usize - magic;
            if tag_name_offset < meta_end {
                unsafe {
                    tag.path = match (&data[tag_name_offset..meta_end]).get_string() {
                        None => String::new(),
                        Some(n) => n
                    }
                }
            }
            map.tags.push(tag);
        }

        return Some(map)
    }
}
