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
    builder::{DataBoxBuilder, PlaceholderDataBox, SuperBoxBuilder},
    BoxType,
};

// Used here as an illustration only. This crate does not parse JSON content.
const JSON_BOX_TYPE: BoxType = BoxType(*b"json");

const RANDOM_BOX_TYPE: BoxType = BoxType(*b"abcd");

#[test]
fn basic_case() {
    let expected_jumbf = hex!(
        "0000002e" // box size
        "6a756d62" // box type = 'jumb'
            "00000026" // box size
            "6a756d64" // box type = 'jumd'
            "00000000000000000000000000000000" // UUID
            "03" // toggles
            "746573742e64657363626f7800" // label
    );

    let sbox =
        SuperBoxBuilder::new(&hex!("00000000000000000000000000000000")).set_label("test.descbox");

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    sbox.write_jumbf(&mut jumbf).unwrap();
    assert_eq!(*jumbf.into_inner(), expected_jumbf);
}

#[test]
fn non_requestable_label() {
    let expected_jumbf = hex!(
        "0000002e" // box size
        "6a756d62" // box type = 'jumb'
            "00000026" // box size
            "6a756d64" // box type = 'jumd'
            "00000000000000000000000000000000" // UUID
            "02" // toggles
            "746573742e64657363626f7800" // label
    );

    let sbox = SuperBoxBuilder::new(&hex!("00000000000000000000000000000000"))
        .set_non_requestable_label("test.descbox");

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    sbox.write_jumbf(&mut jumbf).unwrap();
    assert_eq!(*jumbf.into_inner(), expected_jumbf);
}

#[test]
fn with_id() {
    let expected_jumbf = hex!(
        "00000025" // box size
        "6a756d62" // box type = 'jumb'
            "0000001d" // box size
            "6a756d64" // box type = 'jumd'
            "00000000000000000000000000000000" // UUID
            "04" // toggles
            "00001000" // ID
    );

    let sbox = SuperBoxBuilder::new(&hex!("00000000000000000000000000000000")).set_id(4096);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    sbox.write_jumbf(&mut jumbf).unwrap();
    assert_eq!(*jumbf.into_inner(), expected_jumbf);
}

#[test]
fn with_hash() {
    let expected_jumbf = hex!(
        "0000004e" // box size
        "6a756d62" // box type = 'jumb'
            "00000046" // box size
            "6a756d64" // box type = 'jumd'
            "00000000000000000000000000000000" // UUID
            "0b" // toggles
            "746573742e64657363626f7800" // label
            "54686973206973206120626f67757320"
            "686173682e2e2e2e2e2e2e2e2e2e2e2e" // hash
    );

    let sbox = SuperBoxBuilder::new(&hex!("00000000000000000000000000000000"))
        .set_label("test.descbox")
        .set_sha256_hash(b"This is a bogus hash............" as &[u8; 32]);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    sbox.write_jumbf(&mut jumbf).unwrap();
    assert_eq!(*jumbf.into_inner(), expected_jumbf);
}

#[test]
fn with_private_box() {
    let expected_jumbf = hex!(
        "00000057" // box size
        "6a756d62" // box type = 'jumb'
            "0000004f" // box size
            "6a756d64" // box type = 'jumd'
            "00000000000000000000000000000000" // UUID
            "13" // toggles
            "746573742e64657363626f7800" // label
                "00000029" // box size
                "6a736f6e" // box type = 'json'
                "7b20226c6f636174696f6e223a20224d61726761"
                "746520436974792c204e4a227d" // payload (JSON)
    );

    let private = DataBoxBuilder::from_owned(
        JSON_BOX_TYPE,
        hex!("7b20226c6f636174696f6e223a20224d61726761"
                   "746520436974792c204e4a227d")
        .to_vec(),
    );

    let sbox = SuperBoxBuilder::new(&hex!("00000000000000000000000000000000"))
        .set_label("test.descbox")
        .set_private_box(private);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    sbox.write_jumbf(&mut jumbf).unwrap();
    assert_eq!(*jumbf.into_inner(), expected_jumbf);
}

