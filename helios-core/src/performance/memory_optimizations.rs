//! Memory Performance Optimizations
//!
//! This module provides memory optimization techniques for large-scale data visualization,
//! including memory pooling, zero-copy operations, and efficient data structures.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Custom allocator for high-performance memory management
pub struct OptimizedAllocator {
    stats: Arc<Mutex<AllocatorStats>>,
}

#[derive(Debug, Clone)]
pub struct AllocatorStats {
    pub total_allocations: u64,
    pub total_deallocations: u64,
    pub total_bytes_allocated: u64,
    pub total_bytes_deallocated: u64,
    pub peak_memory_usage: u64,
    pub current_memory_usage: u64,
}

impl OptimizedAllocator {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(AllocatorStats {
                total_allocations: 0,
                total_deallocations: 0,
                total_bytes_allocated: 0,
                total_bytes_deallocated: 0,
                peak_memory_usage: 0,
                current_memory_usage: 0,
            })),
        }
    }

    pub fn get_stats(&self) -> AllocatorStats {
        self.stats.lock().unwrap().clone()
    }
}

unsafe impl GlobalAlloc for OptimizedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            let mut stats = self.stats.lock().unwrap();
            stats.total_allocations += 1;
            stats.total_bytes_allocated += layout.size() as u64;
            stats.current_memory_usage += layout.size() as u64;
            stats.peak_memory_usage = stats.peak_memory_usage.max(stats.current_memory_usage);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        let mut stats = self.stats.lock().unwrap();
        stats.total_deallocations += 1;
        stats.total_bytes_deallocated += layout.size() as u64;
        stats.current_memory_usage = stats.current_memory_usage.saturating_sub(layout.size() as u64);
    }
}

/// Memory pool for frequently allocated objects
pub struct ObjectPool<T> {
    pool: Arc<Mutex<Vec<T>>>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
    reset_fn: Box<dyn Fn(&mut T) + Send + Sync>,
    max_size: usize,
    stats: Arc<Mutex<PoolStats>>,
}

#[derive(Debug, Clone)]
pub struct PoolStats {
    pub objects_created: u64,
    pub objects_reused: u64,
    pub objects_destroyed: u64,
    pub pool_size: usize,
}

impl<T> ObjectPool<T> {
    pub fn new<F, R>(factory: F, reset_fn: R, max_size: usize) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
        R: Fn(&mut T) + Send + Sync + 'static,
    {
        Self {
            pool: Arc::new(Mutex::new(Vec::new())),
            factory: Box::new(factory),
            reset_fn: Box::new(reset_fn),
            max_size,
            stats: Arc::new(Mutex::new(PoolStats {
                objects_created: 0,
                objects_reused: 0,
                objects_destroyed: 0,
                pool_size: 0,
            })),
        }
    }

    pub fn acquire(&self) -> T {
        let mut pool = self.pool.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if let Some(mut obj) = pool.pop() {
            (self.reset_fn)(&mut obj);
            stats.objects_reused += 1;
            stats.pool_size = pool.len();
            obj
        } else {
            stats.objects_created += 1;
            (self.factory)()
        }
    }

    pub fn release(&self, mut obj: T) {
        let mut pool = self.pool.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if pool.len() < self.max_size {
            (self.reset_fn)(&mut obj);
            pool.push(obj);
            stats.pool_size = pool.len();
        } else {
            stats.objects_destroyed += 1;
        }
    }

    pub fn get_stats(&self) -> PoolStats {
        self.stats.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        let mut pool = self.pool.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        pool.clear();
        stats.pool_size = 0;
    }
}

/// Zero-copy string interning for memory efficiency
pub struct StringInterner {
    strings: Arc<Mutex<HashMap<String, usize>>>,
    reverse: Arc<Mutex<Vec<String>>>,
    stats: Arc<Mutex<InternerStats>>,
}

