use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::table::Table;

struct WssfReader {
    disk_version: File,
}

impl WssfReader {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(Self {
            disk_version: file,
        })
    }
}

impl Drop for WssfReader {
    fn drop(&mut self) {
        
        if self.disk_version.sync_all().is_err() {
            println!("failed to sync on drop");
        }
    }
}


struct WssfHeader {
    header_size: u32,
    tabels: Vec<Table>,
}

impl WssfHeader {
    fn write_header(&self, file: &mut File) -> Result<(), Error> {
        let size_bytes = self.header_size.to_le_bytes();
        let table_bytes = self
            .tabels
            .iter()
            .map(|t| t.extract_for_header())
            .flatten()
            .collect::<Vec<u8>>();


        file.seek(SeekFrom::Start(0))?;
        file.write(&size_bytes)?;
        file.write(&table_bytes)?;

        Ok(())
    }

    fn read_header(file: &mut File) -> Result<Self, Error> {
        let mut size_buf = [0_u8; 4];
        file.read(&mut size_buf)?;
        let size = u32::from_le_bytes(size_buf);
        let mut tabels_buf = Vec::with_capacity(size as usize);
        file.read(&mut tabels_buf)?;
        let tabels = Table::from_header_info(tabels_buf);

        todo!()
    }
}
