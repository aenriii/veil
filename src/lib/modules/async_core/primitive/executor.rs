use core::task::{Context, Poll};
use alloc::collections::VecDeque;
use crate::std::desync::{Task, Executor};
use super::dummy_waker;

pub struct PrimitiveExecutor {
    pub(crate) task_queue: VecDeque<Task>,
}

impl PrimitiveExecutor {
    pub fn new() -> PrimitiveExecutor {
        PrimitiveExecutor {
            task_queue: VecDeque::new(),
        }
    }
}

impl Executor for PrimitiveExecutor {
    fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task)
    }

    fn run(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            if task.poll(&mut context).is_pending() {
                self.task_queue.push_back(task),
            }
        }
    }
}
