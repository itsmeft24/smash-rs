#[repr(C)]
#[derive(Debug)]
pub struct ParameterReceiver {
    pub vtable: *const (),
}