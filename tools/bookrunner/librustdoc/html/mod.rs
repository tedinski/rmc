// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Modifications Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// See GitHub history for details.
crate mod escape;
crate mod format;
crate mod highlight;
crate mod layout;
mod length_limit;
// used by the error-index generator, so it needs to be public
pub mod markdown;
crate mod render;
crate mod sources;
crate mod static_files;
crate mod toc;
mod url_parts_builder;
