// Descriptor wallet library extending bitcoin & miniscript functionality
// by LNP/BP Association (https://lnp-bp.org)
// Written in 2020-2022 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the Apache-2.0 License
// along with this software.
// If not, see <https://opensource.org/licenses/Apache-2.0>.

// In the future this mod will probably become part of bitcoin library

// Coding conventions
#![recursion_limit = "256"]
#![deny(dead_code, /* missing_docs, */ warnings)]
#![allow(clippy::init_numbered_fields)]

//! General workflow for working with ScriptPubkey* types:
//! ```text
//! Template -> Descriptor -> Structure -> PubkeyScript -> TxOut
//!
//! TxOut -> PubkeyScript -> Descriptor -> Structure -> Format
//! ```

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate strict_encoding;
#[cfg(feature = "miniscript")]
extern crate miniscript_crate as miniscript;

mod descriptor;
mod input;
pub mod locks;
#[cfg(feature = "miniscript")]
mod templates;
mod tweaks;

pub use descriptor::{
    BareDescriptor, CompositeDescrType, DescrVariants, Error, InnerDescrType, OuterDescrType,
    ParseError, ScriptPubkeyDescr, SpkClass, UnsupportedScriptPubkey,
};
pub use input::InputDescriptor;
#[cfg(feature = "miniscript")]
pub use templates::ScriptTemplate;
pub use tweaks::{OutputTweak, PubkeyTweak, ScriptTweak, TapretTweak, Tret};
