# Phase 7: Async/Await and Concurrency 🔄

Welcome to Phase 7! Master asynchronous programming and concurrency in Rust. Essential for building performant network applications and working with OpenMetadata API.

## 📋 Learning Objectives

By completing this phase, you will:
- ✅ Understand async/await syntax
- ✅ Work with Tokio runtime
- ✅ Handle concurrent operations
- ✅ Use async streams and channels
- ✅ Manage shared state safely
- ✅ Build async HTTP clients

## ⏱️ Estimated Time

7-10 days (3-4 hours per day)

## 📚 Topics Covered

### 1. Async Basics

```rust
// Async function
async fn fetch_data() -> String {
    String::from("data")
}

// Calling async functions
#[tokio::main]
async fn main() {
    let data = fetch_data().await;
    println!("{}", data);
}
```

### 2. Futures

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// A future is a value that may not be ready yet
async fn async_operation() -> Result<String, Error> {
    // Async work
    Ok("result".to_string())
}
```

### 3. Tokio Runtime

```rust
// Single-threaded runtime
#[tokio::main]
async fn main() {
    // Code here
}

// Multi-threaded runtime
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // Code here
}

// Manual runtime
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // Async code
    });
}
```

### 4. Concurrent Operations

```rust
use tokio::join;
use tokio::try_join;

async fn fetch_tables() -> Vec<Table> { /* ... */ }
async fn fetch_schemas() -> Vec<Schema> { /* ... */ }
async fn fetch_databases() -> Vec<Database> { /* ... */ }

// Run concurrently, wait for all
async fn fetch_all() {
    let (tables, schemas, databases) = join!(
        fetch_tables(),
        fetch_schemas(),
        fetch_databases()
    );
}

// With error handling
async fn fetch_all_fallible() -> Result<(), Error> {
    let (tables, schemas) = try_join!(
        fetch_tables_result(),
        fetch_schemas_result()
    )?;
    Ok(())
}
```

### 5. Spawning Tasks

```rust
use tokio::task;

async fn process_table(table: Table) {
    // Process table
}

async fn process_many(tables: Vec<Table>) {
    let mut handles = vec![];

    for table in tables {
        let handle = task::spawn(async move {
            process_table(table).await;
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }
}
```

### 6. Channels

```rust
use tokio::sync::mpsc;

async fn producer_consumer() {
    let (tx, mut rx) = mpsc::channel(100);

    // Producer
    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });

    // Consumer
    while let Some(value) = rx.recv().await {
        println!("Received: {}", value);
    }
}
```

### 7. Shared State

```rust
use tokio::sync::{Mutex, RwLock};
use std::sync::Arc;

// Mutex for exclusive access
let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);

tokio::spawn(async move {
    let mut num = counter_clone.lock().await;
    *num += 1;
});

// RwLock for multiple readers
let data = Arc::new(RwLock::new(HashMap::new()));
let data_clone = Arc::clone(&data);

// Read
{
    let reader = data.read().await;
    println!("{:?}", reader.get("key"));
}

// Write
{
    let mut writer = data.write().await;
    writer.insert("key", "value");
}
```

### 8. Async Streams

```rust
use tokio_stream::{StreamExt, Stream};

async fn stream_example() {
    let mut stream = tokio_stream::iter(vec![1, 2, 3, 4, 5]);

    while let Some(value) = stream.next().await {
        println!("{}", value);
    }
}

// Async generator
async fn generate_numbers() -> impl Stream<Item = i32> {
    tokio_stream::iter(0..100)
}
```

## 💻 Hands-On Exercises

### Exercise 1: Basic Async Operations
**File**: `exercises/ex01_async_basics.rs`

Practice async/await:
```rust
async fn fetch_user(id: u32) -> User {
    // Simulate API call
    tokio::time::sleep(Duration::from_millis(100)).await;
    User { id, name: format!("User {}", id) }
}

#[tokio::main]
async fn main() {
    let user = fetch_user(1).await;
    println!("{:?}", user);
}
```

### Exercise 2: Concurrent Requests
**File**: `exercises/ex02_concurrent.rs`

Fetch multiple resources concurrently:
```rust
async fn fetch_all_metadata() -> Result<Metadata, Error> {
    let (tables, schemas, databases) = try_join!(
        fetch_tables(),
        fetch_schemas(),
        fetch_databases()
    )?;

    Ok(Metadata { tables, schemas, databases })
}
```

### Exercise 3: Worker Pool
**File**: `exercises/ex03_worker_pool.rs`

Build a task queue with workers:
```rust
struct WorkerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>,
}

impl WorkerPool {
    fn new(size: usize) -> Self { /* ... */ }
    async fn execute(&self, task: Task) { /* ... */ }
}
```

### Exercise 4: Rate Limiter
**File**: `exercises/ex04_rate_limiter.rs`

Implement a rate limiter for API calls:
```rust
struct RateLimiter {
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    async fn acquire(&self) -> bool { /* ... */ }
}
```

### Exercise 5: Async Cache
**File**: `exercises/ex05_async_cache.rs`

Build an async cache with expiration:
```rust
struct AsyncCache<K, V> {
    store: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
}

