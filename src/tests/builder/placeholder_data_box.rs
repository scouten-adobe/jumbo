// Copyright 2024 Adobe. All rights reserved.
// This file is licensed to you under the Apache License,
// Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// or the MIT license (http://opensource.org/licenses/MIT),
// at your option.

// Unless required by applicable law or agreed to in writing,
// this software is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR REPRESENTATIONS OF ANY KIND, either express or
// implied. See the LICENSE-MIT and LICENSE-APACHE files for the
// specific language governing permissions and limitations under
// each license.

use std::io::{Cursor, Write};

use hex_literal::hex;

use crate::{
    builder::{
        to_box::{jumbf_size, write_jumbf},
        PlaceholderDataBox, ToBox,
    },
    BoxType,
};

const RANDOM_BOX_TYPE: BoxType = BoxType(*b"abcd");

#[test]
fn simple_case() {
    let expected_jumbf = hex!(
        "00000018" // box size
        "61626364" // box type = 'abcd'
        "00000000000000000000000000000000" // placeholder
    );

    let pbox = PlaceholderDataBox::new(RANDOM_BOX_TYPE, 16);

    assert_eq!(pbox.box_type(), RANDOM_BOX_TYPE);
    assert_eq!(pbox.payload_size().unwrap(), 16);
    assert_eq!(jumbf_size(&pbox).unwrap(), 24);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    write_jumbf(&pbox, &mut jumbf).unwrap();
    assert_eq!(*jumbf.get_ref(), expected_jumbf);

    let expected_jumbf = hex!(
        "00000018" // box size
        "61626364" // box type = 'abcd'
        "31323334353637383930000000000000" // replacement payload
    );

    pbox.replace_payload(&mut jumbf, &expected_jumbf[8..18])
        .unwrap();
    assert_eq!(*jumbf.get_ref(), expected_jumbf);
}

#[test]
fn error_write_payload_only() {
    // PlaceholderDataBox reports an error if its .write_payload() method
    // is called by itself.

    let pbox = PlaceholderDataBox::new(RANDOM_BOX_TYPE, 16);

    let mut payload = Cursor::new(Vec::<u8>::new());
    let err = pbox.write_payload(&mut payload).unwrap_err();
    assert_eq!(
        "Custom { kind: Other, error: \"placeholder stream should have some data already\" }",
        format!("{err:?}")
    );
}

#[test]
fn error_payload_too_large() {
    let expected_jumbf = hex!(
        "00000018" // box size
        "61626364" // box type = 'abcd'
        "00000000000000000000000000000000" // placeholder
    );

    let pbox = PlaceholderDataBox::new(RANDOM_BOX_TYPE, 16);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    write_jumbf(&pbox, &mut jumbf).unwrap();
    assert_eq!(*jumbf.get_ref(), expected_jumbf);

    let payload_too_large = [1u8; 17];
    let err = pbox
        .replace_payload(&mut jumbf, &payload_too_large)
        .unwrap_err();

    assert_eq!(
            "Custom { kind: Other, error: \"replace_payload: payload (17 bytes) is larger than reserved capacity (16 bytes)\" }",
            format!("{err:?}")
        );

    // No part of the original JUMBF as written should have been changed.
    assert_eq!(*jumbf.get_ref(), expected_jumbf);
}

#[test]
fn error_write_jumbf_not_called() {
    let pbox = PlaceholderDataBox::new(RANDOM_BOX_TYPE, 16);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    let payload = [1u8; 16];
    let err = pbox.replace_payload(&mut jumbf, &payload).unwrap_err();

    assert_eq!(
            "Custom { kind: Other, error: \"replace_payload: no offset recorded; call write_jumbf() first\" }",
            format!("{err:?}")
        );

    // No part of the original JUMBF as written should have been changed.
    assert_eq!(*jumbf.get_ref(), []);
}

#[test]
fn offset() {
    let expected_jumbf = hex!(
        "41424344" // arbitrary prefix = 'ABCD'
        "00000018" // box size
        "61626364" // box type = 'abcd'
        "00000000000000000000000000000000" // placeholder
    );

    let pbox = PlaceholderDataBox::new(RANDOM_BOX_TYPE, 16);

    assert_eq!(pbox.box_type(), RANDOM_BOX_TYPE);
    assert_eq!(pbox.payload_size().unwrap(), 16);
    assert_eq!(jumbf_size(&pbox).unwrap(), 24);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    jumbf.write_all(b"ABCD").unwrap();

    write_jumbf(&pbox, &mut jumbf).unwrap();
    assert_eq!(*jumbf.get_ref(), expected_jumbf);

    assert_eq!(pbox.offset(), Some(12));
}

#[test]
fn offset_before_write() {
    let pbox = PlaceholderDataBox::new(RANDOM_BOX_TYPE, 16);

    assert_eq!(pbox.box_type(), RANDOM_BOX_TYPE);
    assert_eq!(pbox.payload_size().unwrap(), 16);
    assert_eq!(jumbf_size(&pbox).unwrap(), 24);
    assert_eq!(pbox.offset(), None);
}
