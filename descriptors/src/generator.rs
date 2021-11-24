// Descriptor wallet library extending bitcoin & miniscript functionality
// by LNP/BP Association (https://lnp-bp.org)
// Written in 2020-2021 by
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

use std::collections::HashMap;
use std::str::FromStr;

use amplify::Wrapper;
use bitcoin::secp256k1::{Secp256k1, Verification};
use bitcoin::Script;
use bitcoin_hd::UnhardenedIndex;
use bitcoin_scripts::{Category, PubkeyScript};
#[cfg(feature = "serde")]
use serde_with::{As, DisplayFromStr};

use super::{DeriveLockScript, Error, Expanded, Template, Variants};

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[derive(
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Debug,
    Display,
    StrictEncode,
    StrictDecode
)]
#[display("{variants}<{template}>")]
pub struct Generator {
    pub template: Template,

    #[cfg_attr(feature = "serde", serde(with = "As::<DisplayFromStr>"))]
    pub variants: Variants,
}

/// Error parsing descriptor generator: unrecognized string
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display, Error
)]
#[display(doc_comments)]
pub struct GeneratorParseError;

impl FromStr for Generator {
    type Err = GeneratorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim_end_matches('>').split('<');
        let me = Generator {
            variants: split
                .next()
                .ok_or(GeneratorParseError)?
                .parse()
                .map_err(|_| GeneratorParseError)?,
            template: split
                .next()
                .ok_or(GeneratorParseError)?
                .parse()
                .map_err(|_| GeneratorParseError)?,
        };
        if split.next().is_some() {
            Err(GeneratorParseError)
        } else {
            Ok(me)
        }
    }
}

impl Generator {
    pub fn descriptors<C: Verification>(
        &self,
        ctx: &Secp256k1<C>,
        index: UnhardenedIndex,
    ) -> Result<HashMap<Category, Expanded>, Error> {
        let mut descriptors = HashMap::with_capacity(5);
        let single = if let Template::SingleSig(_) = self.template {
            Some(
                self.template
                    .try_derive_public_key(ctx, index)
                    .expect("Can't fail"),
            )
        } else {
            None
        };
        if self.variants.bare {
            let d = if let Some(pk) = single {
                Expanded::Pk(pk)
            } else {
                Expanded::Bare(
                    self.template
                        .derive_lock_script(ctx, index, Category::Bare)?
                        .into_inner()
                        .into(),
                )
            };
            descriptors.insert(Category::Bare, d);
        }
        if self.variants.hashed {
            let d = if let Some(pk) = single {
                Expanded::Pkh(pk)
            } else {
                Expanded::Sh(
                    self.template
                        .derive_lock_script(ctx, index, Category::Hashed)?
                        .into(),
                )
            };
            descriptors.insert(Category::Hashed, d);
        }
        if self.variants.nested {
            let d = if let Some(pk) = single {
                Expanded::ShWpkh(pk)
            } else {
                Expanded::ShWsh(
                    self.template
                        .derive_lock_script(ctx, index, Category::Nested)?
                        .into(),
                )
            };
            descriptors.insert(Category::Nested, d);
        }
        if self.variants.segwit {
            let d = if let Some(pk) = single {
                Expanded::Wpkh(pk)
            } else {
                Expanded::Wsh(
                    self.template
                        .derive_lock_script(ctx, index, Category::SegWitV0)?
                        .into(),
                )
            };
            descriptors.insert(Category::SegWitV0, d);
        }
        /* TODO #15: Enable once Taproot will go live
        if self.variants.taproot {
            scripts.push(content.taproot());
        }
         */
        Ok(descriptors)
    }

    #[inline]
    pub fn pubkey_scripts<C: Verification>(
        &self,
        ctx: &Secp256k1<C>,
        index: UnhardenedIndex,
    ) -> Result<HashMap<Category, Script>, Error> {
        Ok(self
            .descriptors(ctx, index)?
            .into_iter()
            .map(|(cat, descr)| (cat, PubkeyScript::from(descr).into()))
            .collect())
    }
}
