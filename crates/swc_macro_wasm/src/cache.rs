use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use swc_ecma_ast::Program;
use regex::Regex;
use once_cell::sync::Lazy;
use crate::config::OptimizationConfig;

/// Global cache for compiled regexes to avoid recompilation
pub static REGEX_CACHE: Lazy<Arc<Mutex<HashMap<String, Regex>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

/// Compiled regexes used throughout the optimization process
pub static MODULE_PATTERN_STR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#""([^"]+)"\s*:\s*"#).expect("Failed to compile module pattern string regex")
});

pub static MODULE_PATTERN_NUM: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?m)^\s*(\d+)\s*:\s*"#).expect("Failed to compile module pattern number regex")
});

pub static REQUIRE_PATTERN_STR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"__webpack_require__\s*\(\s*"([^"]+)"\s*\)"#).expect("Failed to compile require pattern string regex")
});

pub static REQUIRE_PATTERN_NUM: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"__webpack_require__\s*\(\s*(\d+)\s*\)"#).expect("Failed to compile require pattern number regex")
});

pub static ENTRY_POINT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"__webpack_require__\(([^)]+)\);").expect("Failed to compile entry point regex")
});

/// AST emission cache for reducing redundant code generation
#[derive(Debug)]
pub struct AstEmissionCache {
    cache: HashMap<u64, String>,
    max_size: usize,
    hits: usize,
    misses: usize,
}

impl AstEmissionCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size.min(1000)),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    
    /// Get cached emission or None if not found
    pub fn get(&mut self, program: &Program) -> Option<String> {
        let hash = self.compute_ast_hash(program);
        if let Some(cached) = self.cache.get(&hash) {
            self.hits += 1;
            Some(cached.clone())
        } else {
            self.misses += 1;
            None
        }
    }
    
    /// Store emission in cache
    pub fn store(&mut self, program: &Program, emission: String) {
        if self.cache.len() >= self.max_size {
            // Simple eviction: remove oldest entry
            if let Some(key) = self.cache.keys().next().copied() {
                self.cache.remove(&key);
            }
        }
        
        let hash = self.compute_ast_hash(program);
        self.cache.insert(hash, emission);
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            hits: self.hits,
            misses: self.misses,
            entries: self.cache.len(),
            max_size: self.max_size,
            hit_rate: if self.hits + self.misses == 0 { 0.0 } else { 
                self.hits as f64 / (self.hits + self.misses) as f64 
            },
        }
    }
    
    /// Clear cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
    
    /// Compute a hash of the AST for caching purposes
    fn compute_ast_hash(&self, program: &Program) -> u64 {
        let mut hasher = DefaultHasher::new();
        
        // Hash based on the structure and key content of the AST
        // This is a simplified approach - in production we might want a more sophisticated hash
        match program {
            Program::Module(module) => {
                "module".hash(&mut hasher);
                module.body.len().hash(&mut hasher);
                // Hash first few statements for uniqueness
                for (i, stmt) in module.body.iter().enumerate() {
                    if i >= 5 { break; } // Limit to avoid excessive computation
                    format!("{:?}", stmt).hash(&mut hasher);
                }
            }
            Program::Script(script) => {
                "script".hash(&mut hasher);
                script.body.len().hash(&mut hasher);
                // Hash first few statements
                for (i, stmt) in script.body.iter().enumerate() {
                    if i >= 5 { break; }
                    format!("{:?}", stmt).hash(&mut hasher);
                }
            }
        }
        
        hasher.finish()
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub entries: usize,
    pub max_size: usize,
    pub hit_rate: f64,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cache stats: {} entries ({}/{}), hit rate: {:.2}%, hits: {}, misses: {}", 
               self.entries, self.entries, self.max_size, self.hit_rate * 100.0, self.hits, self.misses)
    }
}

/// Collection reuse pool to reduce allocations
#[derive(Debug)]
pub struct CollectionPool {
    string_vecs: Vec<Vec<String>>,
    usize_vecs: Vec<Vec<usize>>,
    hash_sets: Vec<std::collections::HashSet<String>>,
    enabled: bool,
}

