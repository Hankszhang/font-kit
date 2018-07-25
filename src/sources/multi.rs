// font-kit/src/sources/multi.rs
//
// Copyright © 2018 The Pathfinder Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A source that encapsulates multiple sources and allows them to be queried as a group.
//!
//! This is useful when an application wants a library of fonts consisting of the installed system
//! fonts plus some other application-supplied fonts.

use descriptor::Class;
use error::SelectionError;
use family_handle::FamilyHandle;
use handle::Handle;
use properties::Properties;
use source::Source;

/// A source that encapsulates multiple sources and allows them to be queried as a group.
///
/// This is useful when an application wants a library of fonts consisting of the installed system
/// fonts plus some other application-supplied fonts.
pub struct MultiSource {
    subsources: Vec<Box<Source>>,
}

impl MultiSource {
    pub fn from_sources(subsources: Vec<Box<Source>>) -> MultiSource {
        MultiSource {
            subsources,
        }
    }

    pub fn all_families(&self) -> Result<Vec<String>, SelectionError> {
        let mut families = vec![];
        for subsource in &self.subsources {
            families.extend(try!(subsource.all_families()).into_iter())
        }
        Ok(families)
    }

    pub fn select_family_by_name(&self, family_name: &str)
                                 -> Result<FamilyHandle, SelectionError> {
        for subsource in &self.subsources {
            match subsource.select_family_by_name(family_name) {
                Ok(family) => return Ok(family),
                Err(SelectionError::NotFound) => {}
                Err(err) => return Err(err),
            }
        }
        Err(SelectionError::NotFound)
    }

    pub fn select_by_postscript_name(&self, postscript_name: &str)
                                     -> Result<Handle, SelectionError> {
        for subsource in &self.subsources {
            match subsource.select_by_postscript_name(postscript_name) {
                Ok(font) => return Ok(font),
                Err(SelectionError::NotFound) => {}
                Err(err) => return Err(err),
            }
        }
        Err(SelectionError::NotFound)
    }

    #[inline]
    pub fn select_best_match(&self, classes: &[Class], properties: &Properties)
                             -> Result<Handle, SelectionError> {
        <Self as Source>::select_best_match(self, classes, properties)
    }
}

impl Source for MultiSource {
    #[inline]
    fn all_families(&self) -> Result<Vec<String>, SelectionError> {
        self.all_families()
    }

    #[inline]
    fn select_family_by_name(&self, family_name: &str) -> Result<FamilyHandle, SelectionError> {
        self.select_family_by_name(family_name)
    }

    #[inline]
    fn select_by_postscript_name(&self, postscript_name: &str) -> Result<Handle, SelectionError> {
        self.select_by_postscript_name(postscript_name)
    }
}
