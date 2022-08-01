// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use super::ffi;
use super::MSIHANDLE;
use std::ffi::CString;

pub enum Field {
    String(String),
    Integer(i32),
}

impl From<&str> for Field {
    fn from(s: &str) -> Self {
        Field::String(s.to_owned())
    }
}

pub struct Record {
    h: MSIHANDLE,
}

impl Record {
    pub fn new(message: Option<String>, fields: Vec<Field>) -> Self {
        unsafe {
            let h = ffi::MsiCreateRecord(fields.len() as u32);
            if let Some(message) = message {
                // TODO: Return result containing NulError if returned.
                let message = CString::new(message).unwrap();
                ffi::MsiRecordSetString(h, 0, message.as_ptr());
            }

            for (idx, field) in fields.iter().enumerate() {
                let idx: u32 = idx.try_into().unwrap();
                match field {
                    Field::String(s) => {
                        // TODO: Return result containing NulError if returned.
                        let s = CString::new(s.to_owned()).unwrap();
                        ffi::MsiRecordSetString(h, idx + 1, s.as_ptr());
                    }
                    Field::Integer(i) => {
                        ffi::MsiRecordSetInteger(h, idx + 1, *i);
                    }
                }
            }

            Record { h }
        }
    }

    pub fn field_count(&self) -> u32 {
        unsafe { ffi::MsiRecordGetFieldCount(self.h) }
    }
}

impl Drop for Record {
    fn drop(&mut self) {
        unsafe {
            ffi::MsiCloseHandle(self.h);
        }
    }
}

impl From<&str> for Record {
    fn from(s: &str) -> Self {
        unsafe {
            let h = ffi::MsiCreateRecord(0u32);
            // TODO: Return result containing NulError if returned.
            let s = CString::new(s).unwrap();
            ffi::MsiRecordSetString(h, 0, s.as_ptr());

            Record { h }
        }
    }
}

impl From<String> for Record {
    fn from(s: String) -> Self {
        unsafe {
            let h = ffi::MsiCreateRecord(0u32);
            let s = CString::new(s).unwrap();
            ffi::MsiRecordSetString(h, 0, s.as_ptr());

            Record { h }
        }
    }
}

impl Into<MSIHANDLE> for Record {
    fn into(self) -> MSIHANDLE {
        self.h
    }
}

impl Into<MSIHANDLE> for &Record {
    fn into(self) -> MSIHANDLE {
        self.h
    }
}
