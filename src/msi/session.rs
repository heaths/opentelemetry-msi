// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use super::ffi;
use super::{MessageType, Record, MSIHANDLE};
use std::ffi::CString;

pub struct Session {
    h: MSIHANDLE,
    owned: bool,
}

impl Session {
    pub fn message(&self, kind: MessageType, record: &Record) -> i32 {
        unsafe { ffi::MsiProcessMessage(self.h, kind as u32, record.into()) }
    }

    pub fn property(&self, name: &str) -> String {
        unsafe {
            // TODO: Return result containing NulError if returned.
            let name = CString::new(name).unwrap();

            let mut value_len = 0u32;
            let value = CString::default();

            if ffi::MsiGetProperty(
                self.h,
                name.as_ptr(),
                value.as_ptr() as ffi::LPSTR,
                &mut value_len as *mut u32,
            ) == ffi::ERROR_MORE_DATA
            {
                let mut value_len = value_len + 1u32;
                let mut value: Vec<u8> = vec![0; value_len as usize];

                ffi::MsiGetProperty(
                    self.h,
                    name.as_ptr(),
                    value.as_mut_ptr() as ffi::LPSTR,
                    &mut value_len as *mut u32,
                );

                value.truncate(value_len as usize);
                return String::from_utf8(value).unwrap();
            }

            String::default()
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                ffi::MsiCloseHandle(self.h);
            }
        }
    }
}

impl From<MSIHANDLE> for Session {
    fn from(h: MSIHANDLE) -> Self {
        Session { h, owned: false }
    }
}
