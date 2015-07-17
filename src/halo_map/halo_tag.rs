pub struct HaloTag {
    pub path : String,
    pub class_a : u32,
    pub class_b : u32,
    pub class_c : u32,
    pub identity : u32,
    pub indexed : bool,
    pub data_offset : Option<usize>,
    pub data_address : Option<u32>,
    pub resource_map : Option<String>,
    pub resource_map_index : Option<u32>,
}
