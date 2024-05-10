//! Conversion specific utilities

use zcash_client_backend::address::Address;
use zcash_primitives::transaction::{components::amount::NonNegativeAmount, TxId};
use zingoconfig::ChainType;

use super::error::ConversionError;

/// Converts txid from hex-encoded `&str` to `zcash_primitives::transaction::TxId`.
///
/// TxId byte order is displayed in the reverse order to how it's encoded.
pub fn txid_from_hex_encoded_str(txid: &str) -> Result<TxId, ConversionError> {
    let txid_bytes = hex::decode(txid).unwrap();
    let mut txid_bytes = <[u8; 32]>::try_from(txid_bytes).unwrap();
    txid_bytes.reverse();
    Ok(TxId::from_bytes(txid_bytes))
}

pub(crate) fn address_from_str(
    address: &str,
    chain: &ChainType,
) -> Result<Address, ConversionError> {
    Address::decode(chain, address)
        .ok_or_else(|| ConversionError::InvalidAddress(address.to_string()))
}

pub(crate) fn zatoshis_from_u64(amount: u64) -> Result<NonNegativeAmount, ConversionError> {
    NonNegativeAmount::from_u64(amount).map_err(|_e| ConversionError::OutsideValidRange)
}

/// Conversions for use in testing only
#[cfg(feature = "test-features")]
pub mod testing {
    use zingoconfig::ChainType;

    use crate::data::receivers::Receivers;

    use super::{address_from_str, zatoshis_from_u64};

    /// Converts primitive rust types to zcash receiver types for constructing transaction requests.
    ///
    /// # Panics
    ///
    /// Panics if the address, amount or memo conversion fails.
    pub fn receivers_from_send_inputs(
        address_amount_memo_tuples: Vec<(&str, u64, Option<&str>)>,
        chain: &ChainType,
    ) -> Receivers {
        address_amount_memo_tuples
            .into_iter()
            .map(|(address, amount, memo)| {
                let address = address_from_str(address, chain).expect("should be a valid address");
                let amount = zatoshis_from_u64(amount)
                    .expect("should be inside the range of valid zatoshis");
                let memo = memo.map(|memo| {
                    crate::wallet::utils::interpret_memo_string(memo.to_string())
                        .expect("should be able to interpret memo")
                });

                (address, amount, memo)
            })
            .collect()
    }
}