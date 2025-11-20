# Phase 8: HTTP APIs and OpenMetadata Basics 🌐

Welcome to Phase 8! Now we put everything together and start building real OpenMetadata integrations. This is where your Rust journey meets practical data catalog operations!

## 📋 Learning Objectives

By completing this phase, you will:
- ✅ Build HTTP clients with reqwest
- ✅ Serialize/deserialize JSON with serde
- ✅ Authenticate with OpenMetadata
- ✅ Perform CRUD operations on metadata
- ✅ Handle API pagination
- ✅ Implement retry logic
- ✅ Build a basic OpenMetadata client

## ⏱️ Estimated Time

7-10 days (3-4 hours per day)

## 📚 Topics Covered

### 1. HTTP with reqwest

```rust
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .get("http://localhost:8585/api/v1/tables")
        .header("Authorization", "Bearer YOUR_TOKEN")
        .send()
        .await?;

    let tables: Vec<Table> = response.json().await?;
    println!("Found {} tables", tables.len());

    Ok(())
}
```

### 2. Serde for JSON

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Table {
    pub id: String,
    pub name: String,
    pub fully_qualified_name: String,
    pub columns: Vec<Column>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DataType {
    Int,
    BigInt,
    String,
    Float,
    Boolean,
    Timestamp,
}
```

### 3. OpenMetadata Authentication

```rust
#[derive(Clone)]
pub struct AuthConfig {
    pub token: String,
}

impl AuthConfig {
    pub fn from_jwt(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }

    pub fn authorization_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}
```

### 4. OpenMetadata Client Structure

```rust
pub struct OpenMetadataClient {
    client: reqwest::Client,
    base_url: String,
    auth: AuthConfig,
}

impl OpenMetadataClient {
    pub fn new(base_url: String, auth: AuthConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            auth,
        }
    }

    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self.client
            .get(&url)
            .header("Authorization", self.auth.authorization_header())
            .send()
            .await?
            .error_for_status()?;

        response.json().await.map_err(Into::into)
    }

    async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<R> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self.client
            .post(&url)
            .header("Authorization", self.auth.authorization_header())
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?
            .error_for_status()?;

        response.json().await.map_err(Into::into)
    }
}
```

## 💻 Hands-On Exercises

### Exercise 1: Basic HTTP Client
**File**: `exercises/ex01_http_client/src/main.rs`

Build a basic HTTP client:
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // GET request
    let tables = client
        .get("http://localhost:8585/api/v1/tables")
        .header("Authorization", "Bearer TOKEN")
        .send()
        .await?
        .json::<TableList>()
        .await?;

    println!("Tables: {:#?}", tables);
    Ok(())
}
```

### Exercise 2: Table Operations
**File**: `exercises/ex02_table_ops/src/main.rs`

Implement table CRUD operations:
```rust
impl OpenMetadataClient {
    pub async fn list_tables(&self) -> Result<Vec<Table>> {
        self.get("api/v1/tables").await
    }

    pub async fn get_table(&self, fqn: &str) -> Result<Table> {
        let path = format!("api/v1/tables/name/{}", fqn);
        self.get(&path).await
    }

    pub async fn create_table(&self, request: CreateTableRequest) -> Result<Table> {
        self.post("api/v1/tables", &request).await
    }

    pub async fn update_table(&self, fqn: &str, table: Table) -> Result<Table> {
        let path = format!("api/v1/tables/name/{}", fqn);
        self.put(&path, &table).await
    }

    pub async fn delete_table(&self, fqn: &str) -> Result<()> {
        let path = format!("api/v1/tables/name/{}", fqn);
        self.delete(&path).await
    }
}
```

### Exercise 3: Database Service Operations
**File**: `exercises/ex03_database_service/src/main.rs`

Work with database services:
```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseService {
    pub id: String,
    pub name: String,
    pub service_type: ServiceType,
    pub connection: ConnectionConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceType {
    Snowflake,
    #[serde(rename = "BigQuery")]
    BigQuery,
    PostgreSQL,
    MySQL,
    Redshift,
}

impl OpenMetadataClient {
    pub async fn create_database_service(
        &self,
        request: CreateDatabaseServiceRequest,
    ) -> Result<DatabaseService> {
        self.post("api/v1/services/databaseServices", &request).await
    }

    pub async fn list_database_services(&self) -> Result<Vec<DatabaseService>> {
        self.get("api/v1/services/databaseServices").await
    }
}
```

### Exercise 4: Lineage Operations
**File**: `exercises/ex04_lineage/src/main.rs`

Add and query lineage:
```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddLineageRequest {
    pub edge: LineageEdge,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageEdge {
    pub from_entity: EntityReference,
    pub to_entity: EntityReference,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityReference {
    pub id: String,
    #[serde(rename = "type")]
    pub entity_type: String,
}

impl OpenMetadataClient {
    pub async fn add_lineage(&self, request: AddLineageRequest) -> Result<()> {
        self.post("api/v1/lineage", &request).await
    }

    pub async fn get_lineage(&self, fqn: &str) -> Result<LineageGraph> {
        let path = format!("api/v1/lineage/table/name/{}", fqn);
        self.get(&path).await
    }
}
```

### Exercise 5: Pagination
**File**: `exercises/ex05_pagination/src/main.rs`

Handle paginated responses:
```rust
#[derive(Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub paging: Option<Paging>,
}

#[derive(Debug, Deserialize)]
pub struct Paging {
    pub total: u64,
    pub after: Option<String>,
    pub before: Option<String>,
}

impl OpenMetadataClient {
    pub async fn list_all_tables(&self) -> Result<Vec<Table>> {
        let mut all_tables = Vec::new();
        let mut after: Option<String> = None;

        loop {
            let mut params = vec![("limit", "100")];
            if let Some(cursor) = &after {
                params.push(("after", cursor));
            }

            let response: PaginatedResponse<Table> =
                self.get_with_params("api/v1/tables", &params).await?;

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
}
```

