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

use crate::BoxType;

#[test]
fn impl_debug() {
    let x = BoxType([1, 2, 3, 4]);
    assert_eq!(format!("{x:#?}"), "[0x01, 0x02, 0x03, 0x04]");

    let x = BoxType(*b"abcd");
    assert_eq!(format!("{x:#?}"), "b\"abcd\"");

    let x = BoxType([b'a', b'b', b'c', 0x7f]);
    assert_eq!(format!("{x:#?}"), "[0x61, 0x62, 0x63, 0x7f]");
}
