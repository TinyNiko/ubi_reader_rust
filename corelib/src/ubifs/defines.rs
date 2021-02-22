//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Adapted in part from linux-source-3.2/fs/ubi/ubi-media.h
// for use in Python.
// Oct. 2013 by Jason Pruitt
//
// Original copyright notice.
// --------------------------
//
// This file is part of UBIFS.
//
// Copyright (C) 2006-2008 Nokia Corporation.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published by
// the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along with
// this program; if not, write to the Free Software Foundation, Inc., 51
// Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
//
// Authors: Artem Bityutskiy (Битюцкий Артём)
//          Adrian Hunter
//
// //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Constant defines

// Common Header.
pub const UBIFS_NODE_MAGIC: [u8;4] = [0x31,0x18,0x10,0x06]; // b"\x31\x18\x10\x06" // Set to LSB

// Initial CRC32 value.
const UBIFS_CRC32_INIT: u32 = 0xFFFFFFFF;

// Do not compress data smaller than this.
const UBIFS_MIN_COMPR_LEN: i32 = 128;

// If difference between compressed data length and compressed data
// length, is less than this, do not compress data.
const UBIFS_MIN_COMPRESS_DIFF: i32 = 64;

// Root inode number
const UBIFS_ROOT_INO: i32 = 1;

// Lowest inode number for regular inodes, non-internal inodes.
const UBIFS_FIRST_INO: i32 = 64;

// Max file name and extended attr length (muptple of 8 minus 1.
const UBIFS_MAX_NLEN: i32 = 255;

// Max number of data journal heads.
const UBIFS_MAX_JHEADS: i32 = 1;

// Max data node data length/amount attached to inode node.
const UBIFS_BLOCK_SIZE: i32 = 4096;
const UBIFS_BLOCK_SHIFT: i32 = 12;

// UBIFS padding byte pattern.
const UBIFS_PADDING_BYTE: u8 = 0xCE;

// Max key length
const UBIFS_MAX_KEY_LEN: i32 = 16;

// Key length of simple format.
const UBIFS_SK_LEN: i32 = 8;

// Min index tree fanout.
const UBIFS_MIN_FANOUT: i32 = 3;

// Max number of levels in UBIFS indexing B-tree.
const UBIFS_MAX_LEVELS: i32 = 512;

// Max amount of data attached to inode in bytes.
const UBIFS_MAX_INO_DATA: i32 = UBIFS_BLOCK_SIZE;

// LEB Properties Tree fanout (power of 2) and fanout.
const UBIFS_LPT_FANOUT: i32 = 4;
const UBIFS_LPT_FANOUT_SHIFT: i32 = 2;

// LEB Properties Tree bit field sizes.
const UBIFS_LPT_CRC_BITS: i32  = 16;
const UBIFS_LPT_CRC_BYTES: i32 = 2;
const UBIFS_LPT_TYPE_BITS: i32 = 4;

// LEB Properties Tree node types.
const UBIFS_LPT_PNODE: i32         = 0; // LPT leaf node (contains LEB Properties)
const UBIFS_LPT_NNODE: i32         = 1; // LPT internal node
const UBIFS_LPT_LTAB: i32          = 2; // LPT"s own lprops table
const UBIFS_LPT_LSAVE: i32         = 3; // LPT"s save table (big model only)
const UBIFS_LPT_NODE_CNT: i32      = 4; // count of LPT node types
const UBIFS_LPT_NOT_A_NODE: i32 = (1 << UBIFS_LPT_TYPE_BITS) - 1; // 4 bits of 1

// Inode types
const UBIFS_ITYPE_REG: i32  = 0; // Regular file
const UBIFS_ITYPE_DIR: i32  = 1; // Directory
const UBIFS_ITYPE_LNK: i32  = 2; // Soft link
const UBIFS_ITYPE_BLK: i32  = 3; // Block device node
const UBIFS_ITYPE_CHR: i32  = 4; // Char device node
const UBIFS_ITYPE_FIFO: i32 = 5; // FIFO
const UBIFS_ITYPE_SOCK: i32 = 6; // Socket
const UBIFS_ITYPES_CNT: i32 = 7; // Support file type count

// Supported key has functions
const UBIFS_KEY_HASH_R5: i32   = 0; // R5 hash
const UBIFS_KEY_HASH_TEST: i32 = 1; // Test hash, returns first 4 bytes of name
// const PRINT_UBIFS_KEY_HASH:  = ["r5", "test"];

// Supported key formats
const UBIFS_SIMPLE_KEY_FMT: i32 = 0;

