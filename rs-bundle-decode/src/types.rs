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
    use angstrom_types::contract_payloads::{
        Asset, Pair, Signature,
        angstrom::{
            AngstromBundle, OrderQuantities, StandingValidation, TopOfBlockOrder, UserOrder,
        },
        rewards::{PoolUpdate, RewardsUpdate},
    };
    use pade::{PadeDecode, PadeEncode};

    impl From<OrderQuantities> for super::OrderQuantities {
        fn from(value: OrderQuantities) -> Self {
            match value {
                OrderQuantities::Exact { quantity } => Self::Exact { quantity },
                OrderQuantities::Partial {
                    min_quantity_in,
                    max_quantity_in,
                    filled_quantity,
                } => Self::Partial {
                    min_quantity_in,
                    max_quantity_in,
                    filled_quantity,
                },
            }
        }
    }

    impl From<Signature> for super::Signature {
        fn from(value: Signature) -> Self {
            match value {
                Signature::Contract { from, signature } => Self::Contract {
                    from: from.0.0,
                    signature: signature.into(),
                },
                Signature::Ecdsa { v, r, s } => Self::Ecdsa { v, r: r.0, s: s.0 },
            }
        }
    }

    impl From<StandingValidation> for super::StandingValidation {
        fn from(value: StandingValidation) -> Self {
            Self {
                nonce: value.nonce(),
                deadline: value.deadline(),
            }
        }
    }

    impl From<UserOrder> for super::UserOrder {
        fn from(value: UserOrder) -> Self {
            Self {
                ref_id: value.ref_id,
                use_internal: value.use_internal,
                pair_index: value.pair_index,
                min_price: value.min_price.to_be_bytes(),
                recipient: value.recipient.map(Into::into),
                hook_data: value.hook_data.map(Into::into),
                zero_for_one: value.zero_for_one,
                standing_validation: value.standing_validation.map(Into::into),
                order_quantities: value.order_quantities.into(),
                max_extra_fee_asset0: value.max_extra_fee_asset0,
                extra_fee_asset0: value.extra_fee_asset0,
                exact_in: value.exact_in,
                signature: value.signature.into(),
            }
        }
    }

    impl From<TopOfBlockOrder> for super::TopOfBlockOrder {
        fn from(value: TopOfBlockOrder) -> Self {
            Self {
                use_internal: value.use_internal,
                recipient: value.recipient.map(Into::into),
                signature: value.signature.into(),
                quantity_in: value.quantity_in,
                quantity_out: value.quantity_out,
                max_gas_asset_0: value.max_gas_asset_0,
                gas_used_asset_0: value.gas_used_asset_0,
                pairs_index: value.pairs_index,
                zero_for_1: value.zero_for_1,
            }
        }
    }

    impl From<RewardsUpdate> for super::RewardsUpdate {
        fn from(value: RewardsUpdate) -> Self {
            match value {
                RewardsUpdate::MultiTick {
                    start_tick,
                    start_liquidity,
                    quantities,
                    reward_checksum,
                } => Self::MultiTick {
                    start_tick: start_tick.to_be_bytes(),
                    start_liquidity,
                    quantities,
                    reward_checksum: reward_checksum.to_be_bytes(),
                },
                RewardsUpdate::CurrentOnly {
                    amount,
                    expected_liquidity,
                } => Self::CurrentOnly {
                    amount,
                    expected_liquidity,
                },
            }
        }
    }

    impl From<Asset> for super::Asset {
        fn from(value: Asset) -> Self {
            Self {
                addr: value.addr.0.0,
                save: value.save,
                take: value.take,
                settle: value.settle,
            }
        }
    }

    impl From<Pair> for super::Pair {
        fn from(value: Pair) -> Self {
            Self {
                index0: value.index0,
                index1: value.index1,
                store_index: value.store_index,
                price_1over0: value.price_1over0.to_be_bytes(),
            }
        }
    }

    impl From<PoolUpdate> for super::PoolUpdate {
        fn from(value: PoolUpdate) -> Self {
            Self {
                zero_for_one: value.zero_for_one,
                pair_index: value.pair_index,
                swap_in_quantity: value.swap_in_quantity,
                rewards_update: value.rewards_update.into(),
            }
        }
    }

    impl From<AngstromBundle> for super::AngstromBundle {
        fn from(value: AngstromBundle) -> Self {
            Self {
                assets: value.assets.into_iter().map(Into::into).collect(),
                pairs: value.pairs.into_iter().map(Into::into).collect(),
                pool_updates: value.pool_updates.into_iter().map(Into::into).collect(),
                top_of_block_orders: value
                    .top_of_block_orders
                    .into_iter()
                    .map(Into::into)
                    .collect(),
                user_orders: value.user_orders.into_iter().map(Into::into).collect(),
            }
        }
    }

    #[test]
    fn test_bundle_matches() {
        let test_bundle_bytes_ext = bytes!(
            "0x09c5eabe00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000174000088a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000001dd7e000000000000000000000000013dcd7600000000000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000000000000000000000000000001bed63d818030b00002600000001000000000000000000000000000000000000000000000000000000000000000000000000330200000000000000000000001bed63d818030b0000000000000000000000000000000a0000000000000000000013806b76267d000084080000000000000000001bed63d818030b000000000000000000000000013dcd6c0000000000000000000000000003bafc0000000000000000000000000001dd7e00001ce7254b94c5bea4da8303c64c16dd07106247ca887e452f283f4a69d53c304a453f213d526e766320c829d2189748ab1a4ddd8c3d0ce12e96723b64ddae6fe6e8000000000000000000000000000000"
        );

        let test_bundle_bytes = Bytes::from_iter(&test_bundle_bytes_ext[(4 + 32 + 32)..]);

        let real =
            AngstromBundle::pade_decode(&mut test_bundle_bytes.to_vec().as_slice(), None).unwrap();

        println!("{real:?}");

        let real_encoded = real.pade_encode();

        let mine =
            super::AngstromBundle::pade_decode(&mut test_bundle_bytes.to_vec().as_slice(), None)
                .unwrap();
        let mine_encoded = mine.pade_encode();

        assert_eq!(mine, real.into());
        assert_eq!(real_encoded, mine_encoded);
    }
}
