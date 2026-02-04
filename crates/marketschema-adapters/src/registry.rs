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
/// if let Some(adapter) = AdapterRegistry::get("myapi") {
///     println!("Source: {}", adapter.source_name());
/// }
/// ```
pub struct AdapterRegistry;

impl AdapterRegistry {
    /// Registers an adapter factory with the given source name.
    ///
    /// Returns an error if an adapter with the same name is already registered.
    pub fn register<F>(source_name: impl Into<String>, factory: F) -> Result<(), AdapterError>
    where
        F: Fn() -> Box<dyn BaseAdapter> + Send + Sync + 'static,
    {
        let name = source_name.into();
        let mut registry = REGISTRY.write().expect("Registry lock poisoned");

        if registry.contains_key(&name) {
            return Err(AdapterError::DuplicateRegistration(name));
        }

        registry.insert(name, Arc::new(factory));
        Ok(())
    }

    /// Gets an adapter instance by source name.
    ///
    /// Returns None if no adapter is registered with the given name.
    pub fn get(source_name: &str) -> Option<Box<dyn BaseAdapter>> {
        let registry = REGISTRY.read().expect("Registry lock poisoned");
        registry.get(source_name).map(|factory| factory())
    }

    /// Returns a list of all registered adapter source names.
    pub fn list_adapters() -> Vec<String> {
        let registry = REGISTRY.read().expect("Registry lock poisoned");
        registry.keys().cloned().collect()
    }

    /// Checks if an adapter is registered with the given source name.
    pub fn is_registered(source_name: &str) -> bool {
        let registry = REGISTRY.read().expect("Registry lock poisoned");
        registry.contains_key(source_name)
    }

    /// Clears all registered adapters.
    ///
    /// This is primarily intended for testing to ensure test isolation.
    pub fn clear() {
        let mut registry = REGISTRY.write().expect("Registry lock poisoned");
        registry.clear();
    }
}