// Simple key format uses 29 bits for storing UBIFS name and hash.
const UBIFS_S_KEY_BLOCK_BITS: i32 = 29;
const UBIFS_S_KEY_BLOCK_MASK: i32 = 0x1FFFFFFF;
const UBIFS_S_KEY_HASH_BITS: i32  = UBIFS_S_KEY_BLOCK_BITS;
const UBIFS_S_KEY_HASH_MASK: i32  = UBIFS_S_KEY_BLOCK_MASK;

// Key types
const UBIFS_INO_KEY: i32       = 0; // Inode node key
const UBIFS_DATA_KEY: i32      = 1; // Data node key
const UBIFS_DENT_KEY: i32      = 2; // Directory node key
const UBIFS_XENT_KEY: i32      = 3; // Extended attribute entry key
const UBIFS_KEY_TYPES_CNT: i32 = 4; // Supported key count

// Number of reserved LEBs for Superblock area
const UBIFS_SB_LEBS: i32 = 1;

// Number of reserved LEBs for master area
const UBIFS_MST_LEBS: i32 = 2;

// First LEB of the Superblock area
const UBIFS_SB_LNUM: i32 = 0;

// First LEB of the master area
const UBIFS_MST_LNUM: i32 = (UBIFS_SB_LNUM + UBIFS_SB_LEBS);

// First LEB of log area
const UBIFS_LOG_LNUM: i32 = (UBIFS_MST_LNUM + UBIFS_MST_LEBS);

// On-flash inode flags
const UBIFS_COMPR_FL: i32     = 1;  // Use compression for this inode
const UBIFS_SYNC_FL: i32      = 2; // Has to be synchronous I/O
const UBIFS_IMMUTABLE_FL: i32 = 4;  // Inode is immutable
const UBIFS_APPEND_FL: i32    = 8;  // Writes may only append data
const UBIFS_DIRSYNC_FL: i32   = 16; // I/O on this directory inode must be synchronous
const UBIFS_XATTR_FL: i32     = 32; // This inode is inode for extended attributes

// Inode flag bits used by UBIFS
const UBIFS_FL_MASK: i32 = 0x0000001F;

// Compression alogrithms.
const UBIFS_COMPR_NONE: i32        = 0; // No compression
const UBIFS_COMPR_LZO: i32         = 1; // LZO compression
const UBIFS_COMPR_ZLIB: i32        = 2; // ZLIB compression
const UBIFS_COMPR_TYPES_CNT: i32   = 3; // Count of supported compression types
// const PRINT_UBIFS_COMPR = ["none","lzo","zlib"];

// UBIFS node types
const UBIFS_INO_NODE: i32         = 0;  // Inode node
const UBIFS_DATA_NODE: i32         = 1;  // Data node
const UBIFS_DENT_NODE: i32         = 2;  // Directory entry node
const UBIFS_XENT_NODE: i32         = 3;  // Extended attribute node
const UBIFS_TRUN_NODE: i32         = 4;  // Truncation node
const UBIFS_PAD_NODE: i32          = 5;  // Padding node
const UBIFS_SB_NODE: i32           = 6;  // Superblock node
const UBIFS_MST_NODE: i32          = 7;  // Master node
const UBIFS_REF_NODE: i32          = 8;  // LEB reference node
const UBIFS_IDX_NODE: i32          = 9;  // Index node
const UBIFS_CS_NODE: i32           = 10; // Commit start node
const UBIFS_ORPH_NODE: i32         = 11; // Orphan node
const UBIFS_NODE_TYPES_CNT: i32    = 12; // Count of supported node types

// Master node flags
const UBIFS_MST_DIRTY: i32     = 1; // Rebooted uncleanly
const UBIFS_MST_NO_ORPHS: i32  = 2; // No orphans present
const UBIFS_MST_RCVRY: i32     = 4; // Written by recovery

// Node group type
const UBIFS_NO_NODE_GROUP: i32         = 0; // This node is not part of a group
const UBIFS_IN_NODE_GROUP: i32         = 1; // This node is part of a group
const UBIFS_LAST_OF_NODE_GROUP: i32    = 2; // This node is the last in a group

// Superblock flags
const UBIFS_FLG_BIGLPT: i32        = 2; // if "big" LPT model is used if set.
const UBIFS_FLG_SPACE_FIXUP: i32   = 4; // first-mount "fixup" of free space within


// Struct defines

// Common header node
// const UBIFS_COMMON_HDR_FORMAT = "<IIQIBB2s"
// const UBIFS_COMMON_HDR_FIELDS = ["magic",     // UBIFS node magic number.
//                            "crc",       // CRC32 checksum of header.
//                            "sqnum",     // Sequence number.
//                            "len",       // Full node length.
//                            "node_type", // Node type.
//                            "group_type",// Node group type.
//                            "padding"]   // Reserved for future, zeros.
const UBIFS_COMMON_HDR_SZ: i32 = 24;
                            // LEBs needed.
