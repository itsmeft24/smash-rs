#[repr(align(8))]
#[derive(Debug)]
pub struct Mutex(pub [u8; 32]);
