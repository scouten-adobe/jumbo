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

//! Toggles in a JUMBF description box describe which additional
//! information is present in the description

/// Toggle bit 0 (0x01) indicates that this superbox can be requested
/// via URI requests.
pub(crate) const REQUESTABLE: u8 = 0x01;

/// Toggle bit 1 (0x02) indicates that the label has an optional textual label.
pub(crate) const HAS_LABEL: u8 = 0x02;

/// Toggle bit 2 (0x04) indicates that the label has an optional
/// application-specific 32-bit identifier.
pub(crate) const HAS_ID: u8 = 0x04;

/// Toggle bit 3 (0x08) indicates that a SHA-256 hash of the superbox's
/// data box is present.
pub(crate) const HAS_HASH: u8 = 0x08;

/// Toggle bit 4 (0x10) indicates that an application-specific "private"
/// box is contained within the description box.
pub(crate) const HAS_PRIVATE_BOX: u8 = 0x10;