// Key offset in key nodes
// out of place because of ordering issues.
const UBIFS_KEY_OFFSET: i32 = UBIFS_COMMON_HDR_SZ;

// Device node descriptor
// const UBIFS_DEV_DESC_FORMAT = "<IQ"
// const UBIFS_DEV_DESC_FIELDS = ["new",  // New type device descriptor.
//                          "huge"] // huge type device descriptor.
const UBIFS_DEV_DESC_SZ: i32 = 12;

// Inode node
// const UBIFS_INO_NODE_FORMAT = "<%ssQQQQQIIIIIIIIIII4sIH26s" % (UBIFS_MAX_KEY_LEN)
// const UBIFS_INO_NODE_FIELDS = ["key",         // Node key
//                          "creat_sqnum", // Sequence number at time of creation.
//                          "size",        // Inode size in bytes (uncompressed).
//                          "atime_sec",   // Access time in seconds.
//                          "ctime_sec",   // Creation time seconds.
//                          "mtime_sec",   // Modification time in seconds.
//                          "atime_nsec",  // Access time in nanoseconds.
//                          "ctime_nsec",  // Creation time in nanoseconds.
//                          "mtime_nsec",  // Modification time in nanoseconds.
//                          "nlink",       // Number of hard links.
//                          "uid",         // Owner ID.
//                          "gid",         // Group ID.
//                          "mode",        // Access flags.
//                          "flags",       // Per-inode flags.
//                          "data_len",    // Inode data length.
//                          "xattr_cnt",   // Count of extended attr this inode has
//                          "xattr_size",  // Summarized size of all extended
//                                         // attributes in bytes.
//                          "padding1",    // Reserved for future, zeros.
//                          "xattr_names", // Sum of lengths of all extended.
//                                         // attribute names belonging to this
//                                         // inode.
//                          "compr_type",  // Compression type used for this inode.
//                          "padding2"]    // Reserved for future, zeros.
//                         // "data" No size
// const UBIFS_INO_NODE_SZ = struct.calcsize(UBIFS_INO_NODE_FORMAT)


// Directory entry node
// const UBIFS_DENT_NODE_FORMAT = "<%ssQBBH4s" % (UBIFS_MAX_KEY_LEN)
// const UBIFS_DENT_NODE_FIELDS = ["key",     // Node key.
//                           "inum",    // Target inode number.
//                           "padding1",// Reserved for future, zeros.
//                           "type",    // Type of target inode.
//                           "nlen",    // Name length.
//                           "padding2"]// Reserved for future, zeros.
//                         // "Name" No size
// const UBIFS_DENT_NODE_SZ = struct.calcsize(UBIFS_DENT_NODE_FORMAT)


// // Data node
// const UBIFS_DATA_NODE_FORMAT = "<%ssIH2s" % (UBIFS_MAX_KEY_LEN)
// const UBIFS_DATA_NODE_FIELDS = ["key",        // Node key.
//                           "size",       // Uncompressed data size.
//                           "compr_type", // Compression type UBIFS_COMPR_*
//                           "padding"]    // Reserved for future, zeros.
//                         // "data" No size
// const UBIFS_DATA_NODE_SZ = struct.calcsize(UBIFS_DATA_NODE_FORMAT)

// // Truncation node
// const UBIFS_TRUN_NODE_FORMAT = "<I12sQQ"
// const UBIFS_TRUN_NODE_FIELDS = ["inum",     // Truncated inode number.
//                           "padding",  // Reserved for future, zeros.
//                           "old_size", // size before truncation.
//                           "new_size"] // Size after truncation.
// const UBIFS_TRUN_NODE_SZ = struct.calcsize(UBIFS_TRUN_NODE_FORMAT)

// Padding node
// const UBIFS_PAD_NODE_FORMAT = "<I"
// const UBIFS_PAD_NODE_FIELDS = ["pad_len"] // Number of bytes after this inode unused.
// const UBIFS_PAD_NODE_SZ = struct.calcsize(UBIFS_PAD_NODE_FORMAT)

