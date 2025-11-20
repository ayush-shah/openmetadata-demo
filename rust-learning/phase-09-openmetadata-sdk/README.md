# Phase 9: OpenMetadata SDK Development 🏗️

Welcome to Phase 9! Build a production-ready OpenMetadata SDK with advanced features. This is where you become a Rust expert by creating professional-grade software.

## 📋 Learning Objectives

By completing this phase, you will:
- ✅ Design a comprehensive SDK architecture
- ✅ Implement the builder pattern
- ✅ Create type-safe API wrappers
- ✅ Build connection pooling
- ✅ Implement caching strategies
- ✅ Add comprehensive error handling
- ✅ Write extensive tests and documentation

## ⏱️ Estimated Time

10-14 days (4-5 hours per day)

## 📚 SDK Architecture

### Project Structure

```
openmetadata-sdk/
├── Cargo.toml
├── README.md
├── LICENSE
├── crates/
│   ├── openmetadata-core/          # Core SDK
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── client.rs
│   │   │   ├── config.rs
│   │   │   ├── error.rs
│   │   │   └── http/
│   │   │       ├── mod.rs
│   │   │       ├── client.rs
│   │   │       └── retry.rs
│   │   └── tests/
│   │
│   ├── openmetadata-types/         # Type definitions
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── common.rs
│   │   │   ├── entities/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── table.rs
│   │   │   │   ├── database.rs
│   │   │   │   ├── schema.rs
│   │   │   │   ├── service.rs
│   │   │   │   ├── pipeline.rs
│   │   │   │   ├── topic.rs
│   │   │   │   ├── dashboard.rs
│   │   │   │   ├── container.rs
│   │   │   │   └── mlmodel.rs
│   │   │   ├── api/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── requests.rs
│   │   │   │   └── responses.rs
│   │   │   └── enums/
│   │   │       ├── mod.rs
│   │   │       ├── service_types.rs
│   │   │       ├── data_types.rs
│   │   │       └── tag_sources.rs
│   │   └── build.rs               # Code generation
│   │
│   ├── openmetadata-api/          # API operations
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── tables.rs
│   │   │   ├── databases.rs
│   │   │   ├── schemas.rs
│   │   │   ├── services.rs
│   │   │   ├── pipelines.rs
│   │   │   ├── topics.rs
│   │   │   ├── lineage.rs
│   │   │   ├── tags.rs
│   │   │   ├── glossary.rs
│   │   │   ├── search.rs
│   │   │   └── users.rs
│   │   └── tests/
│   │
│   └── openmetadata-cache/        # Caching layer
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── memory.rs
│       │   └── ttl.rs
│       └── tests/
│
├── examples/
│   ├── basic_usage.rs
│   ├── create_entities.rs
│   ├── lineage_tracking.rs
│   ├── bulk_operations.rs
│   └── search_metadata.rs
│
└── tests/
    ├── integration/
    │   ├── mod.rs
    │   ├── tables_test.rs
    │   ├── lineage_test.rs
    │   └── services_test.rs
    └── common/
        └── mod.rs
```

## 🏗️ Core Components

### 1. Configuration with Builder Pattern

```rust
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub auth: AuthConfig,
    pub timeout: Duration,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub cache_enabled: bool,
    pub cache_ttl: Duration,
}

pub struct ConfigBuilder {
    base_url: Option<String>,
    auth: Option<AuthConfig>,
    timeout: Duration,
    max_retries: u32,
    retry_delay: Duration,
    cache_enabled: bool,
    cache_ttl: Duration,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            base_url: None,
            auth: None,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
            cache_enabled: true,
            cache_ttl: Duration::from_secs(300),
        }
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    pub fn auth(mut self, auth: AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }

    pub fn build(self) -> Result<Config, ConfigError> {
        Ok(Config {
            base_url: self.base_url.ok_or(ConfigError::MissingBaseUrl)?,
            auth: self.auth.ok_or(ConfigError::MissingAuth)?,
            timeout: self.timeout,
            max_retries: self.max_retries,
            retry_delay: self.retry_delay,
            cache_enabled: self.cache_enabled,
            cache_ttl: self.cache_ttl,
        })
    }
}
```

### 2. Main Client

```rust
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct OpenMetadataClient {
    http_client: Arc<HttpClient>,
    cache: Arc<RwLock<Cache>>,
    config: Config,
}

impl OpenMetadataClient {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let http_client = Arc::new(HttpClient::new(&config)?);
        let cache = Arc::new(RwLock::new(Cache::new(config.cache_ttl)));

        Ok(Self {
            http_client,
            cache,
            config,
        })
    }

    /// Access table operations
    pub fn tables(&self) -> TableApi {
        TableApi::new(
            Arc::clone(&self.http_client),
            Arc::clone(&self.cache),
        )
    }

    /// Access database operations
    pub fn databases(&self) -> DatabaseApi {
        DatabaseApi::new(Arc::clone(&self.http_client))
    }

    /// Access lineage operations
    pub fn lineage(&self) -> LineageApi {
        LineageApi::new(Arc::clone(&self.http_client))
    }

    /// Access service operations
    pub fn services(&self) -> ServiceApi {
        ServiceApi::new(Arc::clone(&self.http_client))
    }

    /// Search across all entities
    pub fn search(&self) -> SearchApi {
        SearchApi::new(Arc::clone(&self.http_client))
    }

    /// Clear all caches
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}
```

