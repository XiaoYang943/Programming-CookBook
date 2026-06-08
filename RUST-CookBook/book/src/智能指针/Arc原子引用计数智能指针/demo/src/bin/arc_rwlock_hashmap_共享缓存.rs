use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

type SharedCache = Arc<RwLock<HashMap<String, usize>>>;

fn record_request(cache: SharedCache, route: &'static str) {
    // 写入时取得独占写锁，避免多个线程同时修改 HashMap。
    let mut routes = cache.write().expect("lock poisoned");
    *routes.entry(route.to_string()).or_insert(0) += 1;
}

fn read_request_count(cache: SharedCache, route: &'static str) -> usize {
    // 读取时取得共享读锁；没有写入者时，多个读者可以同时读取。
    let routes = cache.read().expect("lock poisoned");
    routes.get(route).copied().unwrap_or(0)
}

fn main() {
    let cache: SharedCache = Arc::new(RwLock::new(HashMap::new()));

    let mut workers = Vec::new();

    for _ in 0..4 {
        // Arc::clone 只增加引用计数，让新线程也拥有同一个缓存。
        let cache = Arc::clone(&cache);
        workers.push(thread::spawn(move || {
            for _ in 0..25 {
                record_request(Arc::clone(&cache), "/api/users");
            }
        }));
    }

    for worker in workers {
        worker.join().expect("worker panicked");
    }

    assert_eq!(read_request_count(Arc::clone(&cache), "/api/users"), 100);
    assert_eq!(Arc::strong_count(&cache), 1);
}
