use std::sync::Mutex;
use test_run_options::ExternalResource;

// Functionality that is not part of ExternalResource's public API but is useful for testing
pub trait ExternalResourceExt {
    fn reset(&mut self);
}
impl ExternalResourceExt for ExternalResource {
    fn reset(&mut self) {
        unimplemented!();
    }
}

lazy_static::lazy_static! {
    // You abstract mutually exclusive access to an external resource through this global var
    pub static ref EXTERNAL_RESOURCE: Mutex<ExternalResource> = Mutex::new(ExternalResource);
}
