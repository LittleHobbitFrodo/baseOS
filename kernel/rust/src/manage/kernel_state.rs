//	manage/kernel_state.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build





/// [`KernelState`] is enumerator used to check what phase is kernel in
/// - you are supposed to edit this enum for your specific use
pub enum KernelState {
    Init(KernelInitState),
    Runtime(KernelRuntimeState),
    Shutdown(KernelShutdownState)
}

pub enum KernelInitState {
    Unknown,
    Base,
    Memory,
    //  you can add more!
}

pub enum KernelRuntimeState {
    Unknown,
}

pub enum KernelShutdownState {
    Unknown,
}






impl KernelState {
    pub fn as_str(&self) -> &'static [u8] {
        match self {
            Self::Init(i) => i.as_str(),
            Self::Runtime(r) => r.as_str(),
            Self::Shutdown(s) => s.as_str(),
        }
    }
}

impl KernelInitState {
    pub fn as_str(&self) -> &'static[u8] {
        match self {
            Self::Unknown => b"unknown",
            Self::Base => b"BASE",
            Self::Memory => b"Memory",
        }
    }
}

impl KernelRuntimeState {
    pub fn as_str(&self) -> &'static [u8] {
        match self {
            Self::Unknown => b"unknown",
        }
    }
}

impl KernelShutdownState {
    pub fn as_str(&self) -> &'static [u8] {
        match self {
            Self::Unknown => b"unknown",
        }
    }
}


