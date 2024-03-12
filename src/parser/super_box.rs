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

use crate::{
    box_type::SUPER_BOX_TYPE,
    debug::*,
    parser::{Box, DescriptionBox, Error, ParseResult},
};

/// A JUMBF superbox contains a description box and zero or more
/// data boxes, each of which may or may not be a superbox.
#[derive(Clone, Eq, PartialEq)]
pub struct SuperBox<'a> {
    /// Description box.
    pub desc: DescriptionBox<'a>,

    /// Child boxes. (These are referred to in some documentation
    /// as "data boxes.")
    pub child_boxes: Vec<ChildBox<'a>>,

    /// Original box data.
    ///
    /// This the original byte slice that was parsed to create this box.
    /// It is preserved in case a future client wishes to re-serialize this
    /// box as is.
    pub original: &'a [u8],
}

impl<'a> SuperBox<'a> {
    /// Parse a byte-slice as a JUMBF superbox, and return a tuple of the
    /// remainder of the input and the parsed super box.
    ///
    /// Children of this superbox which are also superboxes will be parsed
    /// recursively.
    ///
    /// The returned object uses zero-copy, and so has the same lifetime as the
    /// input.
    pub fn from_slice(i: &'a [u8]) -> ParseResult<Self> {
        let (i, boxx): (&'a [u8], Box<'a>) = Box::from_slice(i)?;
        let (_, sbox) = Self::from_box(boxx)?;
        Ok((i, sbox))
    }

    /// Convert an existing JUMBF box to a JUMBF superbox.
    ///
    /// This consumes the existing [`Box`] object and will return an appropriate
    /// error if the box doesn't match the expected syntax for a superbox.
    ///
    /// Returns a tuple of the remainder of the input from the box (which should
    /// typically be empty) and the new [`SuperBox`] object.
    pub fn from_box(boxx: crate::parser::Box<'a>) -> ParseResult<'a, Self> {
        if boxx.tbox != SUPER_BOX_TYPE {
            return Err(nom::Err::Error(Error::InvalidSuperBoxType(boxx.tbox)));
        }

        let (i, desc) = DescriptionBox::from_slice(boxx.data)?;

        let (i, child_boxes) = boxes_from_slice(i)?;
        let child_boxes = child_boxes
            .into_iter()
            .map(|d| {
                if d.tbox == SUPER_BOX_TYPE {
                    let (_, sbox) = Self::from_box(d)?;
                    Ok(ChildBox::SuperBox(sbox))
                } else {
                    Ok(ChildBox::DataBox(d))
                }
            })
            .collect::<Result<Vec<ChildBox<'a>>, Error>>()?;

        Ok((
            i,
            Self {
                desc,
                child_boxes,
                original: boxx.original,
            },
        ))
    }

    /// Find a child superbox of this superbox by label and verify that
    /// exactly one such child exists.
    ///
    /// If label contains one or more slash (`/`) characters, the label
    /// will be treated as a hierarchical label and this function can then
    /// be used to traverse nested data structures.
    ///
    /// Will return `None` if no matching child superbox is found _or_ if
    /// more than one matching child superbox is found.
    pub fn find_by_label(&self, label: &str) -> Option<&Self> {
        let (label, suffix) = match label.split_once('/') {
            Some((label, suffix)) => (label, Some(suffix)),
            None => (label, None),
        };

        let matching_children: Vec<&SuperBox> = self
            .child_boxes
            .iter()
            .filter_map(|boxx| match boxx {
                ChildBox::SuperBox(sbox) => {
                    if let Some(sbox_label) = sbox.desc.label {
                        if sbox_label == label && sbox.desc.requestable {
                            Some(sbox)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        if let Some(sbox) = matching_children.first() {
            if matching_children.len() == 1 {
                if let Some(suffix) = suffix {
                    return sbox.find_by_label(suffix);
                } else {
                    Some(sbox)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// If the first child box of this superbox is a data box, return it.
    /// Otherwise, return `None`.
    ///
    /// This is a convenience function for the common case where the superbox
    /// contains a non-superbox payload that needs to be interpreted further.
    pub fn data_box(&'a self) -> Option<&'a Box<'a>> {
        self.child_boxes.first().and_then(|boxx| match boxx {
            ChildBox::DataBox(boxx) => Some(boxx),
            _ => None,
        })
    }
}

impl<'a> Debug for SuperBox<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("SuperBox")
            .field("desc", &self.desc)
            .field("child_boxes", &self.child_boxes)
            .field("original", &DebugByteSlice(self.original))
            .finish()
    }
}

// Parse boxes from slice until slice is empty.
fn boxes_from_slice(i: &[u8]) -> ParseResult<Vec<Box<'_>>> {
    let mut result: Vec<Box> = vec![];
    let mut i = i;

    while !i.is_empty() {
        let (x, boxx) = Box::from_slice(i)?;
        i = x;
        result.push(boxx);
    }

    Ok((i, result))
}

/// This type represents a single box within a superbox,
/// which may itself be a superbox or or a regular box.
///
/// Note that this crate doesn't parse the content or ascribe
/// meaning to any type of box other than superbox (`jumb`) or
/// description box (`jumd`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChildBox<'a> {
    /// A superbox.
    SuperBox(SuperBox<'a>),

    /// Any other kind of box.
    DataBox(crate::parser::Box<'a>),
}
