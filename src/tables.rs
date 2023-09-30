use crate::rows::ROW_SIZE;


const PAGE_SIZE: u32 = 4096;
const TABLE_MAX_PAGES: u32 = 100;
const ROWS_PER_PAGE: u32 = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS: u32 = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    pub num_rows: u32,
    pub pages: Vec<Box<[u8; PAGE_SIZE as usize]>>
}

impl Table {

    pub fn new() -> Self {
        let num_rows = 0;
        let mut pages: Vec<Box<[u8; PAGE_SIZE as usize]>> = vec![];

        for _ in 0..TABLE_MAX_PAGES {
            pages.push(Box::new([0u8; PAGE_SIZE as usize]))
        }

        Self { num_rows, pages }
    }

    pub fn row_slot(&mut self, row_num: u32) -> &mut [u8] {
        let page_num: usize = (row_num / ROWS_PER_PAGE).try_into().unwrap();
        let _page = &self.pages[page_num];

        // if page.is_none() {
        //     page = &Some(Box::new([0u8; PAGE_SIZE as usize]));
        //     self.pages[page_num] = page.clone();
        // }

        let row_offset: u32 = row_num % ROWS_PER_PAGE;
        let byte_offset: u32 = row_offset * ROW_SIZE;
        println!("row_off: {}, byte_offset: {}", row_offset, byte_offset);
        &mut self.pages[page_num][byte_offset as usize..(byte_offset + ROW_SIZE) as usize]
    }
}