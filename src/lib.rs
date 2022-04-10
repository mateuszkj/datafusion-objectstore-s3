// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![warn(missing_docs)]

//! [DataFusion-ObjectStore-S3](https://github.com/datafusion-contrib/datafusion-objectstore-s3)
//! provides a `TableProvider` interface for using `Datafusion` to query data in S3.  This includes AWS S3
//! and services such as MinIO that implement the S3 API.
//!
//! ## Examples
//! Examples for querying AWS and other implementors, such as MinIO, are shown below.
//!
//! Load credentials from default AWS credential provider (such as environment or ~/.aws/credentials)
//!
//! ```ignore
//! # use std::sync::Arc;
//! # use datafusion::error::Result;
//! # use datafusion_objectstore_s3::object_store::s3::S3FileSystem;
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let s3_file_system = Arc::new(S3FileSystem::default().await);
//! # Ok(())
//! # }
//! ```
//!
//! `S3FileSystem::default()` is a convenience wrapper for `S3FileSystem::new(None, None, None, None, None, None)`.
//!
//! Connect to implementor of S3 API (MinIO, in this case) using access key and secret.
//!
//! ```rust
//! use datafusion_objectstore_s3::object_store::s3::{S3FileSystem, S3FileSystemOptions};
//! use datafusion_data_access::Result;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! // Example credentials provided by MinIO
//! const MINIO_ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
//! const MINIO_SECRET_ACCESS_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
//! const BUCKET_NAME: &str = "data";
//! const MINIO_ENDPOINT: &str = "http://localhost:9000";
//!
//! let s3_file_system = S3FileSystem::new_custom(
//!     BUCKET_NAME,
//!     MINIO_ENDPOINT,
//!     Some(MINIO_ACCESS_KEY_ID),
//!     Some(MINIO_SECRET_ACCESS_KEY),
//!     S3FileSystemOptions::from_envs()?
//! )?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! Using DataFusion's `ListingOtions` and `ListingTable` we register a table into a DataFusion `SessionContext` so that it can be queried.
//!
//! ```rust
//! use std::sync::Arc;
//!
//! use datafusion::datasource::listing::*;
//! use datafusion::datasource::TableProvider;
//! use datafusion::prelude::SessionContext;
//! use datafusion::datasource::file_format::parquet::ParquetFormat;
//! use datafusion::error::Result;
//!
//! use datafusion_objectstore_s3::object_store::s3::{S3FileSystem, S3FileSystemOptions};
//!
//! # const MINIO_ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
//! # const MINIO_SECRET_ACCESS_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
//! # const BUCKET_NAME: &str = "data";
//! # const MINIO_ENDPOINT: &str = "http://localhost:9000";
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let filename = "alltypes_plain.snappy.parquet";
//!
//! # let s3_file_system = Arc::new(S3FileSystem::new_custom(
//! #     BUCKET_NAME,
//! #     MINIO_ENDPOINT,
//! #     Some(MINIO_ACCESS_KEY_ID),
//! #     Some(MINIO_SECRET_ACCESS_KEY),
//! #     S3FileSystemOptions::default(),
//! # )?);
//!
//! let config = ListingTableConfig::new(s3_file_system, filename).infer().await?;
//!
//! let table = ListingTable::try_new(config)?;
//!
//! let mut ctx = SessionContext::new();
//!
//! ctx.register_table("tbl", Arc::new(table))?;
//!
//! let df = ctx.sql("SELECT * FROM tbl").await?;
//! df.show();
//! # Ok(())
//! # }
//! ```
//!
//! We can also register the `S3FileSystem` directly as an `ObjectStore` on an `SessionContext`. This provides an idiomatic way of creating `TableProviders` that can be queried.
//!
//! ```rust
//! use std::sync::Arc;
//!
//! use datafusion::datasource::listing::*;
//! use datafusion::datasource::TableProvider;
//! use datafusion::prelude::SessionContext;
//! use datafusion::datasource::file_format::parquet::ParquetFormat;
//! use datafusion::error::Result;
//!
//! use datafusion_objectstore_s3::object_store::s3::{S3FileSystem, S3FileSystemOptions};
//!
//! # const MINIO_ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
//! # const MINIO_SECRET_ACCESS_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
//! # const BUCKET_NAME: &str = "data";
//! # const MINIO_ENDPOINT: &str = "http://localhost:9000";
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let s3_file_system = Arc::new(S3FileSystem::new_custom(
//! #     BUCKET_NAME,
//! #     MINIO_ENDPOINT,
//! #     Some(MINIO_ACCESS_KEY_ID),
//! #     Some(MINIO_SECRET_ACCESS_KEY),
//! #     S3FileSystemOptions::default(),
//! # )?);
//! let mut ctx = SessionContext::new();
//! let runtime_env = ctx.runtime_env();
//!
//! runtime_env.register_object_store("s3", s3_file_system.clone());
//!
//! let uri = "s3://alltypes_plain.snappy.parquet";
//! let (object_store, name) = runtime_env.object_store(uri)?;
//!
//! let config = ListingTableConfig::new(s3_file_system, uri).infer().await?;
//!
//! let table = ListingTable::try_new(config)?;
//!
//! ctx.register_table("tbl", Arc::new(table))?;
//!
//! let df = ctx.sql("SELECT * FROM tbl").await?;
//! df.show();
//! # Ok(())
//! # }
//! ```
//!

pub mod object_store;
