use fuse::{mount, Filesystem, ReplyAttr, Request};
use libc;
use std::env;

struct TestFilesystem {
    id_string: String,
}

impl Filesystem for TestFilesystem {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!(
            "Stat'ing a filesytem with the following id: {}",
            self.id_string
        );
        println!("getattr(ino={})", ino);
        reply.error(libc::ENOSYS);
    }
}

pub fn mount_test_filesystem() {
    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
            return;
        }
    };

    let filesystem = TestFilesystem {
        id_string: String::from("None"),
    };

    mount(filesystem, &mountpoint, &[]);
}
