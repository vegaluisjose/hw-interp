use std::collections::HashMap;

type TraceData = Vec<i32>;
type TraceMap = HashMap<String, TraceData>;

#[derive(Clone, Debug)]
pub struct Trace {
    map: TraceMap,
}

impl Default for Trace {
    fn default() -> Trace {
        Trace {
            map: TraceMap::new(),
        }
    }
}

impl Trace {
    pub fn push_value(&mut self, id: &str, value: i32) {
        if let Some(data) = self.map.get_mut(id) {
            data.push(value);
        } else {
            self.map.insert(id.to_string(), vec![value]);
        }
    }

    pub fn pop_value(&mut self, id: &str) -> i32 {
        if let Some(data) = self.map.get_mut(id) {
            if let Some(value) = data.pop() {
                value
            } else {
                panic!("Error: id {} has empty queue", id);
            }
        } else {
            panic!("Error: {} not found", id);
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut iter = self.map.values();
        if let Some(data) = iter.next() {
            let mut current = data.len();
            while let Some(next) = iter.next() {
                if next.len() != current {
                    break;
                } else {
                    current = next.len();
                }
            }
        }
        iter.next().is_none() && !self.map.is_empty()
    }

    pub fn cycles(&self) -> u32 {
        let mut iter = self.map.values();
        if let Some(data) = iter.next() {
            data.len() as u32
        } else {
            panic!("Error: empty queue");
        }
    }
}