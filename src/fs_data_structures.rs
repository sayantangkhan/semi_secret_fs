use fuse::{FileAttr, FileType};
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};

type Inode = u64;

pub struct DirEntry {
    inode: Inode,
    filetype: FileType,
    filename: OsString,
}

#[derive(Debug)]
pub struct InodeTable {
    inodes: HashMap<Inode, FileAttr>,
}

impl InodeTable {
    pub fn new() -> InodeTable {
        let inodes = HashMap::new();
        InodeTable { inodes }
    }

    pub fn get_inode(&self, inode: &Inode) -> Option<&FileAttr> {
        self.inodes.get(inode)
    }
}

pub struct FileDataTable {
    file_data: HashMap<Inode, Vec<u8>>,
}

pub struct DirEntryTable {
    dir_entries: HashMap<Inode, Vec<DirEntry>>,
}

impl DirEntryTable {
    pub fn lookup(&self, inode: Inode, filename: &OsStr) -> Option<FileAttr> {
        todo!()
    }
}
