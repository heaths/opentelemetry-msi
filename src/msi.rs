// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![allow(dead_code)]

use std::ffi::CString;

type LPCSTR = *const i8;
pub type MSIHANDLE = u32;
pub type UINT = u32;

pub const ERROR_SUCCESS: UINT = 0u32;
pub const ERROR_INSTALL_FAILURE: UINT = 1603u32;

pub type INSTALLMESSAGE = u32;
pub const INSTALLMESSAGE_FATALEXIT: INSTALLMESSAGE = 0x0000_0000u32;
pub const INSTALLMESSAGE_ERROR: INSTALLMESSAGE = 0x0100_0000u32;
pub const INSTALLMESSAGE_WARNING: INSTALLMESSAGE = 0x0200_0000u32;
pub const INSTALLMESSAGE_USER: INSTALLMESSAGE = 0x0300_0000u32;
pub const INSTALLMESSAGE_INFO: INSTALLMESSAGE = 0x0400_0000u32;
pub const INSTALLMESSAGE_ACTIONSTART: INSTALLMESSAGE = 0x0800_0000u32;
pub const INSTALLMESSAGE_ACTIONDATA: INSTALLMESSAGE = 0x0900_0000u32;

pub enum RecordField {
    String(String),
    Integer(i32),
}

pub struct Record {
    h: MSIHANDLE,
}

impl Record {
    pub fn new(message: Option<String>, fields: Vec<RecordField>) -> Self {
        unsafe {
            let h = MsiCreateRecord(fields.len() as u32);
            if let Some(message) = message {
                let message = CString::new(message).unwrap();
                MsiRecordSetString(h, 0, message.as_ptr());
            }

            for (idx, field) in fields.iter().enumerate() {
                let idx: u32 = idx.try_into().unwrap();
                match field {
                    RecordField::String(s) => {
                        let s = CString::new(s.as_bytes()).unwrap();
                        MsiRecordSetString(h, idx + 1, s.as_ptr());
                    }
                    RecordField::Integer(i) => {
                        MsiRecordSetInteger(h, idx + 1, *i);
                    }
                }
            }

            Record { h }
        }
    }
}

impl From<&str> for Record {
    fn from(s: &str) -> Self {
        unsafe {
            let h = MsiCreateRecord(0u32);
            let s = CString::new(s).unwrap();
            MsiRecordSetString(h, 0, s.as_ptr());

            Record { h }
        }
    }
}

impl From<String> for Record {
    fn from(s: String) -> Self {
        unsafe {
            let h = MsiCreateRecord(0u32);
            let s = CString::new(s).unwrap();
            MsiRecordSetString(h, 0, s.as_ptr());

            Record { h }
        }
    }
}

impl Drop for Record {
    fn drop(&mut self) {
        unsafe {
            MsiCloseHandle(self.h);
        }
    }
}

pub fn process_message(h: MSIHANDLE, message_type: INSTALLMESSAGE, record: &Record) -> i32 {
    unsafe { MsiProcessMessage(h, message_type, record.h) }
}

#[link(name = "msi")]
extern "C" {
    fn MsiCloseHandle(hAny: MSIHANDLE) -> i32;

    fn MsiCreateRecord(cParams: UINT) -> MSIHANDLE;

    fn MsiProcessMessage(
        hInstall: MSIHANDLE,
        eMessageType: INSTALLMESSAGE,
        hRecord: MSIHANDLE,
    ) -> i32;

    fn MsiRecordSetInteger(hRecord: MSIHANDLE, iField: UINT, iValue: i32) -> i32;

    #[link_name = "MsiRecordSetStringA"]
    fn MsiRecordSetString(hRecord: MSIHANDLE, iField: UINT, szValue: LPCSTR) -> i32;
}
