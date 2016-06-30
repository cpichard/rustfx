
use ofx::core::OfxHost;

pub struct Engine {
    ofx_host: OfxHost,
    // bundles
}

impl Engine {
    pub fn new() -> Engine {
        Engine { 
            ofx_host: OfxHost::new(),
        }
    }
}
