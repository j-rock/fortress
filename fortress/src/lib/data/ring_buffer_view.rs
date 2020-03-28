pub struct RingBufferView {
    len: usize,
    capacity: usize,
    curr_head: usize,
}

impl RingBufferView {
    pub fn with_capacity(capacity: usize) -> RingBufferView {
        RingBufferView {
            len: 0,
            capacity,
            curr_head: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.curr_head = 0;
    }

    pub fn increment_head(&mut self) {
        if self.len != self.capacity {
            self.len += 1;
        }
        self.curr_head = (self.curr_head + 1) % self.capacity;
    }

    pub fn drop_last(&mut self) {
        if self.len == 0 {
            return;
        }

        if self.curr_head == self.len {
            self.curr_head -= 1;
        }

        self.len -= 1;
    }

    pub fn add_element_at_head<T>(&self, element: T, data: &mut Vec<T>) {
        if self.len < self.capacity {
            data.push(element);
        } else {
            data[self.curr_head] = element;
        }
    }
}