// // Superblock node
// const UBIFS_SB_NODE_FORMAT = "<2sBBIIIIIQIIIIIIIH2sIIQI16sI3968s"
// const UBIFS_SB_NODE_FIELDS = ["padding",          // Reserved for future, zeros.
//                         "key_hash",         // Type of hash func used in keys.
//                         "key_fmt",          // Format of the key.
//                         "flags",            // File system flags.
//                         "min_io_size",      // Min I/O unit size.
//                         "leb_size",         // LEB size in bytes.
//                         "leb_cnt",          // LEB count used by FS.
//                         "max_leb_cnt",      // Max count of LEBS used by FS.
//                         "max_bud_bytes",    // Max amount of data stored in buds.
//                         "log_lebs",         // Log size in LEBs.
//                         "lpt_lebs",         // Number of LEBS used for lprops
//                                             // table.
//                         "orph_lebs",        // Number of LEBS used for
//                                             // recording orphans.
//                         "jhead_cnt",        // Count of journal heads
//                         "fanout",           // Tree fanout, max number of links
//                                             // per indexing node.
//                         "lsave_cnt",        // Number of LEB numbers in LPT"s
//                                             // save table.
//                         "fmt_version",      // UBIFS on-flash format version.
//                         "default_compr",    // Default compression used.
//                         "padding1",         // Reserved for future, zeros.
//                         "rp_uid",           // Reserve pool UID
//                         "rp_gid",           // Reserve pool GID
//                         "rp_size",          // Reserve pool size in bytes
//                         "time_gran",        // Time granularity in nanoseconds.
//                         "uuid",             // UUID generated when the FS image
//                                             // was created.
//                         "ro_compat_version",// UBIFS R/O Compatibility version.
//                         "padding2"]         //Reserved for future, zeros
// const UBIFS_SB_NODE_SZ = struct.calcsize(UBIFS_SB_NODE_FORMAT)

// // Master node
// const UBIFS_MST_NODE_FORMAT = "<QQIIIIIIIIQQQQQQIIIIIIIIIIII344s"
// const UBIFS_MST_NODE_FIELDS = ["highest_inum",// Highest inode number in the
//                                         // committed index.
//                          "cmt_no",      // Commit Number.
//                          "flags",       // Various flags.
//                          "log_lnum",    // LEB num start of log.
//                          "root_lnum",   // LEB num of root indexing node.
//                          "root_offs",   // Offset within root_lnum
//                          "root_len",    // Root indexing node length.
//                          "gc_lnum",     // LEB reserved for garbage collection.
//                          "ihead_lnum",  // LEB num of index head.
//                          "ihead_offs",  // Offset of index head.
//                          "index_size",  // Size of index on flash.
//                          "total_free",  // Total free space in bytes.
//                          "total_dirty", // Total dirty space in bytes.
//                          "total_used",  // Total used space in bytes (data LEBs)
//                          "total_dead",  // Total dead space in bytes (data LEBs)
//                          "total_dark",  // Total dark space in bytes (data LEBs)
//                          "lpt_lnum",    // LEB num of LPT root nnode.
//                          "lpt_offs",    // Offset of LPT root nnode.
//                          "nhead_lnum",  // LEB num of LPT head.
//                          "nhead_offs",  // Offset of LPT head.
//                          "ltab_lnum",   // LEB num of LPT"s own lprop table.
//                          "ltab_offs",   // Offset of LPT"s own lprop table.
//                          "lsave_lnum",  // LEB num of LPT"s save table.
//                          "lsave_offs",  // Offset of LPT"s save table.
//                          "lscan_lnum",  // LEB num of last LPT scan.
//                          "empty_lebs",  // Number of empty LEBs.
//                          "idx_lebs",    // Number of indexing LEBs.
//                          "leb_cnt",     // Count of LEBs used by FS.
//                          "padding"]     // Reserved for future, zeros.
// const UBIFS_MST_NODE_SZ = struct.calcsize(UBIFS_MST_NODE_FORMAT)

// // LEB Reference node
// const UBIFS_REF_NODE_FORMAT = "<III28s"
// const UBIFS_REF_NODE_FIELDS = ["lnum",    // Referred LEB number.
//                          "offs",    // Start offset of referred LEB.
//                          "jhead",   // Journal head number.
//                          "padding"] // Reserved for future, zeros.
// const UBIFS_REF_NODE_SZ = struct.calcsize(UBIFS_REF_NODE_FORMAT)

// // key/reference/length branch
// const UBIFS_BRANCH_FORMAT = "<III%ss" % (UBIFS_SK_LEN)
// const UBIFS_BRANCH_FIELDS = ["lnum",  // LEB number of target node.
//                        "offs",  // Offset within lnum.
//                        "len",   // Target node length.
//                        "key"]   // Using UBIFS_SK_LEN as size.
// const UBIFS_BRANCH_SZ = struct.calcsize(UBIFS_BRANCH_FORMAT)

// Indexing node
// const UBIFS_IDX_NODE_FORMAT = "<HH"
// const UBIFS_IDX_NODE_FIELDS = ["child_cnt",   // Number of child index nodes.
//                          "level"]       // Tree level.
                        // branches, no size.
const UBIFS_IDX_NODE_SZ:i32 = 4;

// File chunk size for reads.
pub const FILE_CHUNK_SZ: i32 = 5 * 1024 * 1024;