impl<K, V> AsyncCache<K, V> {
    async fn get(&self, key: &K) -> Option<V> { /* ... */ }
    async fn set(&self, key: K, value: V) { /* ... */ }
    async fn invalidate(&self, key: &K) { /* ... */ }
}
```

## 🎯 Mini-Project: Async HTTP Client

Build an async HTTP client for OpenMetadata:

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct OpenMetadataClient {
    client: Client,
    base_url: String,
    token: String,
    cache: Arc<RwLock<Cache>>,
}

impl OpenMetadataClient {
    pub fn new(base_url: String, token: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            token,
            cache: Arc::new(RwLock::new(Cache::new())),
        }
    }

    pub async fn list_tables(&self) -> Result<Vec<Table>, Error> {
        // Check cache
        {
            let cache = self.cache.read().await;
            if let Some(tables) = cache.get("tables") {
                return Ok(tables.clone());
            }
        }

        // Fetch from API
        let url = format!("{}/api/v1/tables", self.base_url);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        let tables: Vec<Table> = response.json().await?;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.set("tables", tables.clone());
        }

        Ok(tables)
    }

    pub async fn get_table(&self, fqn: &str) -> Result<Table, Error> {
        let url = format!("{}/api/v1/tables/name/{}", self.base_url, fqn);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        response.json().await.map_err(Into::into)
    }

    pub async fn create_table(&self, table: CreateTableRequest) -> Result<Table, Error> {
        let url = format!("{}/api/v1/tables", self.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&table)
            .send()
            .await?;

        response.json().await.map_err(Into::into)
    }

    // Batch operation - concurrent
    pub async fn get_tables_batch(&self, fqns: Vec<String>) -> Vec<Result<Table, Error>> {
        let mut handles = vec![];

        for fqn in fqns {
            let client = self.clone();
            let handle = tokio::spawn(async move {
                client.get_table(&fqn).await
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            results.push(handle.await.unwrap());
        }

        results
    }
}
```

**Features:**
1. Async HTTP requests
2. Concurrent batch operations
3. Caching layer
4. Error handling
5. Token authentication
6. Rate limiting (bonus)

## 📖 Async Patterns

### Pattern 1: Timeout

```rust
use tokio::time::{timeout, Duration};

async fn fetch_with_timeout() -> Result<Data, Error> {
    timeout(Duration::from_secs(5), fetch_data())
        .await
        .map_err(|_| Error::Timeout)?
}
```

### Pattern 2: Retry Logic

```rust
async fn fetch_with_retry(max_retries: u32) -> Result<Data, Error> {
    for attempt in 0..max_retries {
        match fetch_data().await {
            Ok(data) => return Ok(data),
            Err(e) if e.is_retryable() => {
                tokio::time::sleep(Duration::from_secs(2_u64.pow(attempt))).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    Err(Error::MaxRetriesExceeded)
}
```

### Pattern 3: Select

```rust
use tokio::select;

async fn first_to_complete() {
    let task1 = fetch_from_source1();
    let task2 = fetch_from_source2();

    select! {
        result = task1 => println!("Source 1: {:?}", result),
        result = task2 => println!("Source 2: {:?}", result),
    }
}
```

### Pattern 4: Background Tasks

```rust
use tokio::task::JoinHandle;

struct BackgroundProcessor {
    handle: Option<JoinHandle<()>>,
}

impl BackgroundProcessor {
    fn start() -> Self {
        let handle = tokio::spawn(async {
            loop {
                process_background_tasks().await;
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        });

        Self { handle: Some(handle) }
    }

    async fn stop(mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}
```

## ✅ Knowledge Check

Before moving to Phase 8, ensure you can:

- [ ] Write async functions
- [ ] Use .await correctly
- [ ] Set up Tokio runtime
- [ ] Run concurrent operations with join!
- [ ] Spawn background tasks
- [ ] Use channels for communication
- [ ] Share state with Arc<Mutex<T>>
- [ ] Work with async streams
- [ ] Handle timeouts and cancellation
- [ ] Implement retry logic
- [ ] Use select! for racing futures
- [ ] Build async HTTP clients

## 🎓 Performance Tips

### Avoid Blocking in Async Code

```rust
// ❌ Bad - blocks the executor
async fn bad_example() {
    std::thread::sleep(Duration::from_secs(1));
}

// ✅ Good - yields to executor
async fn good_example() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}

// For blocking IO, use spawn_blocking
async fn with_blocking_io() {
    let result = tokio::task::spawn_blocking(|| {
        // Blocking IO here
        std::fs::read_to_string("file.txt")
    }).await.unwrap();
}
```

### Buffer Size Tuning

```rust
// Small buffer for backpressure
let (tx, rx) = mpsc::channel(10);

// Large buffer for throughput
let (tx, rx) = mpsc::channel(10000);

// Unbounded (use carefully)
let (tx, rx) = mpsc::unbounded_channel();
```

## 📚 Additional Resources

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Documentation](https://docs.rs/tokio/)
- [reqwest Documentation](https://docs.rs/reqwest/)

## ➡️ Next Steps

Ready to integrate with OpenMetadata API?

**[Phase 8: HTTP APIs and OpenMetadata Basics](../phase-08-http-apis/README.md)**

You'll build a real HTTP client for OpenMetadata and start interacting with the API!
