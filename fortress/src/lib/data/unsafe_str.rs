pub struct UnsafeStr {
    data: *const str
}

impl UnsafeStr {
    pub fn from(s: &str) -> Self {
        UnsafeStr {
            data: &*s
        }
    }

    pub unsafe fn dereference<'a, 'b>(&'a self) -> &'b str {
        &*self.data
    }
}