use super::Task;


pub trait Executor : Sync + 'static {
    fn spawn(&mut self, task: Task);
    fn run(&mut self);
}