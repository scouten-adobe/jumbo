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
use pretty_assertions_sorted::assert_eq;

use crate::{
    box_type::DESCRIPTION_BOX_TYPE,
    parser::{DataBox, Error, ReadPastEndOfSlice},
};

type TDataBox<'a> = DataBox<&'a [u8]>;

#[test]
fn simple_box() {
    let jumbf = hex!(
        "00000026" // box size
        "6a756d64" // box type = 'jumd'
        "00000000000000000000000000000000" // UUID
        "03" // toggles
        "746573742e64657363626f7800" // label
    );

    let (dbox, rem) = DataBox::from_source(jumbf.as_slice()).unwrap();
    assert!(rem.is_empty());

    assert_eq!(
        dbox,
        TDataBox {
            tbox: DESCRIPTION_BOX_TYPE,
            data: &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 116, 101, 115, 116, 46, 100,
                101, 115, 99, 98, 111, 120, 0,
            ],
            original: &jumbf,
        }
    );

    assert_eq!(format!("{dbox:#?}"), "DataBox {\n    tbox: b\"jumd\",\n    data: 30 bytes starting with [00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 03, 74, 65, 73],\n    original: 38 bytes starting with [00, 00, 00, 26, 6a, 75, 6d, 64, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00],\n}");
}

#[test]
fn error_incomplete_box_length() {
    let jumbf = hex!(
        "000002" // box size (invalid, needs to be 32 bits)
    );

    assert_eq!(
        DataBox::from_source(jumbf.as_slice()).unwrap_err(),
        Error::SourceError {
            source: ReadPastEndOfSlice { wanted: 4, have: 3 }
        }
    );
}

#[test]
fn error_incomplete_box_type() {
    let jumbf = hex!(
        "00000026" // box size
        "6a756d" // box type = 'jum' (missing last byte)
    );

    assert_eq!(
        DataBox::from_source(jumbf.as_slice()).unwrap_err(),
        Error::SourceError {
            source: ReadPastEndOfSlice { wanted: 4, have: 3 }
        }
    );
}

