use super::disk_manager::DiskManager;
use super::page::*;
use std::collections::HashMap;

pub struct BufferPoolManager {
    // number of pages in the buffer pool
    pool_size: usize,
    // buffer pool pages
    pages: Vec<Page>,
    // next page id to be allocated
    next_page_id: PageId,
    // tracks pages that are current in memory
    page_table: HashMap<PageId, FrameId>,
    // disk manager implementation
    disk_manager: DiskManager,
    // list of free frames that dont have any page on them
    free_list: Vec<FrameId>,
}

impl BufferPoolManager {
    pub fn new(pool_size: usize, disk_manager: DiskManager) -> Self {
        let mut pages: Vec<Page> = Vec::with_capacity(pool_size);
        let mut free_list: Vec<FrameId> = Vec::with_capacity(pool_size);

        // all pages start in the free list
        for i in 0..pool_size {
            free_list.push(i.try_into().unwrap());
        }

        // alocate empty pages
        for _ in 0..pool_size {
            pages.push(Page::new());
        }

        BufferPoolManager {
            pool_size,
            pages,
            page_table: HashMap::new(),
            disk_manager,
            free_list,
            next_page_id: -1,
        }
    }

    pub fn new_page(&mut self) -> Result<&mut Page, String> {
        let next_page_id = self.next_page_id + 1;

        if let Some(frame_id) = self.free_list.pop() {
            self.next_page_id += 1;
            self.page_table.insert(next_page_id, frame_id);

            let mut page = &mut self.pages[frame_id];
            page.id = next_page_id;
            page.pin_count += 1;

            Ok(page)
        } else {
            // no replacer implemented yet
            return Err("free list is empty".to_string());
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::remove_file;
    use std::path::Path;

    const DB: &str = "bpm.db";

    fn setup() {
        remove_file(Path::new(DB)).ok();
    }

    fn teardown() {
        remove_file(Path::new(DB)).ok();
    }

    #[test]
    fn get_page_and_write_data() {
        setup();

        let disk_manager = DiskManager::new(String::from(DB));
        let mut buffer_pool = BufferPoolManager::new(2, disk_manager);

        let page: &mut Page = buffer_pool.new_page().expect("failed to allocate page");
        assert_eq!(page.id, 0);

        // write some data into the page
        page.data[..5].copy_from_slice(&[0x48, 0x65, 0x6c, 0x6c, 0x6f]);
        unsafe { assert_eq!(std::str::from_utf8_unchecked(&page.data[..5]), "Hello") }
        let page = buffer_pool.new_page().expect("failed to allocate page");
        assert_eq!(page.id, 1);

        assert!(buffer_pool.new_page().is_err(), "free list must be empty");

        teardown();
    }
}
