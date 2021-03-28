use crate::ubi::UbiBase;
use std::vec::Vec;
use std::collections::HashMap;


pub struct Description {
    file_offset: isize,
    peb_num: isize,
    leb_num: isize,
    size: isize,
    vid_hdr: i32,
    is_interal_vol: bool,
    vtbl_secs: Vec<i32>

}

impl Description {
    fn new() -> Description{
        Description{
            file_offset: 0,
            peb_num: 0,
            leb_num: 0,
            size: 0,
            vid_hdr: 0,
            is_interal_vol: false,
            vtbl_secs: vec![1,2,3]
        }
    }
    fn to_str() {

    }

    fn display() {

    }
}


pub fn get_blocks_in_list() {

}

pub fn extract_blocks(ubi: &mut UbiBase) -> HashMap<i32, i32> {
    let mut out = HashMap::new();
    ubi.file.seek(ubi.file.start_offset as u64);
    let ped_count = 0;
    let cur_offset = 0;
    let bad_blocks = Vec::<i32>::new();
    println!("{}", ubi.file.start_offset);
    println!("{}", ubi.file.end_offset);
    println!("{}", ubi.file.block_size);

    for  index in (ubi.file.start_offset..ubi.file.end_offset as u32).step_by(ubi.file.block_size as usize) {

    }

    out
}
