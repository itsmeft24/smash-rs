use crate::{cpp, lib};

#[repr(C)]
pub struct UnkStruct1 {
    pub vtable: *const (),
    pub unk: cpp::SharedPtr<()>,
}

#[repr(C)]
pub struct UnkStruct2 {
    pub vtable: *const (),
    pub unk: cpp::SharedPtr<()>,
}

#[repr(C)]
pub struct FighterParamAccessor2Entry {
    pub unk3: u64,
    pub unk1: UnkStruct1,
    pub unk2: UnkStruct2,
}

#[repr(C)]
pub struct FighterParamAccessor2 {
    pub fighter_param: cpp::SharedPtr<lib::ParameterReceiver>,
    pub fighter_param_motion: cpp::SharedPtr<lib::ParameterReceiver>, 
    pub fighter_param_thrown: cpp::SharedPtr<lib::ParameterReceiver>,
    pub spirits: cpp::SharedPtr<lib::ParameterReceiver>,
    pub amiibo: cpp::SharedPtr<lib::ParameterReceiver>,
    pub the_other_prcs: cpp::SharedPtr<()>,
    pub entries: [FighterParamAccessor2Entry; 94],
}