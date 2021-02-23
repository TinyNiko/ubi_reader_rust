use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::SeekFrom;
// mod debugs;
// use crate::ubi::defines::{UBI_EC_HDR_MAGIC, FILE_CHUNK_SZ};
// use corelib::ubifs::defines::{UBIFS_NODE_MAGIC, UBIFS_SB_NODE_SZ, UBIFS_SB_NODE, UBIFS_COMMON_HDR_SZ};
// use corelib::ubifs::nodes;
pub fn guess_start_offset(path: &str, guess_offset: Option<i32>) -> i32{
    println!("hello world");
    return 0;
}

// fn guess_start_offset(path: str, guess_offset=0) {
//     println!("hello world");
// }

pub fn guess_filetype(path: &str, start_offset: Option<i32>) -> Result<[u8;4], &str> {
    // log(guess_filetype, "Looking for file type at {}" ,start_offset)
    let path = Path::new(path);
    let display = path.display();
    let mut buffer = [0; 4];
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    println!("'start '{}", start_offset.unwrap());
    file.seek(SeekFrom::Start(start_offset.unwrap_or(0) as u64));
    file.read(&mut buffer);
    println!("{:?}", buffer);
    if buffer == [85, 66, 73, 35]  {
        println!("good");
        Ok(buffer)
        // log(guess_filetype, "File looks like a UBI image.");
    }else if buffer == [12, 1, 3 ,4] {

        // log(guess_filetype, "File looks like a UBIFS image.");
        Ok(buffer)
    }else {
        // error(guess_filetype, "Fatal", "Could not determine file type.");
        Err("file type is unknow")
    }
}

pub fn guess_leb_size() {

}

pub fn guess_ped_size() {

}

