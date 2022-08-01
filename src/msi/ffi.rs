// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use super::MSIHANDLE;
use std::os::raw::c_char;

pub(crate) type LPSTR = *mut c_char;
pub(crate) type LPCSTR = *const c_char;

pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_NO_MORE_ITEMS: u32 = 259;
pub const ERROR_INSTALL_USEREXIT: u32 = 1602;
pub const ERROR_INSTALL_FAILURE: u32 = 1603;
pub const ERROR_FUNCTION_NOT_CALLED: u32 = 1626;

pub(crate) const ERROR_MORE_DATA: u32 = 234;

#[link(name = "msi")]
extern "C" {
    pub fn MsiCloseHandle(hAny: MSIHANDLE) -> i32;

    pub fn MsiCreateRecord(cParams: u32) -> MSIHANDLE;

    #[link_name = "MsiGetPropertyA"]
    pub fn MsiGetProperty(
        hInstall: MSIHANDLE,
        szName: LPCSTR,
        szValueBuf: LPSTR,
        // cspell:disable-next-line
        pcchValueBuf: *mut u32,
    ) -> u32;

    pub fn MsiProcessMessage(hInstall: MSIHANDLE, eMessageType: u32, hRecord: MSIHANDLE) -> i32;

    pub fn MsiRecordGetFieldCount(hRecord: MSIHANDLE) -> u32;

    pub fn MsiRecordSetInteger(hRecord: MSIHANDLE, iField: u32, iValue: i32) -> i32;

    #[link_name = "MsiRecordSetStringA"]
    pub fn MsiRecordSetString(hRecord: MSIHANDLE, iField: u32, szValue: LPCSTR) -> i32;
}
