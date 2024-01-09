
const PAGE_SIZE: u32 = 4096;

struct Pager {
    file_descriptor: u32,
    file_length: u32,
    pub pages: Vec<Box<[u8; PAGE_SIZE as usize]>>
}