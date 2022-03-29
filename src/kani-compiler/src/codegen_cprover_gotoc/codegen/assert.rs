// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This file contains the code that acts as a wrapper to create the new assert and related statements

use crate::codegen_cprover_gotoc::GotocCtx;
use cbmc::goto_program::{Expr, Location, Stmt};
use std::str::FromStr;

/// The Property Class enum stores all viable options for classifying asserts, cover assume and other related statements
#[derive(Copy, Debug, Clone)]
pub enum PropertyClass {
    ExpectFail,
    UnsupportedStructs,
    DefaultAssertion,
}

impl FromStr for PropertyClass {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<PropertyClass, Self::Err> {
        match input {
            "expect_fail" => Ok(PropertyClass::ExpectFail),
            "unsupported_struct" => Ok(PropertyClass::UnsupportedStructs),
            "assertion" => Ok(PropertyClass::DefaultAssertion),
            _ => Err("No such property class"),
        }
    }
}

impl PropertyClass {
    pub fn as_str(&self) -> &str {
        match self {
            PropertyClass::ExpectFail => "expect_fail",
            PropertyClass::UnsupportedStructs => "unsupported_struct",
            PropertyClass::DefaultAssertion => "assertion",
        }
    }
}

impl<'tcx> GotocCtx<'tcx> {
    pub fn codegen_assert(
        &self,
        cond: Expr,
        property_class: PropertyClass,
        message: &str,
        loc: Location,
    ) -> Stmt {
        assert!(cond.typ().is_bool());

        let property_name = property_class.as_str();

        // Create a Property Location Variant from any given Location type
        let property_location =
            Location::create_location_with_property(message, property_name, loc);

        Stmt::assert_statement(cond, property_name, message, property_location)
    }
}
