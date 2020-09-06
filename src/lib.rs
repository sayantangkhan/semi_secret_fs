mod inode_table;

use fuse::{mount, Filesystem, ReplyAttr, ReplyEntry, Request};
use inode_table::*;
use libc;
use libc::ENOENT;
use std::env;
use std::ffi::OsStr;

struct TestFilesystem {
    inode_table: InodeTable,
}

impl TestFilesystem {
    fn new() -> TestFilesystem {
        let inode_table = InodeTable::new();
        TestFilesystem { inode_table }
    }
}

impl Filesystem for TestFilesystem {
    fn lookup(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEntry) {
        match self.inode_table.get_inode(&parent) {
            Some(file_attr) => {
                let parent_dir = file_attr;
                // Insert code for content handling
            }
            None => {
                reply.error(ENOENT);
            }
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        reply.error(libc::ENOSYS);
    }
}

pub fn mount_test_filesystem() {
    let filesystem = TestFilesystem::new();

    // let mountpoint = match env::args().nth(1) {
    //     Some(path) => path,
    //     None => {
    //         println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
    //         return;
    //     }
    // };

    // let filesystem = TestFilesystem {
    //     id_string: String::from("TestFS"),
    // };

    // mount(filesystem, &mountpoint, &[]);
}
