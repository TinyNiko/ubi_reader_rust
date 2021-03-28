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
use corelib::debugs::{log, error};
use corelib::ubi::Ubi;
use std::fs;
use std::path::PathBuf;

fn create_output_dir(outpath: &str){
    println!("{:?}", outpath);
    let path = Path::new(outpath);
    if path.exists() {
        let mut entry = match PathBuf::from(outpath).read_dir() {
            Ok(e) => e,
            Err(_) => panic!("23333"),
        };
        if !entry.next().is_none(){
            panic!("dir not emptry");
        }
    }else{
        match fs::create_dir(outpath) {
            Err(why) => panic!("! {:?}", why.kind()),
            Ok(_) => {},
        }
    }
}

fn main() {
    let start = Instant::now();
    let description = "Extract contents of a UBI or UBIFS image.";
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
                        .arg(Arg::with_name("keep")
                               .short("k")
                               .long("keep-permissions")
                               .help("Maintain file permissions, requires running as root. (default: False)"))
                        .arg(Arg::with_name("outpath")
                               .short("o")
                               .long("output-dir")
                               .takes_value(true)
                               .help("Specify output directory path."))
                        .arg(Arg::with_name("filepath")
                               .required(true)
                               .help("UBI/UBIFS image file."))
                        .get_matches();

    let file_path = matches.value_of("filepath").unwrap();
    
    unsafe {
        settings::LOGGING_ON = matches.is_present("log");
        settings::LOGGING_ON_VERBOSE = matches.is_present("verbose_log");

        settings::WARN_ONLY_BLOCK_READ_ERRORS = matches.is_present("warn_only_block_read_errors");
    
        settings::IGNORE_BLOCK_HEADER_ERRORS = matches.is_present("ignore_block_header_errors");
    
        settings::UBOOT_FIX = matches.is_present("uboot_fix");

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
    println!("{}", start_offset);
    if matches.is_present("end_offset") {
        end_offset = matches.value_of("end_offset").unwrap().parse().unwrap();
        println!("{}", end_offset);
    }

    let filetype = utils::guess_filetype(file_path, Some(start_offset)).unwrap();
    println!("{:?}", filetype);
    let block_size = 0;
    if matches.is_present("block_size") {
        
    }else {
        // if filetype == {

        // }else if filetype ==  {

        // }
        // if block_size == 0 {
        //     println!("error");
        // }
    }
    println!("what the fuck");
    let  mut outpath: &str = "";
    if matches.is_present("outpath") {
        outpath = matches.value_of("outpath").unwrap();
        println!("{}", outpath); 

        // unsafe{
        //     settings::OUTPUT_DIR = outpath;
        // }
    }
    let mut perms: &str;
    if matches.is_present("keep"){
        perms = matches.value_of("keep").unwrap_or("false");
    }

    println!("{:?}", fs::canonicalize(&outpath));
    let ufile_obj = UbiFile::new(file_path, block_size, start_offset, end_offset);
    if filetype == UBI_EC_HDR_MAGIC {
        println!("good guy");
       let ubi_obj = Ubi::new(ufile_obj);
    //    for image in ubi_obj.images{
    //        img_outpath = 
    //        println!("{}", img_outpath);
    //        for volume in image.volumes{
    //            let blocks = image.volumes[volume].get_blocks(ubi_obj.blocks)

    //            vol_outpath =  ;
    //            println!("{}", vol_outpath);
    //            create_output_dir(vol_outpath);
    //            if vol_blocks.size() == 0 {
    //                continue;
    //            }
    //            let lebv_file = leb_virtual_file::new(ubi_obj, vol_blocks);
    //            ubifs_obj = ubifs::new(lebv_file);
    //            println!("Extracting files to: {}", vol_outpath);
    //            extract_files(ubifs_obj, vol_outpath, perms);
    //        }
    //    }
       
    }else if filetype == UBIFS_NODE_MAGIC {
        let mut ubifs_obj = UbiFs::new(ufile_obj);
        create_output_dir(outpath);
        println!("Extracting files to: {}", &outpath);
        // extract_files(ubifs_obj, outpath, perms);
    }else{
        println!("Something went wrong to get here.");
    }

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

}
