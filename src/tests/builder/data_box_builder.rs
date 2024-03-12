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

use std::io::Cursor;

use hex_literal::hex;

use crate::{
    box_type::DESCRIPTION_BOX_TYPE,
    builder::{
        to_box::{jumbf_size, write_jumbf},
        DataBoxBuilder, ToBox,
    },
};

#[test]
fn simple_box_borrowed() {
    let expected_jumbf = hex!(
        "00000026" // box size
        "6a756d64" // box type = 'jumd'
        "00000000000000000000000000000000" // UUID
        "03" // toggles
        "746573742e64657363626f7800" // label
    );

    let boxx = DataBoxBuilder::from_borrowed(DESCRIPTION_BOX_TYPE, &expected_jumbf[8..]);

    assert_eq!(boxx.box_type(), DESCRIPTION_BOX_TYPE);
    assert_eq!(boxx.payload_size().unwrap(), 30);

    let mut payload = Cursor::new(Vec::<u8>::new());
    boxx.write_payload(&mut payload).unwrap();
    assert_eq!(*payload.into_inner(), expected_jumbf[8..]);

    assert_eq!(jumbf_size(&boxx).unwrap(), 38);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    write_jumbf(&boxx, &mut jumbf).unwrap();
    assert_eq!(*jumbf.into_inner(), expected_jumbf);
}

#[test]
fn simple_box_owned() {
    let expected_jumbf = hex!(
        "00000026" // box size
        "6a756d64" // box type = 'jumd'
        "00000000000000000000000000000000" // UUID
        "03" // toggles
        "746573742e64657363626f7800" // label
    );

    let boxx = DataBoxBuilder::from_owned(DESCRIPTION_BOX_TYPE, expected_jumbf[8..].to_owned());

    assert_eq!(boxx.box_type(), DESCRIPTION_BOX_TYPE);
    assert_eq!(boxx.payload_size().unwrap(), 30);

    let mut payload = Cursor::new(Vec::<u8>::new());
    boxx.write_payload(&mut payload).unwrap();
    assert_eq!(*payload.into_inner(), expected_jumbf[8..]);

    assert_eq!(jumbf_size(&boxx).unwrap(), 38);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    write_jumbf(&boxx, &mut jumbf).unwrap();
    assert_eq!(*jumbf.into_inner(), expected_jumbf);
}
