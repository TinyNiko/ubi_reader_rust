use crate::debugs::log;
use std::io::Seek;
use std::fs::File;
use std::io::SeekFrom;

pub struct UbiFile<'a> {
    pub path: &'a str,
    pub block_size: i32,
    pub start_offset: u32,
    pub end_offset: i32,
    pub file: File
}

impl UbiFile<'_>{
    pub fn new(path: &str, block_size: i32, start_offset: u32, end_offset: i32) -> UbiFile {
        let name = "UBI_FILE";
        let is_valid = false;
        log("UBIFILE open path", path);
        let mut file = File::open(&path).expect("unable to open");
        let file_size = file.seek(SeekFrom::End(0)).unwrap() + 1;
        // log("File Size ", file_size.to_str());
        println!("file size {}", file_size);
        UbiFile {
            file: file,
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
    
    pub fn seek(& mut self, offset: u64){
        self.file.seek(SeekFrom::Start(offset)).unwrap();
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