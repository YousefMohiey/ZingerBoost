use zb_domain::errors::ServiceError;

/// Windows Service Controller using SCM APIs
#[derive(Debug)]
pub struct ServiceController;

impl ServiceController {
    pub fn new() -> Self {
        Self
    }

    pub fn query_start_type(&self, _name: &str) -> Result<u32, ServiceError> {
        // Placeholder
        Ok(2) // SERVICE_AUTO_START
    }

    pub fn set_start_type(&self, _name: &str, _start_type: u32) -> Result<(), ServiceError> {
        // Placeholder
        Ok(())
    }
}
