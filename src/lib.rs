// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

mod msi;
use msi::*;

#[no_mangle]
pub extern "C" fn OTInitialize(h: MSIHANDLE) -> u32 {
    let session = Session::from(h);
    let product_code = session.property("ProductCode");

    let record = Record::new(
        Some("This is one [1] of [2] for product [3]".to_owned()),
        vec![
            Field::String("test".to_owned()),
            Field::Integer(1),
            Field::String(product_code),
        ],
    );

    session.message(MessageType::Info, &record);

    ERROR_SUCCESS
}
