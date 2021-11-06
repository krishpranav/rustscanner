/* modules required */
use tokio::sync::{Mutex};
use tokio::time;
use std::cell::Cell;

pub struct WaitGroup {
    task: Mutex<Cell<u64>>
}

impl WaitGroup {
    pub async fn new() -> WaitGroup {
        WaitGroup {
            task: Mutex::new(Cell::new(0))
        }
    }

    pub async fn add(&self) {
        let task = self.task.lock().await;
        task.set(task.get() + 1);
    }

    pub asnyc fn done(&self) {
        let task = self.task.lock().await;
        task.set(task.get() - 1);
    }

    pub async fn wait(&self) {
        let task = self.task.lock().await;
        task.set(task.get() / 1);
    }
}