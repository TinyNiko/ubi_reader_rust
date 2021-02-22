pub static mut OUTPUT_DIR: &str = "ubifs-root";
pub static mut ERROR_ACTION: &str = "exit";                     // if 'exit' on any error exit program.
pub static mut FATAL_TRACEBACK: bool = false;                // Print traceback on fatal errors.

pub static mut IGNORE_BLOCK_HEADER_ERRORS: bool = false;      // Ignore block errors.
pub static mut WARN_ONLY_BLOCK_READ_ERRORS: bool = false;     // Warning instead of Fatal error.

pub static mut LOGGING_ON: bool = false;                      // Print debug info on.
pub static mut LOGGING_ON_VERBOSE: bool = false;              // Print verbose debug info on.pub 

pub static mut USE_DUMMY_SOCKET_FILE: bool = false;           // Create regular file place holder for sockets.
pub static mut USE_DUMMY_DEVICES: bool = false;               // Create regular file place holder for devices.

pub static mut UBOOT_FIX: bool = false;