## 🎯 Main Project: OpenMetadata Rust Client

Build a complete, functional OpenMetadata client:

**Project Structure:**
```
openmetadata-client/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── client.rs
│   ├── auth.rs
│   ├── error.rs
│   ├── types/
│   │   ├── mod.rs
│   │   ├── table.rs
│   │   ├── database.rs
│   │   ├── service.rs
│   │   ├── lineage.rs
│   │   └── common.rs
│   └── api/
│       ├── mod.rs
│       ├── tables.rs
│       ├── databases.rs
│       ├── services.rs
│       └── lineage.rs
├── examples/
│   ├── list_tables.rs
│   ├── create_table.rs
│   ├── add_lineage.rs
│   └── query_metadata.rs
└── tests/
    └── integration_test.rs
```

**Cargo.toml:**
```toml
[package]
name = "openmetadata-client"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }
thiserror = "1.0"
anyhow = "1.0"
url = "2.5"

[dev-dependencies]
mockito = "1.2"
```

**Key Features to Implement:**

1. **Authentication**
   - JWT token support
   - Token refresh (if needed)

2. **Core Entities**
   - Tables
   - Databases
   - Schemas
   - Database Services
   - Storage Services
   - Pipeline Services

3. **Operations**
   - List (with pagination)
   - Get by name/ID
   - Create
   - Update
   - Delete
   - Search

4. **Lineage**
   - Add lineage relationships
   - Query lineage graph
   - Get upstream/downstream

5. **Tags and Metadata**
   - Add tags
   - Remove tags
   - Update descriptions
   - Manage owners

6. **Error Handling**
   - Network errors
   - API errors
   - Deserialization errors
   - Custom error types

## 📖 Real-World Examples

### Example 1: List All Tables

```rust
use openmetadata_client::{Client, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new("http://localhost:8585/api", "YOUR_JWT_TOKEN");
    let client = Client::new(config);

    let tables = client.tables().list_all().await?;

    for table in tables {
        println!("{}: {}", table.fully_qualified_name, table.name);
        println!("  Columns: {}", table.columns.len());
        if let Some(desc) = &table.description {
            println!("  Description: {}", desc);
        }
    }

    Ok(())
}
```

### Example 2: Create Table with Lineage

```rust
use openmetadata_client::types::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = create_client();

    // Create table
    let create_request = CreateTableRequest {
        name: "users".to_string(),
        database_schema: "snowflake.db.public".to_string(),
        columns: vec![
            Column {
                name: "id".to_string(),
                data_type: DataType::BigInt,
                description: Some("User ID".to_string()),
            },
            Column {
                name: "email".to_string(),
                data_type: DataType::String,
                description: Some("User email".to_string()),
            },
        ],
        description: Some("User table".to_string()),
    };

    let table = client.tables().create(create_request).await?;
    println!("Created table: {}", table.id);

    // Add lineage
    let source_table = client.tables().get("source.db.schema.raw_users").await?;

    client.lineage().add(
        EntityReference::table(&source_table.id),
        EntityReference::table(&table.id),
    ).await?;

    println!("Added lineage relationship");

    Ok(())
}
```

### Example 3: Batch Operations

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = create_client();

    let fqns = vec![
        "db.schema.table1",
        "db.schema.table2",
        "db.schema.table3",
    ];

    // Fetch concurrently
    let tables = client.tables().get_batch(&fqns).await?;

    for (fqn, result) in fqns.iter().zip(tables) {
        match result {
            Ok(table) => println!("✓ {}: {} columns", fqn, table.columns.len()),
            Err(e) => println!("✗ {}: {}", fqn, e),
        }
    }

    Ok(())
}
```

## ✅ Knowledge Check

Before moving to Phase 9, ensure you can:

- [ ] Make HTTP requests with reqwest
- [ ] Serialize/deserialize with serde
- [ ] Handle JSON API responses
- [ ] Implement authentication
- [ ] Handle pagination correctly
- [ ] Implement retry logic
- [ ] Create CRUD operations
- [ ] Work with OpenMetadata API
- [ ] Handle API errors gracefully
- [ ] Write integration tests
- [ ] Use async/await for HTTP calls
- [ ] Batch operations efficiently

## 🎓 Best Practices

### API Client Design

1. **Separation of Concerns**
   - Client handles HTTP
   - Types module for data structures
   - API module for operations

2. **Error Handling**
   - Custom error types
   - Context on failures
   - Proper error propagation

3. **Retry Logic**
   - Exponential backoff
   - Max retries
   - Only retry transient errors

4. **Rate Limiting**
   - Respect API limits
   - Token bucket algorithm
   - Concurrent request limits

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_list_tables() {
        let mut server = Server::new_async().await;
        let mock = server.mock("GET", "/api/v1/tables")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"data": []}"#)
            .create_async()
            .await;

        let client = Client::new(Config::new(&server.url(), "token"));
        let tables = client.tables().list().await.unwrap();

        mock.assert_async().await;
        assert_eq!(tables.len(), 0);
    }
}
```

## 📚 Additional Resources

- [reqwest Documentation](https://docs.rs/reqwest/)
- [serde Documentation](https://serde.rs/)
- [OpenMetadata API Docs](https://docs.open-metadata.org/sdk/python)
- [REST API Best Practices](https://restfulapi.net/)

## ➡️ Next Steps

Ready to build a production-ready SDK?

**[Phase 9: OpenMetadata SDK Development](../phase-09-openmetadata-sdk/README.md)**

You'll create a comprehensive, well-tested SDK with advanced features!
