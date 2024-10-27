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

use std::fmt::{Debug, Formatter};

use nom::{
    number::complete::{be_u32, be_u64},
    Needed,
};

use crate::{
    debug::*,
    parser::{Error, ParseResult},
    BoxType,
};

/// Represents a single JUMBF box.
///
/// This is referred to here as a "data box" since it is intended to house
/// application-specific data. This crate does not ascribe any meaning to the
/// type field or the contents of this box.
///
/// A box is defined as a four-byte data type and a byte-slice payload
/// of any size. The contents of the payload will vary depending on the
/// data type.
#[derive(Clone, Eq, PartialEq)]
pub struct DataBox<'a> {
    /// Box type.
    ///
    /// This field specifies the type of information found in the `data`
    /// field. The value of this field is encoded as a 4-byte big-endian
    /// unsigned integer. However, boxes are generally referred to by an
    /// ISO/IEC 646 character string translation of the integer value.
    ///
    /// For that reason, this is represented here as a 4-byte slice.
    ///
    /// The box type can typically be matched with a byte string constant (i.e.
    /// `b"jumd"`).
    pub tbox: BoxType,

    /// Box contents.
    ///
    /// This field contains the actual information contained within this box.
    /// The format of the box contents depends on the box type and will be
    /// defined individually for each type.
    pub data: &'a [u8],

    /// Original box data.
    ///
    /// This the original byte slice that was parsed to create this box.
    /// It is preserved in case a future client wishes to re-serialize this
    /// box as is.
    pub original: &'a [u8],
}

impl<'a> DataBox<'a> {
    /// Parse a JUMBF box, and return a tuple of the remainder of the input and
    /// the parsed box.
    ///
    /// The returned object uses zero-copy, and so has the same lifetime as the
    /// input.
    pub fn from_slice(source: &'a [u8]) -> ParseResult<'a, Self> {
        let (i, len) = be_u32(source)?;

        let (i, tbox): (&'a [u8], BoxType) = if i.len() >= 4 {
            let (tbox, i) = i.split_at(4);
            (i, tbox.into())
        } else {
            return Err(nom::Err::Error(Error::Incomplete(Needed::new(4))));
        };

        let (i, len, original_len) = match len {
            0 => (i, i.len(), source.len()),
            1 => {
                let (i, len) = be_u64(i)?;
                if len >= 16 {
                    (i, len as usize - 16, len as usize)
                } else {
                    return Err(nom::Err::Error(Error::InvalidBoxLength(len as u32)));
                }
            }
            2..=7 => {
                return Err(nom::Err::Error(Error::InvalidBoxLength(len)));
            }
            len => (i, len as usize - 8, len as usize),
        };

        if i.len() >= len {
            let (data, i) = i.split_at(len);
            Ok((
                i,
                Self {
                    tbox,
                    data,
                    original: &source[0..original_len],
                },
            ))
        } else {
            Err(nom::Err::Error(Error::Incomplete(Needed::new(len))))
        }
    }
}

impl<'a> Debug for DataBox<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("DataBox")
            .field("tbox", &self.tbox)
            .field("data", &DebugByteSlice(self.data))
            .field("original", &DebugByteSlice(self.original))
            .finish()
    }
}
