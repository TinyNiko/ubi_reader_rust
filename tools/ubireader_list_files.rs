// use ubireader::setting;
// use ubireader::ubi;
// use ubireader::ubi::defines;
// use ubireader::ubifs;
// use ubireader::ubifs::list;
// use ubireader::ubifs::defines;
// use ubireader::ubi_io;
// use ubireader::debug;
// use ubireader::utils;
use std::env;
use std::time::{Duration, Instant};
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main(){
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let description = "List and Extract files of a UBI or UBIFS image.";
    let matches = App::new(description)
                          .version("1.0")
                          .arg(Arg::with_name("log")
                               .short("l")
                               .long("log")
                               .help("Print extraction information to screen."))
                          .get_matches();

    println!("hello");
    // println!("{}", matches);
    let log = matches.value_of("config").unwrap_or("default.conf");
    println!("{}",log);
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    // if len(sys.argv) == 1:
    // parser.print_help()
    // sys.exit(1)

// args = parser.parse_args()

// settings.logging_on = args.log

// settings.logging_on_verbose = args.verbose

// settings.warn_only_block_read_errors = args.warn_only_block_read_errors

// settings.ignore_block_header_errors = args.ignore_block_header_errors

// settings.uboot_fix = args.uboot_fix

// if args.filepath:
//     path = args.filepath
//     if not os.path.exists(path):
//         parser.error("File path doesn't exist.")

// if args.start_offset:
//     start_offset = args.start_offset
// elif args.guess_offset:
//     start_offset = guess_start_offset(path, args.guess_offset)
// else:
//     start_offset = guess_start_offset(path)

// if args.end_offset:
//     end_offset = args.end_offset
// else:
//     end_offset = None

// filetype = guess_filetype(path, start_offset)
// if not filetype:
//     parser.error('Could not determine file type.')

// if args.block_size:
//     block_size = args.block_size
// else:
//     if filetype == UBI_EC_HDR_MAGIC:
//         block_size = guess_peb_size(path)
//     elif filetype == UBIFS_NODE_MAGIC:
//         block_size = guess_leb_size(path)

//     if not block_size:
//         parser.error('Block size could not be determined.')

// # Create file object.
// ufile_obj = ubi_file(path, block_size, start_offset, end_offset)

// if filetype == UBI_EC_HDR_MAGIC:
//     # Create UBI object
//     ubi_obj = ubi(ufile_obj)

//     # Loop through found images in file.
//     for image in ubi_obj.images:

//         # Loop through volumes in each image.
//         for volume in image.volumes:

//             # Get blocks associated with this volume.
//             vol_blocks = image.volumes[volume].get_blocks(ubi_obj.blocks)

//             # Skip volume if empty.
//             if not len(vol_blocks):
//                 continue

//             # Create LEB backed virtual file with volume blocks.
//             # Necessary to prevent having to load entire UBI image
//             # into memory.
//             lebv_file = leb_virtual_file(ubi_obj, vol_blocks)

//             # Create UBIFS object.
//             ubifs_obj = ubifs(lebv_file)

//             if args.listpath:
//                 list_files(ubifs_obj, args.listpath)
//             if args.copyfile and args.copyfiledest:
//                 copy_file(ubifs_obj, args.copyfile, args.copyfiledest)

// elif filetype == UBIFS_NODE_MAGIC:
//     # Create UBIFS object
//     ubifs_obj = ubifs(ufile_obj)

//     if args.listpath:
//         list_files(ubifs_obj, args.listpath)
//     if args.copyfile and args.copyfiledest:
//         copy_file(ubifs_obj, args.copyfile, args.copyfiledest)

// else:
//     print('Something went wrong to get here.')
}