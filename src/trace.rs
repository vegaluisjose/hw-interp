use std::collections::HashMap;
use std::collections::VecDeque;

type TraceData = VecDeque<i32>;
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
    pub fn enq(&mut self, id: &str, value: i32) {
        if let Some(data) = self.map.get_mut(id) {
            data.push_front(value);
        } else {
            let mut data = VecDeque::new();
            data.push_front(value);
            self.map.insert(id.to_string(), data);
        }
    }

    pub fn deq(&mut self, id: &str) -> i32 {
        if let Some(data) = self.map.get_mut(id) {
            if let Some(value) = data.pop_back() {
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