#[test]
fn error_invalid_box_length() {
    let jumbf = hex!(
        "00000002" // box size (invalid)
        "6A756D62" // box type = 'jumb'
    );

    assert_eq!(
        DataBox::from_source(jumbf.as_slice()).unwrap_err(),
        Error::InvalidBoxLength(2)
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

    let (dbox, rem) = DataBox::from_source(jumbf.as_slice()).unwrap();
    assert!(rem.is_empty());

    assert_eq!(
        dbox,
        TDataBox {
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

    let (dbox, rem) = DataBox::from_source(jumbf.as_slice()).unwrap();
    assert!(rem.is_empty());

    assert_eq!(
        dbox,
        TDataBox {
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
        DataBox::from_source(jumbf.as_slice()).unwrap_err(),
        Error::InvalidBoxLength(14)
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
        DataBox::from_source(jumbf.as_slice()).unwrap_err(),
        Error::SourceError {
            source: ReadPastEndOfSlice {
                wanted: 30,
                have: 17
            }
        }
    );
}

mod offset_within_superbox {
    // The "happy path" cases for offset_within_superbox are
    // covered in the SuperBox test suite. This test suite is
    // intended to prove safe behavior given incorrect and/or
    // hostile inputs.

    use hex_literal::hex;
    use pretty_assertions_sorted::assert_eq;

    use crate::parser::SuperBox;

    #[test]
    fn abuse_read_to_eof() {
        // In this test case, we abuse JUMBF's ability to use 0
        // as the "box size" to mean read to "end of input."

        // We parse the same JUMBF superblock twice with different input
        // lengths, which means the pointers will align, but the data box
        // from the longer parse run will overrun the container of the
        // shorter parse run.

        // The `offset_within_superbox` code should detect this and
        // return `None` in this case.

        let jumbf = hex!(
        "00000000" // box size
        "6a756d62" // box type = 'jumb'
            "00000028" // box size
            "6a756d64" // box type = 'jumd'
            "6332637300110010800000aa00389b71" // UUID
            "03" // toggles
            "633270612e7369676e617475726500" // label
            // ----
            "00000000" // box size
            "75756964" // box type = 'uuid'
            "6332637300110010800000aa00389b717468697320776f756c64206e6f726d616c6c792062652062696e617279207369676e617475726520646174612e2e2e" // data (type unknown)
        );

        let (sbox_full, rem) = SuperBox::from_source(jumbf.as_slice()).unwrap();
        assert!(rem.is_empty());

        assert_eq!(sbox_full.original.len(), 119);

        let (sbox_short, rem) = SuperBox::from_source(&jumbf[0..118]).unwrap();

        assert!(rem.is_empty());
        assert_eq!(sbox_short.original.len(), 118);

        let dbox_from_full = sbox_full.data_box().unwrap();

        assert_eq!(
            dbox_from_full.offset_within_superbox(&sbox_full).unwrap(),
            56
        );
        assert!(dbox_from_full.offset_within_superbox(&sbox_short).is_none());

        let dbox_as_child = sbox_full.child_boxes.first().unwrap();
        assert!(dbox_as_child.as_super_box().is_none());

        let dbox_as_child = dbox_as_child.as_data_box().unwrap();
        assert_eq!(dbox_from_full, dbox_as_child);
    }

    #[test]
    fn dbox_precedes_sbox() {
        let jumbf = hex!(
            "00000267" // box size
            "6a756d62" // box type = 'jumb'
                "0000001e" // box size
                "6a756d64" // box type = 'jumd'
                "6332706100110010800000aa00389b71" // UUID
                "03" // toggles
                "6332706100" // label = "c2pa"
                // ---
                "00000241" // box size
                "6a756d62" // box type = 'jumb'
                    "00000024" // box size
                    "6a756d64" // box type = 'jumd'
                    "63326d6100110010800000aa00389b71" // UUID
                    "03" // toggles
                    "63622e61646f62655f3100"
                        // label = "cb.adobe_1"
                    // ---
                    "0000008f" // box size
                    "6a756d62" // box type = 'jumb'
                        "00000029" // box size
                        "6a756d64" // box type = 'jumd'
                        "6332617300110010800000aa00389b71" // UUID
                        "03" // toggles
                        "633270612e617373657274696f6e7300"
                            // label = "c2pa.assertions"
                        // ---
                        "0000005e" // box size
                        "6a756d62" // box type = 'jumb'
                            "0000002d" // box size
                            "6a756d64" // box type = 'jumd'
                            "6a736f6e00110010800000aa00389b71" // UUID
                            "03" // toggles
                            "633270612e6c6f636174696f6e2e62726f616400"
                                // label = "c2pa.location.broad"
                            // ---
                            "00000029" // box size
                            "6a736f6e" // box type = 'json'
                            "7b20226c6f636174696f6e223a20224d61726761"
                            "746520436974792c204e4a227d" // payload (JSON)
                    // ---
                    "0000010f" // box size
                    "6a756d62" // box type = 'jumb'
                        "00000024" // box size
                        "6a756d64" // box type = 'jumd'
                        "6332636c00110010800000aa00389b71" // UUID
                        "03" // toggles
                        "633270612e636c61696d00"
                            // label = "c2pa.claim"
                        // ---
                        "000000e3" // box size
                        "6a736f6e" // box type = 'json'
                        "7b0a2020202020202020202020202272"
                        "65636f7264657222203a202250686f74"
                        "6f73686f70222c0a2020202020202020"
                        "20202020227369676e61747572652220"
                        "3a202273656c66236a756d62663d735f"
                        "61646f62655f31222c0a202020202020"
                        "20202020202022617373657274696f6e"
                        "7322203a205b0a202020202020202020"
                        "202020202020202273656c66236a756d"
                        "62663d61735f61646f62655f312f6332"
                        "70612e6c6f636174696f6e2e62726f61"
                        "643f686c3d3736313432424436323336"
                        "3346220a202020202020202020202020"
                        "5d0a20202020202020207d" // payload (JSON)
                    // ---
                    "00000077" // box size
                    "6a756d62" // box type = 'jumb'
                        "00000028" // box size
                        "6a756d64" // box type = 'jumd'
                        "6332637300110010800000aa00389b71" // UUID
                        "03" // toggles
                        "633270612e7369676e617475726500"
                            // label = "c2pa.signature"
                        // ---
                        "00000047" // box size
                        "75756964" // box type = 'uuid'
                        "6332637300110010800000aa00389b71"
                        "7468697320776f756c64206e6f726d61"
                        "6c6c792062652062696e617279207369"
                        "676e617475726520646174612e2e2e"
        );

        let (sbox, rem) = SuperBox::from_source(jumbf.as_slice()).unwrap();
        assert!(rem.is_empty());

        let claim_dbox = sbox
            .find_by_label("cb.adobe_1/c2pa.claim")
            .unwrap()
            .data_box()
            .unwrap();

        let sig_sbox = sbox
            .find_by_label("cb.adobe_1")
            .unwrap()
            .child_boxes
            .get(2)
            .unwrap();

        assert!(sig_sbox.as_data_box().is_none());

        let sig_sbox = sig_sbox.as_super_box().unwrap();
        assert!(claim_dbox.offset_within_superbox(sig_sbox).is_none());
    }
}
