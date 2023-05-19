use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct StringIdMap {
    map: HashMap<String, usize>,
    next_id: usize,
}

impl StringIdMap {
    pub fn to_id(&mut self, value: &str) -> usize {
        match self.map.get(value) {
            Some(id) => *id,
            None => {
                let id = self.next_id;
                self.next_id += 1;
                self.map.insert(value.to_string(), id);
                id
            }
        }
    }

    pub fn count(&self) -> usize {
        self.next_id
    }
}
