use crate::storage::page::*;
use crate::table::tuple::*;

/**
 * TablePage is a Slotted Page which contains Tuples and Tuples metadata.
 *
 * Slots grow to the right, tuples grow to the left.
 *
 *  ---------------------------------------------------------
 *  | HEADER | SLOTS |  ... FREE SPACE ... | ... TUPLES ... |
 *  ---------------------------------------------------------
 *                   ^ -->             <-- ^
 *     slot offset end                      head pointer
 *
 *  Header format (size in bytes):
 *  ------------------------------------
 *  | next_page_id (4)| num_tuples (4) |
 *  ------------------------------------
 */

const TABLE_PAGE_HEADER_SIZE: usize = 8;

pub struct TablePage<'p> {
    pub next_page_id: PageId,
    pub num_tuples: i32,
    // slots + free space + tuples
    pub page_data: &'p mut [u8; PAGE_SIZE],
}

impl<'p> From<&'p mut Page> for TablePage<'p> {
    fn from(value: &'p mut Page) -> Self {
        let next_page_id = ((value.data[0] as i32) << 24)
            + ((value.data[1] as i32) << 16)
            + ((value.data[2] as i32) << 8)
            + ((value.data[3] as i32) << 0);

        let num_tuples = ((value.data[4] as i32) << 24)
            + ((value.data[5] as i32) << 16)
            + ((value.data[6] as i32) << 8)
            + ((value.data[7] as i32) << 0);

        TablePage {
            next_page_id,
            num_tuples,
            page_data: &mut value.data,
        }
    }
}

impl<'p> TablePage<'p> {
    // Returns the offset where to put this tuple into
    fn next_tuple_offset(&self, t: &Tuple) -> Result<usize, String> {
        let tuple_offset = if self.num_tuples == 0 {
            PAGE_SIZE
        } else {
            let slot_end_offset = &self.page_data[8] as *const u8 as *mut i32;
            unsafe {
                let slot_end_offset = slot_end_offset.offset(self.num_tuples as isize - 1);
                *slot_end_offset as usize
            }
        };

        let tuple_offset = tuple_offset - t.len();

        // check boundry
        let slotset_size = TABLE_PAGE_HEADER_SIZE + ((self.num_tuples + 1) as usize * 4); // 1 slot = 4 bytes
        if slotset_size > tuple_offset {
            Err("tuple does not fit in this page".to_string())
        } else {
            Ok(tuple_offset)
        }
    }

    // Insert the tuple and returns its offset in the page
    pub fn insert_tuple(&mut self, t: &Tuple) -> Result<usize, String> {
        let tuple_offset = self.next_tuple_offset(t)?;

        self.page_data[tuple_offset..tuple_offset + t.len()].copy_from_slice(&t.data);

        // add a new slot poiting to this tuple
        let slot_end_offset = &self.page_data[8] as *const u8 as *mut u32;
        unsafe {
            let slot_end_offset = slot_end_offset.offset(self.num_tuples as isize);
            *slot_end_offset = tuple_offset as u32;
        }

        self.num_tuples += 1;

        Ok(tuple_offset)
    }

    // Returns the tuple in a given slot
    pub fn get_tuple(&mut self, slot_id: usize) -> Tuple {
        let head = unsafe {
            let i: usize = 8 + 4 * slot_id;
            let bytes: [u8; 4] = self.page_data[i..i + 4].try_into().unwrap();
            std::mem::transmute::<[u8; 4], u32>(bytes)
        } as usize;
        let tuple = &self.page_data[head..head + 10];
        let data: Vec<u8> = tuple.try_into().unwrap();
        Tuple { data }
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::table::table_iterator::TableIterator;

    #[test]
    fn insert_tuples() {
        let mut page = Page::new();
        let mut table = TablePage::from(&mut page);

        let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let t = &Tuple { data };
        let slot = table.insert_tuple(t).unwrap();
        assert_eq!(slot, PAGE_SIZE - t.len());

        let data: Vec<u8> = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let t = &Tuple { data };
        let slot = table.insert_tuple(t).unwrap();
        assert_eq!(slot, PAGE_SIZE - t.len() * 2);

        let data: Vec<u8> = vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30];
        let t = &Tuple { data };
        let slot = table.insert_tuple(t).unwrap();
        assert_eq!(slot, PAGE_SIZE - t.len() * 3);

        assert_eq!(table.num_tuples, 3);

        let iter = TableIterator::new(table);
        for tuple in iter {
            println!("{:?}", tuple);
        }
    }

    #[test]
    fn fails_when_page_is_full() {
        let mut page = Page::new();
        let mut table = TablePage::try_from(&mut page).expect("cannot parse page into table page");

        let data: Vec<u8> = vec![1; 1024];
        let t = &Tuple { data };
        assert!(table.insert_tuple(t).is_ok());
        assert!(table.insert_tuple(t).is_ok());
        assert!(table.insert_tuple(t).is_ok());
        assert!(table.insert_tuple(t).is_err());
    }
}
