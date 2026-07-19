//! Bridge infrastructure for loading and interacting with ONNX Runtime policies.

use ort::device::Device;
use ort::session::Session;
use ort::{Result, session};

/// A bridge manager that handles the lifecycle and execution of an ONNX Runtime inference session.
pub struct PolicyBridge {
    /// The active ONNX Runtime session, or `None` if a model has not been loaded yet.
    session: Option<Session>,
}

impl PolicyBridge {
    /// Creates a new, uninitialized `PolicyBridge` instance with no model loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// use policy_bridge::bridge::PolicyBridge;
    /// let bridge = PolicyBridge::new();
    /// ```
    pub fn new() -> Self {
        Self { session: None }
    }

    /// Executes a placeholder action.
    pub fn do_something(&self) {
        println!("hello");
    }

    /// Loads an ONNX model from the specified file path and initializes the inference session.
    ///
    /// # Errors
    ///
    /// Returns an `ort::Error` if the model file is missing, corrupt, or fails to initialize
    /// within the ONNX Runtime engine.
    pub fn load(&mut self, model_path: &str) -> Result<()> {
        let session = Session::builder()?.commit_from_file(model_path)?;
        self.session = Some(session);
        Ok(())
    }

    /// Prints the metadata (Name and Description) of the currently loaded model to stdout.
    ///
    /// If no model has been loaded via [Self::load], it prints a warning message instead.
    ///
    /// # Errors
    ///
    /// Returns an `ort::Error` if retrieving the metadata from the session fails.
    pub fn info(&self) -> Result<()> {
        if let Some(ref session) = self.session {
            let meta = session.metadata()?;
            if let Some(x) = meta.name() {
                println!("Name: {x}");
            }
            if let Some(x) = meta.description() {
                println!("Description: {x}");
            }
        } else {
            println!("The policy is not loaded yet");
        }
        Ok(())
    }

    /// A placeholder method for session build validation.
    pub fn my_code(&self) {
        let _session = Session::builder().unwrap().commit_from_file("ok").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bridge_is_empty() {
        let bridge = PolicyBridge::new();
        assert!(
            bridge.session.is_none(),
            "A new bridge should not have session loaded"
        );
    }

    #[test]
    fn test_load_non_existent_model_fails() {
        let mut bridge = PolicyBridge::new();

        // This should fail because the file doesn't exist return an Err
        let result = bridge.load("non_existent_model.onnx");
        assert!(
            result.is_err(),
            "Loading a missing file should result in an error"
        );
    }
}
