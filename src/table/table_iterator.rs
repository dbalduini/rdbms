use super::{table_page::TablePage, tuple::Tuple};

pub struct TableIterator<'p> {
    table: TablePage<'p>,
    index: usize,
}

impl<'p> TableIterator<'p> {
    pub fn new(table: TablePage<'p>) -> Self {
        TableIterator { table, index: 0 }
    }
}

impl<'p> Iterator for TableIterator<'p> {
    type Item = Tuple;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.table.num_tuples as usize {
            None
        } else {
            let tuple = self.table.get_tuple(self.index);
            self.index += 1;
            Some(tuple)
        }
    }
}
