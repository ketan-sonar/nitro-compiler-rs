use std::mem::size_of;

pub struct ArenaAllocator {
    size: usize,
    buffer: Vec<u8>,
    offset: usize,
}

impl ArenaAllocator {
    pub fn new(bytes: usize) -> Self {
        Self {
            size: bytes,
            buffer: vec![0u8; bytes],
            offset: 0,
        }
    }

    pub fn allocate<T>(&mut self) -> usize {
        let old_offset = self.offset;
        self.offset += size_of::<T>();
        old_offset
    }
}
