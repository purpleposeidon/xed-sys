/*
 * Copyright 2017 William Cody Laeder
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */


//! Intel XED Bindings
//!
//! Just as an FYI you'll need to call `xed_tables_init()` before
//! using any of the internal functions.
//!
//! This library doesn't play well with threads.
//!
//! For (real) docs see: https://software.intel.com/sites/landingpage/xed/ref-manual/html/index.html
#![allow(
    non_snake_case,
    dead_code,
    non_camel_case_types,
    const_err,
    improper_ctypes,
    path_statements,
    unused_parens,
    unused_unsafe,
    no_mangle_const_items,
    private_no_mangle_fns,
    non_upper_case_globals,
    unreachable_code)]

pub mod xed_interface;
pub mod xed_version;
