//! BaseAdapter trait for HTTP client integration.
//!
//! This module provides the [`BaseAdapter`] trait that adapter implementations
//! can use to easily access a shared HTTP client.
//!
//! # Overview
//!
//! The `BaseAdapter` trait provides a standard way for adapter implementations
//! to access an HTTP client. The trait requires implementors to provide a
//! `http_client()` method that returns an `Arc<AsyncHttpClient>`.
//!
//! # Pattern: Lazy Initialization with OnceLock
//!
//! The recommended pattern uses [`std::sync::OnceLock`] for lazy initialization:
//!
//! ```rust
//! use std::sync::{Arc, OnceLock};
//! use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder, BaseAdapter};
//!
//! struct MyAdapter {
//!     http_client: OnceLock<Arc<AsyncHttpClient>>,
//! }
//!
//! impl MyAdapter {
//!     fn new() -> Self {
//!         Self {
//!             http_client: OnceLock::new(),
//!         }
//!     }
//! }
//!
//! impl BaseAdapter for MyAdapter {
//!     fn http_client(&self) -> Arc<AsyncHttpClient> {
//!         self.http_client
//!             .get_or_init(|| {
//!                 Arc::new(AsyncHttpClientBuilder::new().build().unwrap())
//!             })
//!             .clone()
//!     }
//! }
//! ```
//!
//! # Pattern: Constructor Injection
//!
//! For testing or when you need a custom client configuration:
//!
//! ```rust
//! use std::sync::Arc;
//! use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder, BaseAdapter};
//!
//! struct MyAdapter {
//!     http_client: Arc<AsyncHttpClient>,
//! }
//!
//! impl MyAdapter {
//!     fn new(client: Arc<AsyncHttpClient>) -> Self {
//!         Self {
//!             http_client: client,
//!         }
//!     }
//! }
//!
//! impl BaseAdapter for MyAdapter {
//!     fn http_client(&self) -> Arc<AsyncHttpClient> {
//!         self.http_client.clone()
//!     }
//! }
//! ```
//!
//! See: specs/003-http-client-rust/spec.md (US6: BaseAdapter トレイトとの統合)

use std::sync::Arc;

use crate::AsyncHttpClient;

/// Base adapter trait with HTTP client support.
///
/// This trait provides a standard interface for adapter implementations to
/// access a shared HTTP client. The client is returned as an `Arc<AsyncHttpClient>`
/// to allow sharing across multiple tasks without ownership transfer.
///
/// # Thread Safety
///
/// The trait requires `Send + Sync` bounds, meaning all implementors must be
/// safe to share across threads. This is essential for use in async contexts
/// where tasks may run on different threads.
///
/// # Resource Management
///
/// When an adapter is dropped, the reference count on the `Arc<AsyncHttpClient>`
/// is decremented. The HTTP client is only dropped when all references are released.
/// This follows Rust's RAII pattern for automatic resource cleanup.
///
/// # Example
///
/// ```rust
/// use std::sync::{Arc, OnceLock};
/// use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder, BaseAdapter};
///
/// struct ExchangeAdapter {
///     http_client: OnceLock<Arc<AsyncHttpClient>>,
///     base_url: String,
/// }
///
/// impl ExchangeAdapter {
///     fn new(base_url: &str) -> Self {
///         Self {
///             http_client: OnceLock::new(),
///             base_url: base_url.to_string(),
///         }
///     }
///
///     async fn fetch_ticker(&self, symbol: &str) -> Result<serde_json::Value, marketschema_http::HttpError> {
///         let url = format!("{}/ticker/{}", self.base_url, symbol);
///         self.http_client().get_json(&url).await
///     }
/// }
///
/// impl BaseAdapter for ExchangeAdapter {
///     fn http_client(&self) -> Arc<AsyncHttpClient> {
///         self.http_client
///             .get_or_init(|| {
///                 Arc::new(AsyncHttpClientBuilder::new().build().unwrap())
///             })
///             .clone()
///     }
/// }
/// ```
pub trait BaseAdapter: Send + Sync {
    /// Get the HTTP client.
    ///
    /// Returns an `Arc<AsyncHttpClient>` that can be used to make HTTP requests.
    /// The implementation should ensure thread-safe access, typically using
    /// [`std::sync::OnceLock`] for lazy initialization.
    ///
    /// # Returns
    ///
    /// An `Arc<AsyncHttpClient>` that is shared across calls. Multiple calls
    /// to this method should return clones of the same `Arc`, not new client
    /// instances.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::sync::{Arc, OnceLock};
    /// use marketschema_http::{AsyncHttpClient, AsyncHttpClientBuilder, BaseAdapter};
    ///
    /// struct MyAdapter {
    ///     http_client: OnceLock<Arc<AsyncHttpClient>>,
    /// }
    ///
    /// impl BaseAdapter for MyAdapter {
    ///     fn http_client(&self) -> Arc<AsyncHttpClient> {
    ///         self.http_client
    ///             .get_or_init(|| {
    ///                 Arc::new(AsyncHttpClientBuilder::new().build().unwrap())
    ///             })
    ///             .clone()
    ///     }
    /// }
    /// ```
    fn http_client(&self) -> Arc<AsyncHttpClient>;
}

// Compile-time verification that BaseAdapter is object-safe
// This allows using `dyn BaseAdapter` for trait objects
const _: () = {
    #[allow(dead_code)]
    fn assert_object_safe(_: &dyn BaseAdapter) {}
};