### 3. Type-Safe API Operations

```rust
pub struct TableApi {
    client: Arc<HttpClient>,
    cache: Arc<RwLock<Cache>>,
}

impl TableApi {
    pub(crate) fn new(client: Arc<HttpClient>, cache: Arc<RwLock<Cache>>) -> Self {
        Self { client, cache }
    }

    /// List all tables with pagination
    pub async fn list(&self) -> Result<Vec<Table>, Error> {
        self.list_with_params(ListParams::default()).await
    }

    /// List tables with custom parameters
    pub async fn list_with_params(&self, params: ListParams) -> Result<Vec<Table>, Error> {
        let mut all_tables = Vec::new();
        let mut after: Option<String> = None;

        loop {
            let mut query_params = vec![
                ("limit", params.limit.to_string()),
            ];

            if let Some(cursor) = &after {
                query_params.push(("after", cursor.clone()));
            }

            if let Some(fields) = &params.fields {
                query_params.push(("fields", fields.join(",")));
            }

            let response: PaginatedResponse<Table> = self.client
                .get("tables", Some(&query_params))
                .await?;

            all_tables.extend(response.data);

            match response.paging {
                Some(paging) if paging.after.is_some() => {
                    after = paging.after;
                }
                _ => break,
            }
        }

        Ok(all_tables)
    }

    /// Get table by fully qualified name
    pub async fn get(&self, fqn: &str) -> Result<Table, Error> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(table) = cache.get::<Table>(&format!("table:{}", fqn)) {
                return Ok(table);
            }
        }

        // Fetch from API
        let path = format!("tables/name/{}", urlencoding::encode(fqn));
        let table: Table = self.client.get(&path, None).await?;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.set(format!("table:{}", fqn), table.clone());
        }

        Ok(table)
    }

    /// Create a new table
    pub async fn create(&self, request: CreateTableRequest) -> Result<Table, Error> {
        let table: Table = self.client.post("tables", &request).await?;

        // Invalidate list cache
        self.invalidate_list_cache().await;

        Ok(table)
    }

    /// Update existing table
    pub async fn update(&self, fqn: &str, table: Table) -> Result<Table, Error> {
        let path = format!("tables/name/{}", urlencoding::encode(fqn));
        let updated: Table = self.client.put(&path, &table).await?;

        // Invalidate cache
        {
            let mut cache = self.cache.write().await;
            cache.invalidate(&format!("table:{}", fqn));
        }

        Ok(updated)
    }

    /// Delete table
    pub async fn delete(&self, fqn: &str) -> Result<(), Error> {
        let path = format!("tables/name/{}", urlencoding::encode(fqn));
        self.client.delete(&path).await?;

        // Invalidate cache
        {
            let mut cache = self.cache.write().await;
            cache.invalidate(&format!("table:{}", fqn));
        }
        self.invalidate_list_cache().await;

        Ok(())
    }

    /// Batch get tables
    pub async fn get_batch(&self, fqns: &[String]) -> Vec<Result<Table, Error>> {
        let mut handles = Vec::new();

        for fqn in fqns {
            let client = self.client.clone();
            let cache = self.cache.clone();
            let fqn = fqn.clone();

            let handle = tokio::spawn(async move {
                TableApi::new(client, cache).get(&fqn).await
            });

            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await.unwrap());
        }

        results
    }

    /// Add tag to table
    pub async fn add_tag(&self, fqn: &str, tag: TagLabel) -> Result<(), Error> {
        let path = format!("tables/name/{}/tags", urlencoding::encode(fqn));
        self.client.patch(&path, &tag).await?;

        // Invalidate cache
        {
            let mut cache = self.cache.write().await;
            cache.invalidate(&format!("table:{}", fqn));
        }

        Ok(())
    }

    async fn invalidate_list_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.invalidate_pattern("tables:list:*");
    }
}

#[derive(Debug, Clone)]
pub struct ListParams {
    pub limit: usize,
    pub fields: Option<Vec<String>>,
}

impl Default for ListParams {
    fn default() -> Self {
        Self {
            limit: 100,
            fields: None,
        }
    }
}
```

### 4. Comprehensive Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
    },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Entity not found: {entity_type} '{fqn}'")]
    NotFound {
        entity_type: String,
        fqn: String,
    },

    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Rate limit exceeded, retry after {retry_after} seconds")]
    RateLimit {
        retry_after: u64,
    },

    #[error("Timeout after {0:?}")]
    Timeout(std::time::Duration),

    #[error("Cache error: {0}")]
    Cache(String),
}

pub type Result<T> = std::result::Result<T, Error>;

