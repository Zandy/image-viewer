//! Threading utilities

use std::sync::Arc;

use rayon::ThreadPool;

/// Thread pool for background tasks
pub struct ThreadPoolManager {
    pool: Arc<ThreadPool>,
}

impl ThreadPoolManager {
    /// Create a new thread pool manager
    pub fn new(num_threads: usize) -> Self {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .expect("Failed to create thread pool");

        Self {
            pool: Arc::new(pool),
        }
    }

    /// Execute a task in the thread pool
    pub fn spawn<F>(&self,
        f: F,
    ) where
        F: FnOnce() + Send + 'static,
    {
        self.pool.spawn(f);
    }

    /// Get the thread pool
    pub fn pool(&self) -> &ThreadPool {
        &self.pool
    }
}

impl Default for ThreadPoolManager {
    fn default() -> Self {
        let num_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        
        Self::new(num_threads)
    }
}
