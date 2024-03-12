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

use hex_literal::hex;
use nom::{error::ErrorKind, Needed};

use crate::{
    box_type::DESCRIPTION_BOX_TYPE,
    parser::{DataBox, Error},
};

#[test]
fn simple_box() {
    let jumbf = hex!(
        "00000026" // box size
        "6a756d64" // box type = 'jumd'
        "00000000000000000000000000000000" // UUID
        "03" // toggles
        "746573742e64657363626f7800" // label
    );

    let (rem, boxx) = DataBox::from_slice(&jumbf).unwrap();
    assert!(rem.is_empty());

    assert_eq!(
        boxx,
        DataBox {
            tbox: DESCRIPTION_BOX_TYPE,
            data: &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 116, 101, 115, 116, 46, 100,
                101, 115, 99, 98, 111, 120, 0,
            ],
            original: &jumbf,
        }
    );

    assert_eq!(format!("{boxx:#?}"), "DataBox {\n    tbox: b\"jumd\",\n    data: 30 bytes starting with [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 03, 74, 65, 73],\n    original: 38 bytes starting with [00, 00, 00, 26, 6a, 75, 6d, 64, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],\n}");
}

#[test]
fn error_incomplete_box_length() {
    let jumbf = hex!(
        "000002" // box size (invalid, needs to be 32 bits)
    );

    assert_eq!(
        DataBox::from_slice(&jumbf).unwrap_err(),
        nom::Err::Error(Error::NomError(ErrorKind::Eof))
    );
}

#[test]
fn error_incomplete_box_type() {
    let jumbf = hex!(
        "00000026" // box size
        "6a756d" // box type = 'jum' (missing last byte)
    );

    assert_eq!(
        DataBox::from_slice(&jumbf).unwrap_err(),
        nom::Err::Error(Error::Incomplete(Needed::new(4)))
    );
}

#[test]
fn error_invalid_box_length() {
    let jumbf = hex!(
        "00000002" // box size (invalid)
        "6A756D62" // box type = 'jumb'
    );

    assert_eq!(
        DataBox::from_slice(&jumbf).unwrap_err(),
        nom::Err::Error(Error::InvalidBoxLength(2,),)
    );
}

#[test]
fn read_to_eof() {
    let jumbf = hex!(
        "00000000" // box size (read to EOF)
        "6a756d64" // box type = 'jumd'
        "00000000000000000000000000000000" // UUID
        "03" // toggles
        "746573742e64657363626f7800" // label
    );

    let (rem, boxx) = DataBox::from_slice(&jumbf).unwrap();
    assert!(rem.is_empty());

    assert_eq!(
        boxx,
        DataBox {
            tbox: DESCRIPTION_BOX_TYPE,
            data: &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 116, 101, 115, 116, 46, 100,
                101, 115, 99, 98, 111, 120, 0,
            ],
            original: &jumbf,
        }
    );
}

#[test]
fn read_xlbox_size() {
    let jumbf = hex!(
        "00000001" // box size (contained in xlbox)
        "6a756d64" // box type = 'jumd'
        "000000000000002e" // XLbox (extra long box size)
        "00000000000000000000000000000000" // UUID
        "03" // toggles
        "746573742e64657363626f7800" // label
    );

    let (rem, boxx) = DataBox::from_slice(&jumbf).unwrap();
    assert!(rem.is_empty());

    assert_eq!(
        boxx,
        DataBox {
            tbox: DESCRIPTION_BOX_TYPE,
            data: &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 116, 101, 115, 116, 46, 100,
                101, 115, 99, 98, 111, 120, 0,
            ],
            original: &jumbf,
        }
    );
}

#[test]
fn error_xlbox_size_too_small() {
    let jumbf = hex!(
        "00000001" // box size (contained in xlbox)
        "6a756d64" // box type = 'jumd'
        "000000000000000e" // XLbox (INCORRECT extra long box size)
        "00000000000000000000000000000000" // UUID
        "03" // toggles
        "746573742e64657363626f7800" // label
    );

    assert_eq!(
        DataBox::from_slice(&jumbf).unwrap_err(),
        nom::Err::Error(Error::InvalidBoxLength(14,),)
    );
}

#[test]
fn error_incorrect_length() {
    let jumbf = hex!(
        "00000026" // box size
        "6a756d64" // box type = 'jumd'
        "00000000000000000000000000000000" // UUID
        "03" // toggles
        // label (missing)
    );

    assert_eq!(
        DataBox::from_slice(&jumbf).unwrap_err(),
        nom::Err::Error(Error::Incomplete(Needed::new(30)))
    );
}
