#[derive(Debug)]
pub struct Tuple {
    pub data: Vec<u8>,
}

impl Tuple {
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
