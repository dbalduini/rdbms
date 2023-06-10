use std::fs::{remove_file, File, OpenOptions};
use std::io::{Error, Result, Seek, SeekFrom, Write};
use std::os::windows::prelude::FileExt;
use std::path::Path;

pub const PAGE_SIZE: usize = 4096;

pub struct DiskManager {
    db_file: String,
    db_io: File,
}

impl DiskManager {
    pub fn new(db_file: String) -> Self {
        let mut opt = File::options();

        opt.read(true).write(true).create(true);

        let db_io = match opt.open(&db_file) {
            Ok(fd) => fd,
            Err(e) => panic!("Failed to open database {} {:?}", db_file, e),
        };

        let dm = DiskManager { db_file, db_io };

        dm
    }

    pub fn write_page(&mut self, page_id: u64, page: &[u8]) -> Result<usize> {
        let offset: u64 = page_id * (PAGE_SIZE as u64);
        let result = self.db_io.seek_write(page, offset);
        if result.is_ok() {
            self.db_io.flush().expect("failed to flush");
        }
        result
    }

    pub fn read_page(&mut self, page_id: u64, buf: &mut [u8]) -> Result<usize> {
        let offset: u64 = page_id * (PAGE_SIZE as u64);
        self.db_io.seek_read(buf, offset)
    }

    pub fn drop_database(&self) -> Result<()> {
        remove_file(Path::new(&self.db_file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DB: &str = "test.db";

    fn setup() {
        remove_file(Path::new(DB)).ok();
    }

    fn teardown() {
        remove_file(Path::new(DB)).ok();
    }

    #[test]
    fn read_write_page() -> Result<()> {
        setup();
        let mut dm = DiskManager::new(String::from(DB));
        let mut data = [0; PAGE_SIZE];
        let mut buf = [0; PAGE_SIZE];
        let n = "Hello World".as_bytes();

        data[..n.len()].clone_from_slice(&n);

        // tolerate empty read
        dm.read_page(0, &mut buf)?;

        dm.write_page(0, &data)?;
        dm.read_page(0, &mut buf)?;
        assert_eq!(data, buf);

        // reset the buf
        buf.fill(0);

        dm.write_page(5, &data)?;
        dm.read_page(5, &mut buf)?;
        assert_eq!(data, buf);

        teardown();
        Ok(())
    }
}
