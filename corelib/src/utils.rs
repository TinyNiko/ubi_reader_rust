use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::SeekFrom;
extern crate regex;
use regex::bytes::Regex;
// mod debugs;
use crate::debugs::{log, error};
use crate::ubi::defines::{UBI_EC_HDR_MAGIC, FILE_CHUNK_SZ};
use crate::ubifs::defines::{UBIFS_NODE_MAGIC, UBIFS_SB_NODE_SZ, UBIFS_SB_NODE, UBIFS_COMMON_HDR_SZ};
// use corelib::ubifs::nodes;


fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

pub fn guess_start_offset(path: &str, guess_offset: Option<i32>) -> u32{
    println!("hello world");
    let mut file_offset_value: u32 = match guess_offset{
        None => 0,
        Some(val) => val as u32
    };

    let mut file = match File::open(&path){
        Err(why) => panic!("couldn't open :{}", why),
        Ok(file) => file
    };
    let file_size = file.seek(SeekFrom::End(0)).unwrap() + 1;
    // let file_size = file.tell() + 1;
    file.seek(SeekFrom::Start(file_offset_value as u64));
    println!("file size {}", file_size);
    let mut find_offset = false;
    for _ in (0..file_size).step_by(FILE_CHUNK_SZ) {
        println!("wwwwww");
        let mut buf =[0; FILE_CHUNK_SZ];
        file.read(&mut buf);
        let mut ubi_loc: isize = match find_subsequence(&buf, &UBI_EC_HDR_MAGIC) {
            None => -1,
            Some(value) => value as isize
        };
        let mut ubifs_loc: isize = match find_subsequence(&buf, &UBIFS_NODE_MAGIC){
            None => -1,
            Some(value) => value as isize
        };

        if ubifs_loc == -1 && ubi_loc == -1{
            file_offset_value += FILE_CHUNK_SZ as u32 ;
            continue;
        }else{
            if ubi_loc == -1{
                ubi_loc = file_size as isize + 1
            }else if ubifs_loc == -1{
                ubifs_loc = file_size as isize + 1
            }
            if ubi_loc < ubifs_loc{
                println!("good");
                log("guess_start_offset", "Found UBI magic number at ");
                return (ubi_loc as u32 + (file_offset_value as u32)) as u32;
            }
            
        }
    }

    if !find_offset {
        error("2333", "Fatal", "Could not determine start_offset");
    }
    file_offset_value
}

// fn guess_start_offset(path: str, guess_offset=0) {
//     println!("hello world");
// }

pub fn guess_filetype(path: &str, start_offset: Option<u32>) -> Result<[u8;4], &str> {
    // log(guess_filetype, "Looking for file type at {}" ,start_offset)
    let path = Path::new(path);
    let display = path.display();
    let mut buffer = [0; 4];
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file
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

pub fn guess_leb_size(path: &str) -> Result<u32, &str>{
    println!("guess leb size");
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file
    };
    let file_size = file.seek(SeekFrom::End(0)).unwrap() + 1;
    file.seek(SeekFrom::Start(0));
    let mut block_size = 0;
    for i in (0..file_size).step_by(FILE_CHUNK_SZ){
        let mut buf =[0; FILE_CHUNK_SZ];
        file.read(&mut buf);
        let re = Regex::new(r"\x31\x18\x10\x06").unwrap();
        let re_matches = re.find_iter(&buf);
        for mat in re_matches {
            let start = mat.start();
            println!("{:?}", start);
        }
        // for m in re_match {
        // //     let start = m.start()
        // }
    }
    println!("goood");
    Ok(block_size)
}

pub fn guess_peb_size(path: &str) -> Result<u32, &str>{
    println!("guess ped size");
    let mut file_offset: usize = 0;
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}:", why),
        Ok(file) => file
    };
    let file_size = file.seek(SeekFrom::End(0)).unwrap() + 1;
    file.seek(SeekFrom::Start(0));
    let mut block_size = 0;
    // let offsets = vec![];
    // for i in (0..file_size).step_by(FILE_CHUNK_SZ){
    //     let mut buf =[0; FILE_CHUNK_SZ];
    //     file.read(&mut buf);
    //     let re = Regex::new(r"UBI#").unwrap();
    //     let re_matches = re.find_iter(&buf);
    //     for mat in re_matches {
    //         let start = mat.start();
    //         let mut index = 0;
    //         if file_offset == 0 {
    //             file_offset = start;
    //             index = start;
    //         }else{
    //             index = start + file_offset
    //         }
    //         // offsets.append(index);
    //     }
    //     file_offset += FILE_CHUNK_SZ;
    // }
    // let occurances = {};
    // let offsets_size = offsets.size();
    // for i in 0..offsets_size {
    //     let mut diff = 0;
    //     try{

    //     }catch{

    //     }
    //     occurances[diff] += 1;
    // }
    // let most_frequent = 0 
    // for offset in occurances{
    //     if occurances[offset] > most_frequent{
    //         most_frequent = occurances[offset]
    //         block_size = offset
    //     }
    // }
    
    println!("goooood");
    Ok(block_size)
}

