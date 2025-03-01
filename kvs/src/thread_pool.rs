use crate::Result;

pub trait ThreadPool: Sized {
    fn new(threads: u32) -> Result<Self>;
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}

pub struct NaiveThreadPool {}

impl ThreadPool for NaiveThreadPool {
    fn new(threads: u32) -> Result<Self> {
        todo!()
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}

pub struct SharedQueueThreadPool {}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<Self> {
        todo!()
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}

// TODO: use rayon
pub struct RayonThreadPool {}

impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> Result<Self> {
        todo!()
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}
