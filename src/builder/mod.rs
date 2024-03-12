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

//! An interface for building [JUMBF (ISO/IEC 19566-5:2019)]
//! data structures.
//!
//! [JUMBF (ISO/IEC 19566-5:2019)]: (https://www.iso.org/standard/73604.html)

mod data_box_builder;
mod placeholder_data_box;
mod super_box_builder;
pub(crate) mod to_box;

pub use data_box_builder::DataBoxBuilder;
pub use placeholder_data_box::PlaceholderDataBox;
pub use super_box_builder::SuperBoxBuilder;
pub use to_box::{ToBox, WriteAndSeek};