#[test]
fn no_label() {
    let expected_jumbf = hex!(
        "00000021" // box size
        "6a756d62" // box type = 'jumb'
            "00000019" // box size
            "6a756d64" // box type = 'jumd'
            "00000000000000000000000000000000" // UUID
            "00" // toggles
    );

    let sbox = SuperBoxBuilder::new(&hex!("00000000000000000000000000000000"));

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    sbox.write_jumbf(&mut jumbf).unwrap();
    assert_eq!(*jumbf.into_inner(), expected_jumbf);
}

#[test]
fn with_child_boxes() {
    let expected_jumbf = hex!(
        "00000056" // box size
        "6a756d62" // box type = 'jumb'
            "00000019" // box size
            "6a756d64" // box type = 'jumd'
            "00000000000000000000000000000000" // UUID
            "00" // toggles
            // ---
            "00000029" // box size
            "6a736f6e" // box type = 'json'
            "7b20226c6f636174696f6e223a20224d61726761"
            "746520436974792c204e4a227d" // payload (JSON)
            // ---
            "0000000c" // box size
            "61626364" // box type = 'abcd'
            "41424344" // payload
    );

    let cbox1 = DataBoxBuilder::from_owned(
        JSON_BOX_TYPE,
        hex!("7b20226c6f636174696f6e223a20224d61726761"
                   "746520436974792c204e4a227d")
        .to_vec(),
    );

    let cbox2 = DataBoxBuilder::from_borrowed(RANDOM_BOX_TYPE, b"ABCD");

    let sbox = SuperBoxBuilder::new(&hex!("00000000000000000000000000000000"))
        .add_child_box(cbox1)
        .add_child_box(cbox2);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    sbox.write_jumbf(&mut jumbf).unwrap();
    assert_eq!(*jumbf.into_inner(), expected_jumbf);
}

#[test]
fn with_placeholder() {
    let expected_jumbf = hex!(
        "00000062" // box size
        "6a756d62" // box type = 'jumb'
            "00000019" // box size
            "6a756d64" // box type = 'jumd'
            "00000000000000000000000000000000" // UUID
            "00" // toggles
            // ---
            "00000029" // box size
            "6a736f6e" // box type = 'json'
            "7b20226c6f636174696f6e223a20224d61726761"
            "746520436974792c204e4a227d" // payload (JSON)
            // ---
            "00000018" // box size
            "61626364" // box type = 'abcd'
            "00000000000000000000000000000000" // placeholder
    );

    let cbox = DataBoxBuilder::from_owned(
        JSON_BOX_TYPE,
        hex!("7b20226c6f636174696f6e223a20224d61726761"
                   "746520436974792c204e4a227d")
        .to_vec(),
    );

    let pbox = PlaceholderDataBox::new(RANDOM_BOX_TYPE, 16);

    let sbox = SuperBoxBuilder::new(&hex!("00000000000000000000000000000000"))
        .add_child_box(cbox)
        .add_borrowed_child_box(&pbox);

    let mut jumbf = Cursor::new(Vec::<u8>::new());
    sbox.write_jumbf(&mut jumbf).unwrap();
    assert_eq!(*jumbf.get_ref(), expected_jumbf);

    pbox.replace_payload(&mut jumbf, b"0123456789abcdef")
        .unwrap();

    let expected_jumbf = hex!(
        "00000062" // box size
        "6a756d62" // box type = 'jumb'
            "00000019" // box size
            "6a756d64" // box type = 'jumd'
            "00000000000000000000000000000000" // UUID
            "00" // toggles
            // ---
            "00000029" // box size
            "6a736f6e" // box type = 'json'
            "7b20226c6f636174696f6e223a20224d61726761"
            "746520436974792c204e4a227d" // payload (JSON)
            // ---
            "00000018" // box size
            "61626364" // box type = 'abcd'
            "30313233343536373839616263646566" // replaced payload
    );

    assert_eq!(*jumbf.get_ref(), expected_jumbf);
}
