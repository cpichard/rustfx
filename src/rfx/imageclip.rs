use rfx::propertyset::*;

#[derive(Clone)]
pub struct OfxImageClip {
    // TODO move ImageClip where it belongs and fill with relevant code
    pub props: Box<OfxPropertySet>, // TODO set field private and add accessor
}

impl OfxImageClip {
    pub fn new() -> Self {
        OfxImageClip { props: OfxPropertySet::new() }
    }
}
