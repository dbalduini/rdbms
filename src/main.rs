use crate::storage::disk::disk_manager::*;

mod storage;

fn main() {
    let mut disk_manager = DiskManager::new(String::from("test.db"));

    let mut page: Vec<u8> = vec![0; PAGE_SIZE];
    unsafe {
        page.set_len(0);
    }
    for b in 1..10 {
        page.push(b);
    }
    disk_manager
        .write_page(0, &page)
        .expect("failed to write page");

    let mut page: Vec<u8> = vec![0; PAGE_SIZE];
    unsafe {
        page.set_len(0);
    }
    for b in 11..20 {
        page.push(b);
    }

    disk_manager
        .write_page(1, &page)
        .expect("failed to write page");

    // disk_manager.drop_database().expect("Failed to drop database");
}
