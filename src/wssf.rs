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

#[derive(Debug, PartialEq, Eq)]
struct WssfHeader {
    header_size: u32,
    tables: Vec<Table>,
}

impl WssfHeader {
    fn write_header(&self, file: &mut File) -> Result<(), Error> {
        let size_bytes = self.header_size.to_le_bytes();
        let table_bytes = self
            .tables
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
        // when reading the header we need to go to the start
        file.seek(SeekFrom::Start(0))?;

        let mut size_buf = [0_u8; 4];
        file.read(&mut size_buf)?;
        let size = u32::from_le_bytes(size_buf);

        let mut tabels_buf = Vec::new();
        tabels_buf.resize((size-4) as usize, 0_u8);

        file.read(&mut tabels_buf)?;
        println!("{}", tabels_buf.len());

        let tables = Table::from_header_info(tabels_buf);

        Ok(Self {
            header_size: size,
            tables,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn open_test_file<P: AsRef<Path>>(path: P) -> Result<File, Error> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
    }

    #[test]
    fn write_and_read_empty_header() {
        let mut file = open_test_file("test data/empty.wssf").unwrap();
        let header = WssfHeader {
            header_size: 4,
            tables: Vec::new()
        };

        header.write_header(&mut file).unwrap();

        let read_header = WssfHeader::read_header(&mut file).unwrap();
        assert_eq!(header, read_header);
    }

    #[test]
    fn write_and_read_single_table() {
        let mut file = open_test_file("test data/single_table.wssf").unwrap();
        let table =  Table::new(10, 10, 32);

        let header = WssfHeader {
            header_size: 24,
            tables: vec![
                table
            ]
        };

        header.write_header(&mut file).unwrap();

        let read_header = WssfHeader::read_header(&mut file).unwrap();
        assert_eq!(header, read_header);
    }
}