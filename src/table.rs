
// probs need to point to schema or serialize schema somehow,,,,
pub struct Table {
    size_of_item: u32,
    start: usize,
    size: u64,
    schema: Schema,
}

impl Table {
    pub(crate) fn extract_for_header(&self) -> Vec<u8> {
        let mut data = Vec::new();

        let start_b = self.start.to_le_bytes();
        let size_b = self.size.to_le_bytes();
        let item_size_b = self.size_of_item.to_be_bytes();

        data.extend(start_b);
        data.extend(size_b);
        data.extend(item_size_b);

        data
    }

    pub(crate) fn from_header_info(data: Vec<u8>) -> Vec<Self> {
        let mut tabels = Vec::new();

        let mut idx = 0;
        while idx < data.len() {
            idx += 1;
        }

        tabels
    }
}

pub struct Field {
    size: u64,
    name: String,
}

pub struct Schema {
    fields: Vec<Field>,
    name: String,
}