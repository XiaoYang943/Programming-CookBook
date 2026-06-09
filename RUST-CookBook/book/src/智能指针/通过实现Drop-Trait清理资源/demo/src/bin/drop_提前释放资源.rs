use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
struct ThreadPool {
    active_workers: Arc<AtomicUsize>,
    max_workers: usize,
}

impl ThreadPool {
    fn new(max_workers: usize) -> Self {
        Self {
            active_workers: Arc::new(AtomicUsize::new(0)),
            max_workers,
        }
    }

    fn acquire_worker(&self) -> WorkerPermit {
        let previous = self.active_workers.fetch_add(1, Ordering::AcqRel);
        assert!(previous < self.max_workers, "no available worker");

        WorkerPermit {
            active_workers: Arc::clone(&self.active_workers),
        }
    }

    fn active_count(&self) -> usize {
        self.active_workers.load(Ordering::Acquire)
    }
}

struct WorkerPermit {
    active_workers: Arc<AtomicUsize>,
}

impl Drop for WorkerPermit {
    fn drop(&mut self) {
        // permit 被销毁时，自动把并发执行单元归还线程池。
        self.active_workers.fetch_sub(1, Ordering::AcqRel);
    }
}

fn main() {
    let pool = ThreadPool::new(1);
    let permit = pool.acquire_worker();

    assert_eq!(pool.active_count(), 1);
    println!("perform CPU-intensive work");

    // 核心工作已经完成，提前释放并发执行单元，不必等到 main 作用域结束。
    drop(permit);
    assert_eq!(pool.active_count(), 0);

    // 并发执行单元释放后，下一个任务可以立即取得它。
    let _next_permit = pool.acquire_worker();
    assert_eq!(pool.active_count(), 1);
}
