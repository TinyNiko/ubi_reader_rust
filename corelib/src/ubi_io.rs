// use crate::settings;
// use crate::debugs;

pub struct UbiFile<'a> {
    path: &'a str,
    block_size: i32,
    start_offset: i32,
    end_offset: i32
}

impl UbiFile<'_>{
    pub fn new(path: &str, block_size: i32, start_offset: i32, end_offset: i32) -> UbiFile {
        UbiFile{
            path: path,
            block_size: block_size,
            start_offset: start_offset,
            end_offset: end_offset
        }
    }
    fn set_start(){

    }
    fn get_start(){

    }

    fn get_end(){

    }

    fn get_block_size(){

    }
    fn seek(){

    }
    fn read(){

    }
    fn tell(){

    }

    fn last_read_addr(){

    }
    fn reset(){

    }

    fn reader(){

    }
    fn read_block(){

    }

    fn read_block_data(){

    }
}



struct LebVirtualFile<'a> {
    name: &'a str,
    is_valid: bool,
    //ubi:,
    last_read_addr:i32,
    seek: i32,
    last_leb: i32,
    // last_buf: 
}

impl LebVirtualFile<'_> {
    pub fn new() -> LebVirtualFile<'static> {
        LebVirtualFile {
            name: "LebVirtualFile",
            is_valid: true,
            last_read_addr: 0,
            seek: 0,
            last_leb: 0
        }

    }
    pub fn read(){

    }
    pub fn reset() {

    }

    pub fn seek(){

    }

    pub fn tell() {

    }

    pub fn last_read_addr(){
    }

    pub fn reader(){

    }
}