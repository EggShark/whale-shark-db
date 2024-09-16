
// probs need to point to schema or serialize schema somehow,,,,
#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    size_of_item: u32,
    start: usize,
    size: u64,
    schema: Schema,
}

impl Table {
    pub(crate) fn new(size_of_item: u32, start: usize, size: u64) -> Self {
        Self {
            size_of_item,
            start,
            size,
            schema: Schema::default(),
        }
    }

    pub(crate) fn extract_for_header(&self) -> Vec<u8> {
        let mut data = Vec::new();

        let start_b = self.start.to_le_bytes();
        let size_b = self.size.to_le_bytes();
        let item_size_b = self.size_of_item.to_le_bytes();

        data.extend(start_b);
        data.extend(size_b);
        data.extend(item_size_b);

        data
    }

    pub(crate) fn from_header_info(data: Vec<u8>) -> Vec<Self> {
        data.chunks(20)
            .map(|window| {
                let start_b: [u8; 8] = window[0..8].try_into().unwrap();
                let start = usize::from_le_bytes(start_b);
    
                let size_b: [u8; 8] = window[8..16].try_into().unwrap();
                let size = u64::from_le_bytes(size_b);
    
                let item_size_b: [u8; 4] = window[16..20].try_into().unwrap();
                let item_size = u32::from_le_bytes(item_size_b);

                Ok(Self {
                    size_of_item: item_size,
                    start,
                    size,
                    schema: Schema::default(),
                })
            })
            .collect::<Result<Vec<Table>, ()>>().unwrap()
            // TODO ACTUALLY HANDLE ERRORS
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    size: u64,
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Schema {
    fields: Vec<Field>,
    name: String,
}

// really bad default will remove later
impl Default for Schema {
    fn default() -> Self {
        Self {
            fields: Vec::new(),
            name: String::new(),
        }
    }
}