impl CollectionPool {
    pub fn new(enabled: bool) -> Self {
        Self {
            string_vecs: Vec::new(),
            usize_vecs: Vec::new(),
            hash_sets: Vec::new(),
            enabled,
        }
    }
    
    /// Get a reusable string vector
    pub fn get_string_vec(&mut self) -> Vec<String> {
        if self.enabled && !self.string_vecs.is_empty() {
            let mut vec = self.string_vecs.pop().unwrap();
            vec.clear();
            vec
        } else {
            Vec::new()
        }
    }
    
    /// Return a string vector to the pool
    pub fn return_string_vec(&mut self, mut vec: Vec<String>) {
        if self.enabled && self.string_vecs.len() < 50 { // Limit pool size
            vec.clear();
            self.string_vecs.push(vec);
        }
    }
    
    /// Get a reusable usize vector
    pub fn get_usize_vec(&mut self) -> Vec<usize> {
        if self.enabled && !self.usize_vecs.is_empty() {
            let mut vec = self.usize_vecs.pop().unwrap();
            vec.clear();
            vec
        } else {
            Vec::new()
        }
    }
    
    /// Return a usize vector to the pool
    pub fn return_usize_vec(&mut self, mut vec: Vec<usize>) {
        if self.enabled && self.usize_vecs.len() < 50 {
            vec.clear();
            self.usize_vecs.push(vec);
        }
    }
    
    /// Get a reusable hash set
    pub fn get_hash_set(&mut self) -> std::collections::HashSet<String> {
        if self.enabled && !self.hash_sets.is_empty() {
            let mut set = self.hash_sets.pop().unwrap();
            set.clear();
            set
        } else {
            std::collections::HashSet::new()
        }
    }
    
    /// Return a hash set to the pool
    pub fn return_hash_set(&mut self, mut set: std::collections::HashSet<String>) {
        if self.enabled && self.hash_sets.len() < 50 {
            set.clear();
            self.hash_sets.push(set);
        }
    }
    
    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        PoolStats {
            string_vecs: self.string_vecs.len(),
            usize_vecs: self.usize_vecs.len(),
            hash_sets: self.hash_sets.len(),
            enabled: self.enabled,
        }
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub string_vecs: usize,
    pub usize_vecs: usize,
    pub hash_sets: usize,
    pub enabled: bool,
}

impl std::fmt::Display for PoolStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pool stats (enabled: {}): {} string vecs, {} usize vecs, {} hash sets", 
               self.enabled, self.string_vecs, self.usize_vecs, self.hash_sets)
    }
}

/// Get a compiled regex from cache or compile and cache it
pub fn get_cached_regex(pattern: &str) -> Result<Regex, regex::Error> {
    let mut cache = REGEX_CACHE.lock().unwrap();
    
    if let Some(regex) = cache.get(pattern) {
        Ok(regex.clone())
    } else {
        let regex = Regex::new(pattern)?;
        cache.insert(pattern.to_string(), regex.clone());
        Ok(regex)
    }
}

/// Global optimization context that maintains caches and pools
pub struct OptimizationContext {
    pub ast_cache: AstEmissionCache,
    pub collection_pool: CollectionPool,
    pub config: OptimizationConfig,
}

impl OptimizationContext {
    pub fn new(config: OptimizationConfig) -> Self {
        let cache_size = config.memory_config.max_cache_size;
        let reuse_enabled = config.memory_config.reuse_collections;
        
        Self {
            ast_cache: AstEmissionCache::new(cache_size),
            collection_pool: CollectionPool::new(reuse_enabled),
            config,
        }
    }
    
    /// Print performance statistics if monitoring is enabled
    pub fn print_stats(&self) {
        if self.config.enable_performance_monitoring {
            println!("Optimization Performance Stats:");
            println!("  {}", self.ast_cache.stats());
            println!("  {}", self.collection_pool.stats());
        }
    }
}