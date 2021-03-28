extern crate clap;
use clap::{Arg, App};
use std::time::Instant;
use std::path::Path;
use std::process;
use corelib::settings;
use corelib::utils;
use corelib::ubi_io::UbiFile;
use corelib::ubifs::ubifsx::UbiFs;
use corelib::ubi::defines::{UBI_EC_HDR_MAGIC};
use corelib::ubifs::defines::{UBIFS_NODE_MAGIC};
use corelib::ubifs::list::{copy_file, list_files};
use std::fs;
use std::path::PathBuf;

fn main() {
    let start = Instant::now();
    let description = "List and Extract files of a UBI or UBIFS image.";
    let matches = App::new(description)
                          .arg(Arg::with_name("log")
                               .short("l")
                               .long("log")
                               .help("Print extraction information to screen."))
                        .arg(Arg::with_name("verbose_log")
                               .short("v")
                               .long("verbose-log")
                               .help("Prints nearly everything about anything to screen."))
                        .arg(Arg::with_name("peb_size")
                               .short("p")
                               .long("peb-size")
                               .help("Specify PEB size. (UBI Only)"))
                        .arg(Arg::with_name("leb_size")
                               .short("e")
                               .long("leb-size")
                               .help("Specify LEB size. (UBIFS Only)"))
                        .arg(Arg::with_name("start_offset")
                               .short("s")
                               .long("start-offset")
                               .help("Specify offset of UBI/UBIFS data in file. (default: 0)"))
                        .arg(Arg::with_name("end_offset")
                               .short("n")
                               .long("end-offset")
                               .takes_value(true)
                               .help("Specify end offset of UBI/UBIFS data in file."))
                        .arg(Arg::with_name("guess_offset")
                               .short("g")
                               .long("guess-offset")
                               .help("Specify offset to start guessing where UBI data is in file. (default: 0)"))
                        .arg(Arg::with_name("warn_only_block_read_errors")
                               .short("w")
                               .long("warn-only-block-read-errors")
                               .help("Attempts to continue extracting files even with bad block reads. Some data will be missing or corrupted! (default: False)"))
                        .arg(Arg::with_name("ignore_block_header_errors")
                               .short("i")
                               .long("ignore-block-header-errors")
                               .help("Forces unused and error containing blocks to be included and also displayed with log/verbose. (default: False)"))
                        .arg(Arg::with_name("uboot_fix")
                               .short("f")
                               .long("u-boot-fix")
                               .help("Assume blocks with image_seq 0 are because of older U-boot implementations and include them. (default: False)"))
                        .arg(Arg::with_name("listpath")
                               .short("P")
                               .long("path")
                               .help("Path to list."))
                        .arg(Arg::with_name("copyfile")
                               .short("C")
                               .long("copy")
                               .help("File to Copy."))
                        .arg(Arg::with_name("copyfiledest")
                               .short("D")
                               .long("copy-dest")
                               .help("Copy Destination."))
                        .arg(Arg::with_name("filepath")
                               .required(true)
                               .help("UBI/UBIFS image file."))
                        .get_matches();

    let file_path = matches.value_of("filepath").unwrap();
    // let srcdir = PathBuf::from(file_path);
    // println!("{:?}", fs::canonicalize(&srcdir).unwrap());
    // let path = Path::new(file_path);
    // let _display = path.canonicalize();
    // println!("path is {}", _display);
    unsafe {
        settings::LOGGING_ON = matches.is_present("log");
        settings::LOGGING_ON_VERBOSE = matches.is_present("verbose_log");

        settings::WARN_ONLY_BLOCK_READ_ERRORS = matches.is_present("warn_only_block_read_errors");
    
        settings::IGNORE_BLOCK_HEADER_ERRORS = matches.is_present("ignore_block_header_errors");
    
        settings::UBOOT_FIX = matches.is_present("uboot_fix");

        // println!("what iiiii {}", settings::LOGGING_ON);
    }
    let mut start_offset = 0;
    let mut end_offset = 0;
    if matches.is_present("start_offset") {
        start_offset = matches.value_of("end_offset").unwrap().parse().unwrap();
    }else if matches.is_present("guess_offset") {
        let guess_offset:i32 = matches.value_of("guess_offset").unwrap().parse().unwrap();
        start_offset = utils::guess_start_offset(file_path, Some(guess_offset));
    }else {
        start_offset = utils::guess_start_offset(file_path, None);
    }
    println!("start off is {}", start_offset);
    if matches.is_present("end_offset") {
        end_offset = matches.value_of("end_offset").unwrap().parse().unwrap();
        println!("{}", end_offset);
    };

    let filetype = match utils::guess_filetype(file_path, Some(start_offset)){
           Ok(ok) => ok,
           Err(err) => panic!("ggggg file type error")
    };
    println!("{:?}", filetype);
    let mut block_size = 0;
    if matches.is_present("block_size") {
       block_size = matches.value_of("block_size").unwrap().parse().unwrap();
    }else {
        if filetype == UBI_EC_HDR_MAGIC{
            block_size = match utils::guess_peb_size(file_path){
                Ok(ok) => ok,
                Err(err) => panic!("error")
            };
        }else if filetype == UBIFS_NODE_MAGIC {
            block_size = match utils::guess_leb_size(file_path) {
                Ok(ok) => ok,
                Err(err) => panic!("error")
            };
        }
        if block_size == 0 {
            panic!("block size error");
        }
    }
    println!("{}", block_size);

    let ufile_obj = UbiFile::new(file_path, block_size as i32, start_offset, end_offset);
    if filetype == UBI_EC_HDR_MAGIC {
       println!("good guy");
    }else if filetype == UBIFS_NODE_MAGIC {
       let mut ubifs_obj = UbiFs::new(ufile_obj);
       if matches.is_present("listpath"){
            let listpath = matches.value_of("listpath").unwrap();
            ubifs_obj = list_files(ubifs_obj, listpath);
       }
       if matches.is_present("copyfile") && matches.is_present("copyfiledest"){
            let file_path = matches.value_of("copyfile").unwrap();
            let dest_file_path = matches.value_of("copyfiledest").unwrap();
            copy_file(ubifs_obj, file_path, dest_file_path);
       }
    }else{
        println!("Something went wrong to get here.");
    }
    // let log = matches.value_of("log");
    // println!("{}", log);ñ
    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("log").unwrap_or("default.conf");
    // println!("Value for config: {}", config);
    // println!("Hello, world!");
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
