use fuse::FileAttr;
use std::collections::HashMap;

#[derive(Debug)]
pub struct InodeTable {
    inodes: HashMap<u64, FileAttr>,
}

impl InodeTable {
    pub fn new() -> InodeTable {
        let inodes = HashMap::new();
        InodeTable { inodes }
    }

    pub fn get_inode(&self, inode: &u64) -> Option<&FileAttr> {
        self.inodes.get(inode)
    }
}
