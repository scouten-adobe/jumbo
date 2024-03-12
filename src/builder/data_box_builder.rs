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

use std::{borrow::Cow, io::Result};

use crate::{
    builder::{ToBox, WriteAndSeek},
    BoxType,
};

/// A `DataBoxBuilder` allows you to build a simple JUMBF box containing
/// arbitrary binary data.
///
/// A box is defined as a four-byte data type and a binary data payload
/// of any size. The contents of the payload will vary depending on the
/// data type.
///
/// This struct does not ascribe any meaning to the type field or the
/// contents of the payload. These are generally application-specific.
///
/// Typically a `DataBoxBuilder` is added to a [`SuperBoxBuilder`] to generate
/// a larger JUMBF data structure.
///
/// [`SuperBoxBuilder`]: crate::builder::SuperBoxBuilder
pub struct DataBoxBuilder<'a> {
    tbox: BoxType,
    data: Cow<'a, [u8]>,
}

impl<'a> DataBoxBuilder<'a> {
    /// Create a `DataBoxBuilder` from a JUMBF box type and a borrowed byte
    /// slice.
    ///
    /// The byte slice must live as long as the `DataBoxBuilder` struct.
    pub fn from_borrowed(tbox: BoxType, data: &'a [u8]) -> Self {
        Self {
            tbox,
            data: Cow::from(data),
        }
    }

    /// Create a `DataBoxBuilder` from a JUMBF box type and a byte vector.
    ///
    /// Takes ownership of the byte vector.
    pub fn from_owned(tbox: BoxType, data: Vec<u8>) -> Self {
        Self {
            tbox,
            data: Cow::from(data),
        }
    }
}

impl<'a> ToBox for DataBoxBuilder<'a> {
    fn box_type(&self) -> BoxType {
        self.tbox
    }

    fn payload_size(&self) -> Result<usize> {
        Ok(self.data.len())
    }

    fn write_payload(&self, to_stream: &mut dyn WriteAndSeek) -> Result<()> {
        to_stream.write_all(&self.data)
    }
}
