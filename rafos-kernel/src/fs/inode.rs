use super::File;
use crate::device::BLOCK_DEVICE;
use ubuf::UserBuffer;
use alloc::{
    sync::Arc,
    vec::Vec,
};
use spin::{Mutex, Lazy};
use bitflags::*;
use easy_fs::{EasyFileSystem, Inode};

pub struct OSInode {
    readable: bool,
    writable: bool,
    inner: Mutex<OSInodeInner>,
}

pub struct OSInodeInner {
    offset: usize,
    inode: Arc<Inode>,
}

impl OSInode {
    pub fn new(readable: bool, writable: bool, inode: Arc<Inode>) -> Self {
        Self {
            readable,
            writable,
            inner: Mutex::new(OSInodeInner { offset: 0, inode }),
        }
    }
    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.lock();
        let mut buffer = [0u8; 512];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = inner.inode.read_at(inner.offset, &mut buffer);
            if len == 0 {
                break;
            }
            inner.offset += len;
            v.extend_from_slice(&buffer[..len]);
        }
        v
    }
}

pub static ROOT_INODE: Lazy<Arc<Inode>> = Lazy::new(|| {
    let efs = EasyFileSystem::open(BLOCK_DEVICE.clone());
    Arc::new(EasyFileSystem::root_inode(&efs))
});

pub fn list_apps() {
    log::info!("/**** APPS ****");
    for app in ROOT_INODE.ls() {
        log::info!("{}", app);
    }
    log::info!("**************/")
}

bitflags! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 9;
        const TRUNC = 1 << 10;
    }
}

impl OpenFlags {
    /// Do not check validity for simplicity
    /// Return (readable, writable)
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}

pub fn open_file(name: &str, flags: OpenFlags) -> Option<Arc<OSInode>> {
    let (readable, writable) = flags.read_write();
    if flags.contains(OpenFlags::CREATE) {
        if let Some(inode) = ROOT_INODE.find(name) {
            // clear size
            inode.clear();
            Some(Arc::new(OSInode::new(readable, writable, inode)))
        } else {
            // create file
            ROOT_INODE
                .create(name)
                .map(|inode| Arc::new(OSInode::new(readable, writable, inode)))
        }
    } else {
        ROOT_INODE.find(name).map(|inode| {
            if flags.contains(OpenFlags::TRUNC) {
                inode.clear();
            }
            Arc::new(OSInode::new(readable, writable, inode))
        })
    }
}

impl File for OSInode {
    fn readable(&self) -> bool {
        self.readable
    }
    fn writable(&self) -> bool {
        self.writable
    }
    fn read(&self, mut buf: UserBuffer) -> Result<usize, isize> {
        let mut inner = self.inner.lock();
        let mut total_read_size = 0usize;
        for slice in &mut buf.inner {
            let read_size = inner.inode.read_at(inner.offset, *slice);
            if read_size == 0 {
                break;
            }
            inner.offset += read_size;
            total_read_size += read_size;
        }
        Ok(total_read_size)
    }
    fn write(&self, buf: UserBuffer) -> Result<usize, isize> {
        let mut inner = self.inner.lock();
        let mut total_write_size = 0usize;
        for slice in &buf.inner {
            let write_size = inner.inode.write_at(inner.offset, *slice);
            assert_eq!(write_size, slice.len());
            inner.offset += write_size;
            total_write_size += write_size;
        }
        Ok(total_write_size)
    }
    fn awrite(&self, buf: UserBuffer, pid: usize, key: usize) -> Result<usize, isize> {
        unimplemented!()
    }
    fn aread(&self, buf: UserBuffer, cid: usize, pid: usize, key: usize) -> Result<usize, isize> {
        unimplemented!()
    }
}