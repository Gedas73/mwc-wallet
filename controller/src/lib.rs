// Copyright 2019 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Library module for the main wallet functionalities provided by Grin.

#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate gotham_derive;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
use failure;
use grin_wallet_api as apiwallet;
use grin_wallet_config as config;
use grin_wallet_impls as impls;
use grin_wallet_libwallet as libwallet;
use grin_wallet_util::grin_api as api;
use grin_wallet_util::grin_core as core;
use grin_wallet_util::grin_keychain as keychain;
use grin_wallet_util::grin_util as util;

pub mod command;
mod common;
mod contacts;
pub mod controller;
pub mod display;
mod error;
mod executor;
mod mwcmq;
mod tx_proof;

pub use crate::error::{Error, ErrorKind};
