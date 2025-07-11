use pade_macro::{PadeDecode, PadeEncode};
use serde::Serialize;

type Address = [u8; 20];
type U256 = [u8; 32];
type B256 = [u8; 32];
type U160 = [u8; 20];
type I24 = [u8; 3];

#[derive(Debug, PadeDecode, PadeEncode, Serialize, PartialEq)]
pub struct AngstromBundle {
    pub assets: Vec<Asset>,
    pub pairs: Vec<Pair>,
    pub pool_updates: Vec<PoolUpdate>,
    pub top_of_block_orders: Vec<TopOfBlockOrder>,
    pub user_orders: Vec<UserOrder>,
}

#[derive(Debug, PadeDecode, PadeEncode, Serialize, PartialEq)]
pub struct Asset {
    addr: Address,
    save: u128,
    take: u128,
    settle: u128,
}

#[derive(Debug, PadeDecode, PadeEncode, Serialize, PartialEq)]
pub struct Pair {
    index0: u16,
    index1: u16,
    store_index: u16,
    price_1over0: [u8; 32],
}

#[derive(Debug, PadeDecode, PadeEncode, Serialize, PartialEq)]
pub struct PoolUpdate {
    pub zero_for_one: bool,
    pub pair_index: u16,
    pub swap_in_quantity: u128,
    pub rewards_update: RewardsUpdate,
}

#[derive(Debug, PadeDecode, PadeEncode, Serialize, PartialEq)]
pub enum RewardsUpdate {
    MultiTick {
        start_tick: I24,
        start_liquidity: u128,
        quantities: Vec<u128>,
        reward_checksum: U160,
    },
    CurrentOnly {
        amount: u128,
        expected_liquidity: u128,
    },
}

#[derive(Debug, PadeDecode, PadeEncode, Serialize, PartialEq)]
pub struct TopOfBlockOrder {
    pub use_internal: bool,
    pub quantity_in: u128,
    pub quantity_out: u128,
    pub max_gas_asset_0: u128,
    pub gas_used_asset_0: u128,
    pub pairs_index: u16,
    pub zero_for_1: bool,
    pub recipient: Option<Address>,
    pub signature: Signature,
}

#[derive(Debug, PadeDecode, PadeEncode, Serialize, PartialEq)]
pub struct UserOrder {
    pub ref_id: u32,
    pub use_internal: bool,
    pub pair_index: u16,
    pub min_price: U256,
    pub recipient: Option<Address>,
    pub hook_data: Option<Vec<u8>>,
    pub zero_for_one: bool,
    pub standing_validation: Option<StandingValidation>,
    pub order_quantities: OrderQuantities,
    pub max_extra_fee_asset0: u128,
    pub extra_fee_asset0: u128,
    pub exact_in: bool,
    pub signature: Signature,
}

#[derive(Debug, PadeDecode, PadeEncode, Serialize, PartialEq)]
pub struct StandingValidation {
    nonce: u64,
    #[pade_width(5)]
    deadline: u64,
}

#[derive(Debug, PadeDecode, PadeEncode, Serialize, PartialEq)]
pub enum Signature {
    Contract { from: Address, signature: Vec<u8> },
    Ecdsa { v: u8, r: B256, s: B256 },
}

#[derive(Debug, PadeDecode, PadeEncode, Serialize, PartialEq)]
pub enum OrderQuantities {
    Exact {
        quantity: u128,
    },
    Partial {
        min_quantity_in: u128,
        max_quantity_in: u128,
        filled_quantity: u128,
    },
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{Bytes, bytes};

    use angstrom_types::contract_payloads::angstrom::AngstromBundle;
    use pade::{PadeDecode, PadeEncode};

    #[test]
    fn test_bundle_matches() {
        let test_bundle_bytes_ext = bytes!(
            "0x09c5eabe00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000174000088a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000001dd7e000000000000000000000000013dcd7600000000000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000000000000000000000000000001bed63d818030b00002600000001000000000000000000000000000000000000000000000000000000000000000000000000330200000000000000000000001bed63d818030b0000000000000000000000000000000a0000000000000000000013806b76267d000084080000000000000000001bed63d818030b000000000000000000000000013dcd6c0000000000000000000000000003bafc0000000000000000000000000001dd7e00001ce7254b94c5bea4da8303c64c16dd07106247ca887e452f283f4a69d53c304a453f213d526e766320c829d2189748ab1a4ddd8c3d0ce12e96723b64ddae6fe6e8000000000000000000000000000000"
        );

        let test_bundle_bytes = Bytes::from_iter(&test_bundle_bytes_ext[(4 + 32 + 32)..]);

        let real =
            AngstromBundle::pade_decode(&mut test_bundle_bytes.to_vec().as_slice(), None).unwrap();

        let mine =
            super::AngstromBundle::pade_decode(&mut test_bundle_bytes.to_vec().as_slice(), None)
                .unwrap();

        assert_eq!(real.pade_encode(), mine.pade_encode());
    }
}
