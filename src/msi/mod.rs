// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![allow(dead_code)]

// See https://docs.microsoft.com/windows/win32/msi/automation-interface-reference

mod ffi;
mod record;
mod session;

pub use ffi::{
    ERROR_FUNCTION_NOT_CALLED, ERROR_INSTALL_FAILURE, ERROR_INSTALL_USEREXIT, ERROR_NO_MORE_ITEMS,
    ERROR_SUCCESS,
};
pub use record::{Field, Record};
pub use session::Session;
use std::fmt::Debug;

#[repr(C)]
pub enum MessageType {
    FatalExit = 0x0000_0000,
    Error = 0x0100_0000,
    Warning = 0x0200_0000,
    User = 0x0300_0000,
    Info = 0x0400_0000,
    ActionStart = 0x0800_0000,
    ActionData = 0x0900_0000,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct MSIHANDLE(u32);

impl From<u32> for MSIHANDLE {
    fn from(h: u32) -> Self {
        MSIHANDLE(h)
    }
}

impl Debug for MSIHANDLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