#[derive(Debug, Clone)]
pub struct InternerStats {
    pub total_strings: usize,
    pub unique_strings: usize,
    pub memory_saved: usize,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            strings: Arc::new(Mutex::new(HashMap::new())),
            reverse: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(InternerStats {
                total_strings: 0,
                unique_strings: 0,
                memory_saved: 0,
            })),
        }
    }

    pub fn intern(&self, s: &str) -> usize {
        let mut strings = self.strings.lock().unwrap();
        let mut reverse = self.reverse.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        stats.total_strings += 1;
        
        if let Some(&id) = strings.get(s) {
            stats.memory_saved += s.len();
            id
        } else {
            let id = reverse.len();
            strings.insert(s.to_string(), id);
            reverse.push(s.to_string());
            stats.unique_strings += 1;
            id
        }
    }

    pub fn get(&self, id: usize) -> Option<String> {
        let reverse = self.reverse.lock().unwrap();
        reverse.get(id).cloned()
    }

    pub fn get_stats(&self) -> InternerStats {
        self.stats.lock().unwrap().clone()
    }
}

/// Memory-efficient circular buffer
pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    pub fn push(&mut self, item: T) -> Option<T> {
        let mut evicted = None;
        
        if self.size == self.capacity {
            // Buffer is full, evict oldest item
            evicted = Some(std::mem::replace(&mut self.buffer[self.head], item));
            self.head = (self.head + 1) % self.capacity;
        } else {
            // Buffer has space
            if self.buffer.len() < self.capacity {
                self.buffer.push(item);
            } else {
                self.buffer[self.tail] = item;
            }
            self.tail = (self.tail + 1) % self.capacity;
            self.size += 1;
        }
        
        evicted
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let item = std::mem::take(&mut self.buffer[self.head]);
            self.head = (self.head + 1) % self.capacity;
            self.size -= 1;
            Some(item)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            None
        } else {
            Some(&self.buffer[self.head])
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.size = 0;
    }
}

/// Memory-efficient bit set for large datasets
pub struct BitSet {
    bits: Vec<u64>,
    size: usize,
}

impl BitSet {
    pub fn new(size: usize) -> Self {
        let words = (size + 63) / 64; // Round up to nearest word
        Self {
            bits: vec![0; words],
            size,
        }
    }

    pub fn set(&mut self, index: usize) {
        if index < self.size {
            let word = index / 64;
            let bit = index % 64;
            self.bits[word] |= 1 << bit;
        }
    }

    pub fn clear(&mut self, index: usize) {
        if index < self.size {
            let word = index / 64;
            let bit = index % 64;
            self.bits[word] &= !(1 << bit);
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if index < self.size {
            let word = index / 64;
            let bit = index % 64;
            (self.bits[word] & (1 << bit)) != 0
        } else {
            false
        }
    }

    pub fn count_ones(&self) -> usize {
        self.bits.iter().map(|&word| word.count_ones() as usize).sum()
    }

    pub fn clear_all(&mut self) {
        for word in &mut self.bits {
            *word = 0;
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

/// Memory-efficient sparse matrix representation
pub struct SparseMatrix {
    data: HashMap<(usize, usize), f64>,
    rows: usize,
    cols: usize,
}

impl SparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: HashMap::new(),
            rows,
            cols,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        if row < self.rows && col < self.cols {
            if value != 0.0 {
                self.data.insert((row, col), value);
            } else {
                self.data.remove(&(row, col));
            }
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        if row < self.rows && col < self.cols {
            self.data.get(&(row, col)).copied().unwrap_or(0.0)
        } else {
            0.0
        }
    }

    pub fn non_zero_count(&self) -> usize {
        self.data.len()
    }

    pub fn memory_usage(&self) -> usize {
        self.data.len() * (std::mem::size_of::<(usize, usize)>() + std::mem::size_of::<f64>())
    }

    pub fn density(&self) -> f64 {
        self.non_zero_count() as f64 / (self.rows * self.cols) as f64
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}
