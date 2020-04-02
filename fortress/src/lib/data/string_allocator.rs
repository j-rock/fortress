pub struct StringAllocator {
    data: Vec<u8>,
    curr_head: usize,
}

impl StringAllocator {
    pub fn with_capacity(capacity: usize) -> Self {
        StringAllocator {
            data: Vec::with_capacity(capacity),
            curr_head: 0
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.curr_head = 0;
    }

    pub fn allocate(&mut self, s: String) -> Option<&str> {
        let input_buffer: &[u8] = s.as_bytes();
        let input_len = input_buffer.len();
        let range = self.data.get_mut(self.curr_head .. self.curr_head + input_len)?;
        range.copy_from_slice(input_buffer);
        std::str::from_utf8(range).ok()
    }
}