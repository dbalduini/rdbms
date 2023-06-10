pub type PageId = i32;
pub type FrameId = usize;
pub const PAGE_SIZE: usize = 4096;
pub const INVALID_PAGE_ID: PageId = -1;

pub struct Page {
    pub id: PageId,
    pub data: [u8; PAGE_SIZE],
    pub dirty: bool,
    pub pin_count: i32,
}

impl Page {
    pub fn new() -> Self {
        Page {
            id: INVALID_PAGE_ID,
            data: [0; PAGE_SIZE],
            dirty: false,
            pin_count: 0,
        }
    }
}
