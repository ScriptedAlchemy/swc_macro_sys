use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::fmt;

/// Performance monitoring and metrics collection
#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    enabled: bool,
    timers: HashMap<String, Timer>,
    counters: HashMap<String, Counter>,
    memory_stats: MemoryStats,
    session_start: Instant,
}

impl PerformanceMonitor {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            timers: HashMap::new(),
            counters: HashMap::new(),
            memory_stats: MemoryStats::new(),
            session_start: Instant::now(),
        }
    }
    
    /// Start timing an operation
    pub fn start_timer(&mut self, name: &str) {
        if !self.enabled { return; }
        
        let timer = self.timers.entry(name.to_string()).or_insert_with(Timer::new);
        timer.start();
    }
    
    /// Stop timing an operation and record the duration
    pub fn stop_timer(&mut self, name: &str) {
        if !self.enabled { return; }
        
        if let Some(timer) = self.timers.get_mut(name) {
            timer.stop();
        }
    }
    
    /// Increment a counter
    pub fn increment(&mut self, name: &str) {
        if !self.enabled { return; }
        
        let counter = self.counters.entry(name.to_string()).or_insert_with(Counter::new);
        counter.increment();
    }
    
    /// Add value to a counter
    pub fn add_to_counter(&mut self, name: &str, value: usize) {
        if !self.enabled { return; }
        
        let counter = self.counters.entry(name.to_string()).or_insert_with(Counter::new);
        counter.add(value);
    }
    
    /// Record memory usage snapshot
    pub fn record_memory(&mut self, operation: &str) {
        if !self.enabled { return; }
        
        // In a real implementation, we would collect actual memory stats
        // For now, we'll track operation counts as a proxy
        self.memory_stats.record_operation(operation);
    }
    
    /// Get timer statistics
    pub fn get_timer_stats(&self, name: &str) -> Option<TimerStats> {
        self.timers.get(name).map(|timer| timer.stats())
    }
    
    /// Get counter value
    pub fn get_counter(&self, name: &str) -> usize {
        self.counters.get(name).map_or(0, |counter| counter.value)
    }
    
    /// Get all performance statistics
    pub fn get_all_stats(&self) -> PerformanceReport {
        let total_duration = self.session_start.elapsed();
        
        let timer_stats: HashMap<String, TimerStats> = self.timers
            .iter()
            .map(|(name, timer)| (name.clone(), timer.stats()))
            .collect();
            
        let counter_stats: HashMap<String, usize> = self.counters
            .iter()
            .map(|(name, counter)| (name.clone(), counter.value))
            .collect();
        
        PerformanceReport {
            enabled: self.enabled,
            total_duration,
            timer_stats,
            counter_stats,
            memory_stats: self.memory_stats.clone(),
        }
    }
    
    /// Print performance report
    pub fn print_report(&self) {
        if !self.enabled {
            return;
        }
        
        let report = self.get_all_stats();
        println!("{}", report);
    }
    
    /// Reset all statistics
    pub fn reset(&mut self) {
        self.timers.clear();
        self.counters.clear();
        self.memory_stats = MemoryStats::new();
        self.session_start = Instant::now();
    }
    
    /// Create a scoped timer that automatically stops when dropped
    pub fn scoped_timer(&mut self, name: &str) -> ScopedTimer {
        if self.enabled {
            self.start_timer(name);
        }
        ScopedTimer {
            name: name.to_string(),
            monitor: self as *mut PerformanceMonitor,
            enabled: self.enabled,
        }
    }
}

/// Timer for measuring operation durations
#[derive(Debug, Clone)]
struct Timer {
    start_time: Option<Instant>,
    total_duration: Duration,
    call_count: usize,
    min_duration: Duration,
    max_duration: Duration,
}

impl Timer {
    fn new() -> Self {
        Self {
            start_time: None,
            total_duration: Duration::ZERO,
            call_count: 0,
            min_duration: Duration::MAX,
            max_duration: Duration::ZERO,
        }
    }
    
    fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }
    
    fn stop(&mut self) {
        if let Some(start) = self.start_time.take() {
            let duration = start.elapsed();
            self.total_duration += duration;
            self.call_count += 1;
            
            if duration < self.min_duration {
                self.min_duration = duration;
            }
            if duration > self.max_duration {
                self.max_duration = duration;
            }
        }
    }
    
    fn stats(&self) -> TimerStats {
        let avg_duration = if self.call_count > 0 {
            self.total_duration / self.call_count as u32
        } else {
            Duration::ZERO
        };
        
        TimerStats {
            total_duration: self.total_duration,
            call_count: self.call_count,
            avg_duration,
            min_duration: if self.call_count > 0 { self.min_duration } else { Duration::ZERO },
            max_duration: self.max_duration,
        }
    }
}

/// Counter for tracking occurrences and values
#[derive(Debug, Clone)]
struct Counter {
    value: usize,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }
    
    fn increment(&mut self) {
        self.value += 1;
    }
    
    fn add(&mut self, amount: usize) {
        self.value += amount;
    }
}

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    operations: HashMap<String, usize>,
    peak_operations: usize,
}

impl MemoryStats {
    fn new() -> Self {
        Self {
            operations: HashMap::new(),
            peak_operations: 0,
        }
    }
    
