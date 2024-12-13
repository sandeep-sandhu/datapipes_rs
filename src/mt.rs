// file: mt.rs
// purpose: Create and execute multithreaded data pipelines

use std::env;
use std::io::Write;
use std::sync::{Arc, mpsc};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::mpsc::{Receiver, Sender};
use crate::dataobj;


pub fn construct_mt_pipe<T, C> (
    parallel_tasks: Vec<dataobj::ParallelProcess<T, C>>,
    mut serial_tasks: BinaryHeap<dataobj::SerialProcess<T, C>>,
    pipeline_input: Receiver<T>) -> (Receiver<T>, Sender<T>) {

    // save run handles
    let mut thread_run_handles: Vec<JoinHandle<()>> = Vec::new();
    // input of the pipe:
    let mut previous_rx = pipeline_input.clone();
    let (mut pipeline_output, _) = mpsc::channel::<T>();

    while let Some( dataobj::SerialProcess{ priority, id, is_enabled, is_source, is_sink, config_data, method }) = serial_tasks.pop() {
        if is_enabled == true {
            // for each item i in tasks, Start task
            // create a channel with txi, rxi
            let (txi, rxi) = mpsc::channel();
            // start a new thread with tx= txi and rx=previous_rx, and clone of config:
            let handle = thread::spawn(move || method(txi, previous_rx, &config_data));
            thread_run_handles.push(handle);
            previous_rx = rxi;
            pipeline_output = txi.clone();
        } else{
            // Ignoring disabled task id
        }
    }

    // create intermediary channel to copy message to start of serial queue and paralle queue both:
    let (par_tx, par_rx) = mpsc::channel::<T>();

    // now, add parallel tasks to receive from common input and send to common output:
    if parallel_tasks.len() > 0 {
        // TODO: implement this

        // spawn threads, adding each threads tx
    }

    (pipeline_input, pipeline_output)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(0, 0);
    }
}
