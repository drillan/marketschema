//! Thread-safe global adapter registry.

use crate::adapter::BaseAdapter;
use crate::error::AdapterError;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Type alias for adapter factory functions.
pub type AdapterFactory = Arc<dyn Fn() -> Box<dyn BaseAdapter> + Send + Sync>;

/// Global registry for adapters.
static REGISTRY: Lazy<RwLock<HashMap<String, AdapterFactory>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Thread-safe global registry for adapter discovery.
///
/// The registry allows adapters to be registered by their source name
/// and retrieved later for dynamic adapter selection.
///
/// # Example
///
/// ```ignore
/// use marketschema_adapters::AdapterRegistry;
///
/// // Register an adapter
/// AdapterRegistry::register("myapi", || Box::new(MyAdapter::new()))?;
///
/// // Get an adapter instance
/// if let Some(adapter) = AdapterRegistry::get("myapi")? {
///     println!("Source: {}", adapter.source_name());
/// }
/// ```
pub struct AdapterRegistry;

impl AdapterRegistry {
    /// Registers an adapter factory with the given source name.
    ///
    /// Returns an error if an adapter with the same name is already registered,
    /// or if the registry lock is poisoned.
    pub fn register<F>(source_name: impl Into<String>, factory: F) -> Result<(), AdapterError>
    where
        F: Fn() -> Box<dyn BaseAdapter> + Send + Sync + 'static,
    {
        let name = source_name.into();
        let mut registry = REGISTRY.write().map_err(|e| {
            AdapterError::LockPoisoned(format!("Failed to acquire write lock: {}", e))
        })?;

        if registry.contains_key(&name) {
            return Err(AdapterError::DuplicateRegistration(name));
        }

        registry.insert(name, Arc::new(factory));
        Ok(())
    }

    /// Gets an adapter instance by source name.
    ///
    /// Returns `Ok(None)` if no adapter is registered with the given name.
    /// Returns an error if the registry lock is poisoned.
    pub fn get(source_name: &str) -> Result<Option<Box<dyn BaseAdapter>>, AdapterError> {
        let registry = REGISTRY.read().map_err(|e| {
            AdapterError::LockPoisoned(format!("Failed to acquire read lock: {}", e))
        })?;
        Ok(registry.get(source_name).map(|factory| factory()))
    }

    /// Returns a list of all registered adapter source names.
    ///
    /// Returns an error if the registry lock is poisoned.
    pub fn list_adapters() -> Result<Vec<String>, AdapterError> {
        let registry = REGISTRY.read().map_err(|e| {
            AdapterError::LockPoisoned(format!("Failed to acquire read lock: {}", e))
        })?;
        Ok(registry.keys().cloned().collect())
    }

    /// Checks if an adapter is registered with the given source name.
    ///
    /// Returns an error if the registry lock is poisoned.
    pub fn is_registered(source_name: &str) -> Result<bool, AdapterError> {
        let registry = REGISTRY.read().map_err(|e| {
            AdapterError::LockPoisoned(format!("Failed to acquire read lock: {}", e))
        })?;
        Ok(registry.contains_key(source_name))
    }

    /// Clears all registered adapters.
    ///
    /// This is intended for testing to ensure test isolation.
    ///
    /// # Warning
    ///
    /// Do not use in production code as it removes all registered adapters.
    #[cfg(any(test, feature = "test-utils"))]
    pub fn clear() -> Result<(), AdapterError> {
        let mut registry = REGISTRY.write().map_err(|e| {
            AdapterError::LockPoisoned(format!("Failed to acquire write lock: {}", e))
        })?;
        registry.clear();
        Ok(())
    }
}
