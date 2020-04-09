pub struct Base10CharIterator {
    data: [char; 20],
    char_len: usize,
    curr: usize,
}

impl Base10CharIterator {
    pub fn new(n: i64) -> Option<Self> {
        let mut data = ['\0'; 20];
        let mut char_len = 0;

        let negative = n < 0;
        let mut n = if negative { n.abs() } else { n };
        while n > 9 {
            data[char_len] = std::char::from_digit((n % 10) as u32, 10)?;
            char_len += 1;
            n = n / 10;
        }
        data[char_len] = std::char::from_digit(n as u32, 10)?;
        char_len += 1;

        if negative {
            data[char_len] = '-';
            char_len += 1;
        }

        data.get_mut(0..char_len)?.reverse();

        Some(Base10CharIterator {
            data,
            char_len,
            curr: 0,
        })
    }
}

impl Iterator for Base10CharIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.char_len {
            return None;
        }
        let old_curr = self.curr;
        self.curr += 1;
        unsafe {
            Some(*self.data.get_unchecked(old_curr))
        }
    }
}