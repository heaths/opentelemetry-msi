// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

mod msi;
use msi::*;

#[no_mangle]
pub extern "C" fn OTInitialize(h: MSIHANDLE) -> UINT {
    log(h, "This is a log message");

    let record = Record::new(
        Some("This is one [1] of [2]".to_owned()),
        vec![
            RecordField::String("test".to_owned()),
            RecordField::Integer(1),
        ],
    );
    process_message(h, INSTALLMESSAGE_INFO, &record);

    ERROR_SUCCESS
}

fn log(h: MSIHANDLE, s: &str) {
    let r: Record = s.into();
    process_message(h, INSTALLMESSAGE_INFO, &r);
}
