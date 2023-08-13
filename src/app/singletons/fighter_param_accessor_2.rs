use skyline::nn;

use crate::{cpp, lib};

#[repr(C)]
#[derive(Debug)]
pub struct UnkStruct1 {
    pub vtable: *const (),
    pub unk: cpp::SharedPtr<()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct UnkStruct2 {
    pub vtable: *const (),
    pub unk: cpp::SharedPtr<()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct FPA2Entry {
    pub unk1: i32,
    pub unk2: i32,
    pub unk3: UnkStruct1,
    pub unk4: UnkStruct2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FPA2Entry2 {
    pub fighter_param_index: i32,
    pub fighter_param_motion_index: i32,
    pub fighter_param_thrown_index: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FighterParam {
    pub base: lib::ParameterReceiver,
    pub vec: cpp::Vector<()>,
    pub vec2: cpp::Vector<()>,
}

#[repr(C)]
#[derive(Debug)]
pub struct FighterParamMotion {
    pub base: lib::ParameterReceiver,
    pub vec: cpp::Vector<()>,
    pub vec2: cpp::Vector<()>,
}

#[repr(C)]
pub struct FighterParamAccessor2 {
    pub fighter_param: cpp::SharedPtr<FighterParam>,
    pub fighter_param_motion: cpp::SharedPtr<lib::ParameterReceiver>, 
    pub fighter_param_thrown: cpp::SharedPtr<lib::ParameterReceiver>,
    pub spirits: cpp::SharedPtr<lib::ParameterReceiver>,
    pub amiibo: cpp::SharedPtr<lib::ParameterReceiver>,
    pub the_other_prcs: cpp::SharedPtr<()>,
    pub entries: [FPA2Entry; 94],
    pub entries_2: [FPA2Entry2; 94],
    pub unk: cpp::SharedPtr<lib::ParameterReceiver>,
    pub unk_ref_count: u32, // NOT an atomic
    pub unk2: cpp::SharedPtr<lib::ParameterReceiver>,
    pub unk2_ref_count: u32, // NOT an atomic
    pub lock: cpp::Mutex, // std::recursive_mutex
}