pub mod defines;
pub mod block;
use std::collections::HashMap;
use crate::ubi_io::UbiFile;
use crate::ubi::block::extract_blocks;

pub struct UbiBase<'a> {
    name: &'a str,
    file: UbiFile<'a>,
    first_ped_num: usize,
    blocks: usize,
    block_count: usize,
    min_io_size: usize,
    leb_size: usize
}

impl UbiBase <'_>{
    fn new(ubi_file: UbiFile) -> UbiBase {
        let mut ubi_base = UbiBase {
            name: "UBI",
            file: ubi_file,
            first_ped_num: 0,
            blocks: 0,
            block_count: 0,
            min_io_size: 0,
            leb_size: 0
        };
        let blocks = extract_blocks(&mut ubi_base);
        println!("{}",blocks.len());
        let block_count = blocks.len();
        if block_count <= 0{
            // panic!("block count is error");
        }

        ubi_base

    }

    fn get_file(&self) {

    }

    fn get_block_count(&self) {

    }

    fn set_first_ped_num() {

    }

    fn get_first_peb_num() {

    }

    fn get_leb_size() {

    }

    fn get_peb_size() {

    }

    fn get_min_io_size() {

    }

    fn get_blocks() {

    }
}


pub struct Ubi <'a>{
    base: UbiBase <'a>
}

impl Ubi <'_>{
    pub fn new(ubi_file: UbiFile) -> Ubi {
        let ubi_base = UbiBase::new(ubi_file);
        println!("good in UBI now");
        Ubi{
            base: ubi_base
        }
    }

    fn get_images(&self) {

    }
    fn get_data_blocks_list(&self) {

    }
    fn get_layout_blocks_list() {

    }
    fn get_int_vol_blocks_list() {


    }

    fn get_unknown_blocks_list() {

    }
    
    fn display() {

    }
}