    fn record_operation(&mut self, operation: &str) {
        let count = self.operations.entry(operation.to_string()).or_insert(0);
        *count += 1;
        
        let total_ops: usize = self.operations.values().sum();
        if total_ops > self.peak_operations {
            self.peak_operations = total_ops;
        }
    }
}

/// Timer statistics
#[derive(Debug, Clone)]
pub struct TimerStats {
    pub total_duration: Duration,
    pub call_count: usize,
    pub avg_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
}

impl fmt::Display for TimerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "calls: {}, total: {:?}, avg: {:?}, min: {:?}, max: {:?}",
               self.call_count, self.total_duration, self.avg_duration,
               self.min_duration, self.max_duration)
    }
}

/// Complete performance report
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub enabled: bool,
    pub total_duration: Duration,
    pub timer_stats: HashMap<String, TimerStats>,
    pub counter_stats: HashMap<String, usize>,
    pub memory_stats: MemoryStats,
}

impl fmt::Display for PerformanceReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.enabled {
            return write!(f, "Performance monitoring disabled");
        }
        
        writeln!(f, "Performance Report (total time: {:?}):", self.total_duration)?;
        
        // Timer statistics
        if !self.timer_stats.is_empty() {
            writeln!(f, "\nTimers:")?;
            let mut timers: Vec<_> = self.timer_stats.iter().collect();
            timers.sort_by(|a, b| b.1.total_duration.cmp(&a.1.total_duration));
            
            for (name, stats) in timers {
                writeln!(f, "  {}: {}", name, stats)?;
            }
        }
        
        // Counter statistics
        if !self.counter_stats.is_empty() {
            writeln!(f, "\nCounters:")?;
            let mut counters: Vec<_> = self.counter_stats.iter().collect();
            counters.sort_by(|a, b| b.1.cmp(a.1));
            
            for (name, value) in counters {
                writeln!(f, "  {}: {}", name, value)?;
            }
        }
        
        // Memory statistics
        if !self.memory_stats.operations.is_empty() {
            writeln!(f, "\nMemory Operations:")?;
            writeln!(f, "  Peak concurrent operations: {}", self.memory_stats.peak_operations)?;
            
            let mut ops: Vec<_> = self.memory_stats.operations.iter().collect();
            ops.sort_by(|a, b| b.1.cmp(a.1));
            
            for (operation, count) in ops {
                writeln!(f, "  {}: {}", operation, count)?;
            }
        }
        
        Ok(())
    }
}

/// RAII timer that automatically stops when dropped
pub struct ScopedTimer {
    name: String,
    monitor: *mut PerformanceMonitor,
    enabled: bool,
}

impl Drop for ScopedTimer {
    fn drop(&mut self) {
        if self.enabled {
            unsafe {
                (*self.monitor).stop_timer(&self.name);
            }
        }
    }
}

// Macro for convenient timing
#[macro_export]
macro_rules! time_operation {
    ($monitor:expr, $name:expr, $block:block) => {
        {
            let _timer = $monitor.scoped_timer($name);
            $block
        }
    };
}

/// Performance estimation for common operations
pub struct PerformanceEstimator;

impl PerformanceEstimator {
    /// Estimate potential performance improvement from optimizations
    pub fn estimate_improvement(
        modules_before: usize,
        modules_after: usize,
        iterations: usize,
        total_time: Duration,
    ) -> PerformanceEstimate {
        let modules_removed = modules_before.saturating_sub(modules_after);
        let removal_percentage = if modules_before > 0 {
            (modules_removed as f64 / modules_before as f64) * 100.0
        } else {
            0.0
        };
        
        let avg_time_per_iteration = if iterations > 0 {
            total_time / iterations as u32
        } else {
            Duration::ZERO
        };
        
        // Estimate size reduction (roughly proportional to module count)
        let estimated_size_reduction = removal_percentage;
        
        // Estimate parse time improvement (roughly linear with size)
        let estimated_parse_improvement = removal_percentage * 0.8; // Slightly less than linear
        
        PerformanceEstimate {
            modules_removed,
            removal_percentage,
            iterations_taken: iterations,
            avg_time_per_iteration,
            estimated_size_reduction,
            estimated_parse_improvement,
            optimization_efficiency: if total_time.as_millis() > 0 {
                modules_removed as f64 / total_time.as_millis() as f64
            } else {
                0.0
            },
        }
    }
}

/// Performance improvement estimate
#[derive(Debug, Clone)]
pub struct PerformanceEstimate {
    pub modules_removed: usize,
    pub removal_percentage: f64,
    pub iterations_taken: usize,
    pub avg_time_per_iteration: Duration,
    pub estimated_size_reduction: f64,
    pub estimated_parse_improvement: f64,
    pub optimization_efficiency: f64, // modules removed per millisecond
}

impl fmt::Display for PerformanceEstimate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Performance Estimate:")?;
        writeln!(f, "  Modules removed: {} ({:.1}%)", self.modules_removed, self.removal_percentage)?;
        writeln!(f, "  Iterations: {} (avg: {:?})", self.iterations_taken, self.avg_time_per_iteration)?;
        writeln!(f, "  Estimated size reduction: {:.1}%", self.estimated_size_reduction)?;
        writeln!(f, "  Estimated parse improvement: {:.1}%", self.estimated_parse_improvement)?;
        writeln!(f, "  Optimization efficiency: {:.2} modules/ms", self.optimization_efficiency)?;
        Ok(())
    }
}