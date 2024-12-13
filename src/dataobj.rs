use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SerialProcess<T, C> {
    pub priority: isize,
    pub id: usize,
    pub is_enabled: bool,
    pub is_source: bool,
    pub is_sink: bool,
    pub config_data: C,
    pub method: fn(Sender<T>, Receiver<T>, C)
}

impl Ord for SerialProcess<_, _> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

// `PartialOrd` implementation
impl PartialOrd for SerialProcess<_, _> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Clone, Eq, PartialEq)]
pub struct ParallelProcess<T, C> {
    pub id: usize,
    pub name: String,
    pub priority: isize,
    pub is_enabled: bool,
    pub is_source: bool,
    pub is_sink: bool,
    pub config_data: C,
    pub method: fn(Sender<T>, C)
}
