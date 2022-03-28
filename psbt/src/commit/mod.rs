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

//! Managing commitment-related proprietary keys inside PSBT.
//! Supports [`tapret`], [`p2c`] and [`s2c`] commitments.

pub mod p2c;
pub mod tapret;
pub mod s2c {
    #![allow(missing_docs)]
    // TODO: Implement
}

pub use p2c::{P2cOutput, PSBT_IN_P2C_TWEAK, PSBT_P2C_PREFIX};
pub use tapret::{
    TapretOutput, PSBT_OUT_TAPRET_COMMITMENT, PSBT_OUT_TAPRET_HOST, PSBT_OUT_TAPRET_PROOF,
    PSBT_TAPRET_PREFIX,
};