// Convert reqwest errors to our Error type
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::Timeout(std::time::Duration::from_secs(30))
        } else if err.is_status() {
            let status = err.status().unwrap();
            Error::Api {
                status: status.as_u16(),
                message: format!("{}", err),
            }
        } else {
            Error::Http(err)
        }
    }
}
```

### 5. Retry Logic with Exponential Backoff

```rust
use tokio::time::{sleep, Duration};

pub struct RetryStrategy {
    max_retries: u32,
    initial_delay: Duration,
    max_delay: Duration,
    multiplier: f64,
}

impl RetryStrategy {
    pub fn new(max_retries: u32) -> Self {
        Self {
            max_retries,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }

    pub async fn execute<F, T, E>(&self, mut operation: F) -> Result<T, E>
    where
        F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>>,
        E: std::fmt::Display,
    {
        let mut attempt = 0;
        let mut delay = self.initial_delay;

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt >= self.max_retries => {
                    return Err(e);
                }
                Err(e) if should_retry(&e) => {
                    attempt += 1;
                    tracing::warn!(
                        "Attempt {} failed: {}. Retrying in {:?}...",
                        attempt,
                        e,
                        delay
                    );

                    sleep(delay).await;

                    delay = std::cmp::min(
                        Duration::from_secs_f64(delay.as_secs_f64() * self.multiplier),
                        self.max_delay,
                    );
                }
                Err(e) => return Err(e),
            }
        }
    }
}

fn should_retry<E: std::fmt::Display>(error: &E) -> bool {
    let error_str = error.to_string();
    error_str.contains("timeout")
        || error_str.contains("connection")
        || error_str.contains("500")
        || error_str.contains("502")
        || error_str.contains("503")
}
```

## 💻 Advanced Features

### 1. Connection Pooling

```rust
use reqwest::Client;
use std::sync::Arc;

pub struct HttpClient {
    client: Client,
    config: Arc<Config>,
    retry_strategy: RetryStrategy,
}

impl HttpClient {
    pub fn new(config: &Config) -> Result<Self, Error> {
        let client = Client::builder()
            .timeout(config.timeout)
            .pool_max_idle_per_host(20)
            .pool_idle_timeout(Duration::from_secs(90))
            .build()?;

        Ok(Self {
            client,
            config: Arc::new(config.clone()),
            retry_strategy: RetryStrategy::new(config.max_retries),
        })
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        params: Option<&[(&str, String)]>,
    ) -> Result<T> {
        self.retry_strategy
            .execute(|| {
                Box::pin(self.execute_get(path, params))
            })
            .await
    }

    async fn execute_get<T: DeserializeOwned>(
        &self,
        path: &str,
        params: Option<&[(&str, String)]>,
    ) -> Result<T> {
        let url = format!("{}/api/v1/{}", self.config.base_url, path);

        let mut request = self.client
            .get(&url)
            .header("Authorization", self.config.auth.authorization_header())
            .header("Content-Type", "application/json");

        if let Some(params) = params {
            request = request.query(params);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            response.json().await.map_err(Into::into)
        } else {
            let error_body = response.text().await?;
            Err(Error::Api {
                status: status.as_u16(),
                message: error_body,
            })
        }
    }
}
```

### 2. Smart Caching

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

pub struct Cache {
    store: HashMap<String, CacheEntry>,
    ttl: Duration,
}

struct CacheEntry {
    data: Vec<u8>,
    expires_at: Instant,
}

impl Cache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            store: HashMap::new(),
            ttl,
        }
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let entry = self.store.get(key)?;

        if Instant::now() > entry.expires_at {
            return None;
        }

        serde_json::from_slice(&entry.data).ok()
    }

    pub fn set<T: Serialize>(&mut self, key: String, value: T) {
        if let Ok(data) = serde_json::to_vec(&value) {
            self.store.insert(
                key,
                CacheEntry {
                    data,
                    expires_at: Instant::now() + self.ttl,
                },
            );
        }
    }

    pub fn invalidate(&mut self, key: &str) {
        self.store.remove(key);
    }

    pub fn invalidate_pattern(&mut self, pattern: &str) {
        let prefix = pattern.trim_end_matches('*');
        self.store.retain(|k, _| !k.starts_with(prefix));
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn cleanup_expired(&mut self) {
        let now = Instant::now();
        self.store.retain(|_, entry| entry.expires_at > now);
    }
}
```

## ✅ Knowledge Check

Before moving to Phase 10, ensure you can:

- [ ] Design SDK architecture
- [ ] Implement builder pattern
- [ ] Create type-safe APIs
- [ ] Handle connection pooling
- [ ] Implement caching strategies
- [ ] Write comprehensive error types
- [ ] Implement retry logic
- [ ] Write extensive tests
- [ ] Create good documentation
- [ ] Design for extensibility

## 📚 Additional Resources

- [API Design Patterns](https://rust-lang.github.io/api-guidelines/)
- [Rust SDK Best Practices](https://rust-lang.github.io/api-guidelines/checklist.html)
- [Connection Pooling](https://docs.rs/reqwest/)

## ➡️ Next Steps

Ready to build a complete application?

**[Phase 10: Complete OpenMetadata CLI Application](../phase-10-final-project/README.md)**

Build a production-ready CLI tool that showcases everything you've learned!
