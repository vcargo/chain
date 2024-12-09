pub use gns_pairs_storage::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod gns_pairs_storage {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addFees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_fees"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(104usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPairsStorage.FeeGroup[]",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addGroups"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addGroups"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_groups"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPairsStorage.Group[]",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addPairs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addPairs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairs"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                        ::ethers::core::abi::ethabi::ParamType::String,
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPairsStorage.Pair[]",),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("fees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(104usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPairsStorage.FeeGroup",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feesCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("feesCount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllPairsRestrictedMaxLeverage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllPairsRestrictedMaxLeverage",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getGlobalTradeFeeParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getGlobalTradeFeeParams",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(136usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IPairsStorage.GlobalTradeFeeParams",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getGroupLiquidationParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getGroupLiquidationParams",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_groupIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IPairsStorage.GroupLiquidationParams",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPairLiquidationParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPairLiquidationParams",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IPairsStorage.GroupLiquidationParams",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("groups"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("groups"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::String,
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPairsStorage.Group",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("groupsCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("groupsCount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initializeGroupLiquidationParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initializeGroupLiquidationParams",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_groupLiquidationParams",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    ],),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IPairsStorage.GroupLiquidationParams[]",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initializeNewFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initializeNewFees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_tradeFeeParams"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(136usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IPairsStorage.GlobalTradeFeeParams",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPairIndexListed"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isPairIndexListed"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPairListed"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isPairListed"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairCustomMaxLeverage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairCustomMaxLeverage",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairJob"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairJob"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairMaxLeverage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairMaxLeverage"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairMinFeeUsd"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairMinFeeUsd"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairMinLeverage"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairMinLeverage"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairMinPositionSizeUsd"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairMinPositionSizeUsd",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairOraclePositionSizeFeeP"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairOraclePositionSizeFeeP",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairSpreadP"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairSpreadP"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairTotalLiqCollateralFeeP"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairTotalLiqCollateralFeeP",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairTotalPositionSizeFeeP"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairTotalPositionSizeFeeP",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_pairIndex"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::String,
                                ::ethers::core::abi::ethabi::ParamType::String,
                                ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IPairsStorage.Pair",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairsCount"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairsCount"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setGlobalTradeFeeParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setGlobalTradeFeeParams",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_feeParams"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(136usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IPairsStorage.GlobalTradeFeeParams",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setGroupLiquidationParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setGroupLiquidationParams",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_groupIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IPairsStorage.GroupLiquidationParams",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPairCustomMaxLeverages"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setPairCustomMaxLeverages",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_indices"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_values"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateFees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_fees"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(104usize),
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IPairsStorage.FeeGroup[]",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateGroups"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updateGroups"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_groups"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                                32usize
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IPairsStorage.Group[]",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updatePairs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("updatePairs"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pairIndices"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_pairs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(
                                                        8usize
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(
                                                        256usize
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IPairsStorage.Pair[]",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AccessControlUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AccessControlUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("target"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("role"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("access"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressesUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AddressesUpdated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("addresses"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FeeAdded"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeGroup"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(104usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FeeUpdated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeGroup"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(104usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GlobalTradeFeeParamsUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("GlobalTradeFeeParamsUpdated",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("feeParams"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(136usize),
                            ],),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GroupAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("GroupAdded"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("name"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GroupLiquidationParamsUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("GroupLiquidationParamsUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(40usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GroupUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("GroupUpdated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialized"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("version"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PairAdded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PairAdded"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PairCustomMaxLeverageUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PairCustomMaxLeverageUpdated",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("maxLeverage"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PairUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("PairUpdated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("index"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AboveMax"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AboveMax"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AlreadyExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AlreadyExists"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BelowMin"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BelowMin"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BlockOrder"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BlockOrder"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DoesntExist"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DoesntExist"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EndLeverageTooHigh"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("EndLeverageTooHigh"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EndLiqThresholdTooLow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("EndLiqThresholdTooLow",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeNotListed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FeeNotListed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GroupNotListed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("GroupNotListed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InitError"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InitError"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientBalance",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAddresses"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidAddresses"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCollateralIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidCollateralIndex",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInputLength"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidInputLength"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MaxLiqSpreadPTooHigh"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MaxLiqSpreadPTooHigh",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotAllowed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotAllowed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotAuthorized"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotAuthorized"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Overflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Overflow"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PairAlreadyListed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PairAlreadyListed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PairNotListed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PairNotListed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Paused"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StartLeverageTooLow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("StartLeverageTooLow",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StartLiqThresholdTooHigh"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("StartLiqThresholdTooHigh",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsupportedChain"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UnsupportedChain"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongAccess"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongAccess"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongFees"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongFees"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongIndex"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongIndex"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongLength"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongLength"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongLeverages"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongLeverages"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongLiqParamsLeverages"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongLiqParamsLeverages",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongLiqParamsThresholds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongLiqParamsThresholds",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongOrder"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongOrder"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongOrderType"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongOrderType"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongParams"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongParams"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongTradeType"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongTradeType"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ZeroAddress"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroValue"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ZeroValue"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GNSPAIRSSTORAGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct GNSPairsStorage<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GNSPairsStorage<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GNSPairsStorage<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GNSPairsStorage<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GNSPairsStorage<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GNSPairsStorage))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GNSPairsStorage<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                GNSPAIRSSTORAGE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `addFees` (0x0e5c75af) function
        pub fn add_fees(
            &self,
            fees: ::std::vec::Vec<FeeGroup>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 92, 117, 175], fees)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addGroups` (0x60283cba) function
        pub fn add_groups(
            &self,
            groups: ::std::vec::Vec<Group>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 40, 60, 186], groups)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPairs` (0xdb7c3f9d) function
        pub fn add_pairs(
            &self,
            pairs: ::std::vec::Vec<Pair>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 124, 63, 157], pairs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fees` (0x4acc79ed) function
        pub fn fees(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, FeeGroup> {
            self.0
                .method_hash([74, 204, 121, 237], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feesCount` (0x658de48a) function
        pub fn fees_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([101, 141, 228, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllPairsRestrictedMaxLeverage` (0x678b3fb0) function
        pub fn get_all_pairs_restricted_max_leverage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([103, 139, 63, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGlobalTradeFeeParams` (0x48fc3ef3) function
        pub fn get_global_trade_fee_params(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, GlobalTradeFeeParams> {
            self.0
                .method_hash([72, 252, 62, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getGroupLiquidationParams` (0x3572929c) function
        pub fn get_group_liquidation_params(
            &self,
            group_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, GroupLiquidationParams> {
            self.0
                .method_hash([53, 114, 146, 156], group_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPairLiquidationParams` (0x6633ced6) function
        pub fn get_pair_liquidation_params(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, GroupLiquidationParams> {
            self.0
                .method_hash([102, 51, 206, 214], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `groups` (0x96324bd4) function
        pub fn groups(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Group> {
            self.0
                .method_hash([150, 50, 75, 212], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `groupsCount` (0x885e2750) function
        pub fn groups_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([136, 94, 39, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initializeGroupLiquidationParams` (0x85d4390e) function
        pub fn initialize_group_liquidation_params(
            &self,
            group_liquidation_params: ::std::vec::Vec<GroupLiquidationParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 212, 57, 14], group_liquidation_params)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initializeNewFees` (0xc85ac748) function
        pub fn initialize_new_fees(
            &self,
            trade_fee_params: GlobalTradeFeeParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 90, 199, 72], (trade_fee_params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPairIndexListed` (0x281b7ead) function
        pub fn is_pair_index_listed(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([40, 27, 126, 173], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPairListed` (0x1628bfeb) function
        pub fn is_pair_listed(
            &self,
            from: ::std::string::String,
            to: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([22, 40, 191, 235], (from, to))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairCustomMaxLeverage` (0x24a96865) function
        pub fn pair_custom_max_leverage(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([36, 169, 104, 101], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairJob` (0x302f81fc) function
        pub fn pair_job(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::string::String, ::std::string::String),
        > {
            self.0
                .method_hash([48, 47, 129, 252], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairMaxLeverage` (0x281b693c) function
        pub fn pair_max_leverage(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([40, 27, 105, 60], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairMinFeeUsd` (0x8078bfbe) function
        pub fn pair_min_fee_usd(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([128, 120, 191, 190], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairMinLeverage` (0x59a992d0) function
        pub fn pair_min_leverage(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([89, 169, 146, 208], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairMinPositionSizeUsd` (0x5e26ff4e) function
        pub fn pair_min_position_size_usd(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 38, 255, 78], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairOraclePositionSizeFeeP` (0x0cf34255) function
        pub fn pair_oracle_position_size_fee_p(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([12, 243, 66, 85], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairSpreadP` (0xa1d54e9b) function
        pub fn pair_spread_p(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([161, 213, 78, 155], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairTotalLiqCollateralFeeP` (0xfbf3f911) function
        pub fn pair_total_liq_collateral_fee_p(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([251, 243, 249, 17], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairTotalPositionSizeFeeP` (0xc927b4b6) function
        pub fn pair_total_position_size_fee_p(
            &self,
            pair_index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([201, 39, 180, 182], pair_index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairs` (0xb91ac788) function
        pub fn pairs(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Pair> {
            self.0
                .method_hash([185, 26, 199, 136], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairsCount` (0xb81b2b71) function
        pub fn pairs_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([184, 27, 43, 113], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGlobalTradeFeeParams` (0x5f57a073) function
        pub fn set_global_trade_fee_params(
            &self,
            fee_params: GlobalTradeFeeParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 87, 160, 115], (fee_params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setGroupLiquidationParams` (0xd0cb753e) function
        pub fn set_group_liquidation_params(
            &self,
            group_index: ::ethers::core::types::U256,
            params: GroupLiquidationParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 203, 117, 62], (group_index, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPairCustomMaxLeverages` (0xd79261fd) function
        pub fn set_pair_custom_max_leverages(
            &self,
            indices: ::std::vec::Vec<::ethers::core::types::U256>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 146, 97, 253], (indices, values))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateFees` (0x1cf71b9b) function
        pub fn update_fees(
            &self,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
            fees: ::std::vec::Vec<FeeGroup>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 247, 27, 155], (ids, fees))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateGroups` (0x11d79ef5) function
        pub fn update_groups(
            &self,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
            groups: ::std::vec::Vec<Group>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 215, 158, 245], (ids, groups))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePairs` (0x10efa5d5) function
        pub fn update_pairs(
            &self,
            pair_indices: ::std::vec::Vec<::ethers::core::types::U256>,
            pairs: ::std::vec::Vec<Pair>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 239, 165, 213], (pair_indices, pairs))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AccessControlUpdated` event
        pub fn access_control_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccessControlUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `AddressesUpdated` event
        pub fn addresses_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AddressesUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `FeeAdded` event
        pub fn fee_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeAddedFilter> {
            self.0.event()
        }
        ///Gets the contract's `FeeUpdated` event
        pub fn fee_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeeUpdatedFilter> {
            self.0.event()
        }
        ///Gets the contract's `GlobalTradeFeeParamsUpdated` event
        pub fn global_trade_fee_params_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GlobalTradeFeeParamsUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GroupAdded` event
        pub fn group_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GroupAddedFilter> {
            self.0.event()
        }
        ///Gets the contract's `GroupLiquidationParamsUpdated` event
        pub fn group_liquidation_params_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GroupLiquidationParamsUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `GroupUpdated` event
        pub fn group_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GroupUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PairAdded` event
        pub fn pair_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PairAddedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PairCustomMaxLeverageUpdated` event
        pub fn pair_custom_max_leverage_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PairCustomMaxLeverageUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PairUpdated` event
        pub fn pair_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PairUpdatedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GNSPairsStorageEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for GNSPairsStorage<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AboveMax` with signature `AboveMax()` and selector `0x0ad1e31b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "AboveMax", abi = "AboveMax()")]
    pub struct AboveMax;
    ///Custom Error type `AlreadyExists` with signature `AlreadyExists()` and selector `0x23369fa6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "AlreadyExists", abi = "AlreadyExists()")]
    pub struct AlreadyExists;
    ///Custom Error type `BelowMin` with signature `BelowMin()` and selector `0x10906acb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BelowMin", abi = "BelowMin()")]
    pub struct BelowMin;
    ///Custom Error type `BlockOrder` with signature `BlockOrder()` and selector `0xd325f874`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BlockOrder", abi = "BlockOrder()")]
    pub struct BlockOrder;
    ///Custom Error type `DoesntExist` with signature `DoesntExist()` and selector `0x80375d5a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "DoesntExist", abi = "DoesntExist()")]
    pub struct DoesntExist;
    ///Custom Error type `EndLeverageTooHigh` with signature `EndLeverageTooHigh()` and selector `0xcb115746`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "EndLeverageTooHigh", abi = "EndLeverageTooHigh()")]
    pub struct EndLeverageTooHigh;
    ///Custom Error type `EndLiqThresholdTooLow` with signature `EndLiqThresholdTooLow()` and selector `0x21b5730d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "EndLiqThresholdTooLow", abi = "EndLiqThresholdTooLow()")]
    pub struct EndLiqThresholdTooLow;
    ///Custom Error type `FeeNotListed` with signature `FeeNotListed()` and selector `0x9216fe22`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "FeeNotListed", abi = "FeeNotListed()")]
    pub struct FeeNotListed;
    ///Custom Error type `GroupNotListed` with signature `GroupNotListed()` and selector `0x94b81634`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "GroupNotListed", abi = "GroupNotListed()")]
    pub struct GroupNotListed;
    ///Custom Error type `InitError` with signature `InitError()` and selector `0xb071c0a4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InitError", abi = "InitError()")]
    pub struct InitError;
    ///Custom Error type `InsufficientBalance` with signature `InsufficientBalance()` and selector `0xf4d678b8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InsufficientBalance", abi = "InsufficientBalance()")]
    pub struct InsufficientBalance;
    ///Custom Error type `InvalidAddresses` with signature `InvalidAddresses()` and selector `0xca07b5fb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidAddresses", abi = "InvalidAddresses()")]
    pub struct InvalidAddresses;
    ///Custom Error type `InvalidCollateralIndex` with signature `InvalidCollateralIndex()` and selector `0x3efb8e60`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidCollateralIndex", abi = "InvalidCollateralIndex()")]
    pub struct InvalidCollateralIndex;
    ///Custom Error type `InvalidInputLength` with signature `InvalidInputLength()` and selector `0x7db491eb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidInputLength", abi = "InvalidInputLength()")]
    pub struct InvalidInputLength;
    ///Custom Error type `MaxLiqSpreadPTooHigh` with signature `MaxLiqSpreadPTooHigh()` and selector `0xcd1049ff`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MaxLiqSpreadPTooHigh", abi = "MaxLiqSpreadPTooHigh()")]
    pub struct MaxLiqSpreadPTooHigh;
    ///Custom Error type `NotAllowed` with signature `NotAllowed()` and selector `0x3d693ada`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotAllowed", abi = "NotAllowed()")]
    pub struct NotAllowed;
    ///Custom Error type `NotAuthorized` with signature `NotAuthorized()` and selector `0xea8e4eb5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotAuthorized", abi = "NotAuthorized()")]
    pub struct NotAuthorized;
    ///Custom Error type `Overflow` with signature `Overflow()` and selector `0x35278d12`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Overflow", abi = "Overflow()")]
    pub struct Overflow;
    ///Custom Error type `PairAlreadyListed` with signature `PairAlreadyListed()` and selector `0x92214ba3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PairAlreadyListed", abi = "PairAlreadyListed()")]
    pub struct PairAlreadyListed;
    ///Custom Error type `PairNotListed` with signature `PairNotListed()` and selector `0x23d16341`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PairNotListed", abi = "PairNotListed()")]
    pub struct PairNotListed;
    ///Custom Error type `Paused` with signature `Paused()` and selector `0x9e87fac8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Paused", abi = "Paused()")]
    pub struct Paused;
    ///Custom Error type `StartLeverageTooLow` with signature `StartLeverageTooLow()` and selector `0xa3d1c4d1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "StartLeverageTooLow", abi = "StartLeverageTooLow()")]
    pub struct StartLeverageTooLow;
    ///Custom Error type `StartLiqThresholdTooHigh` with signature `StartLiqThresholdTooHigh()` and selector `0x5b99296f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "StartLiqThresholdTooHigh", abi = "StartLiqThresholdTooHigh()")]
    pub struct StartLiqThresholdTooHigh;
    ///Custom Error type `UnsupportedChain` with signature `UnsupportedChain()` and selector `0xd21eab37`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "UnsupportedChain", abi = "UnsupportedChain()")]
    pub struct UnsupportedChain;
    ///Custom Error type `WrongAccess` with signature `WrongAccess()` and selector `0x6c5ffd54`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongAccess", abi = "WrongAccess()")]
    pub struct WrongAccess;
    ///Custom Error type `WrongFees` with signature `WrongFees()` and selector `0x0d4b93bc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongFees", abi = "WrongFees()")]
    pub struct WrongFees;
    ///Custom Error type `WrongIndex` with signature `WrongIndex()` and selector `0x4461028a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongIndex", abi = "WrongIndex()")]
    pub struct WrongIndex;
    ///Custom Error type `WrongLength` with signature `WrongLength()` and selector `0x92aa5ab2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongLength", abi = "WrongLength()")]
    pub struct WrongLength;
    ///Custom Error type `WrongLeverages` with signature `WrongLeverages()` and selector `0x523ae9c7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongLeverages", abi = "WrongLeverages()")]
    pub struct WrongLeverages;
    ///Custom Error type `WrongLiqParamsLeverages` with signature `WrongLiqParamsLeverages()` and selector `0xa1d05279`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongLiqParamsLeverages", abi = "WrongLiqParamsLeverages()")]
    pub struct WrongLiqParamsLeverages;
    ///Custom Error type `WrongLiqParamsThresholds` with signature `WrongLiqParamsThresholds()` and selector `0x0bedb6fd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongLiqParamsThresholds", abi = "WrongLiqParamsThresholds()")]
    pub struct WrongLiqParamsThresholds;
    ///Custom Error type `WrongOrder` with signature `WrongOrder()` and selector `0x09686b32`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongOrder", abi = "WrongOrder()")]
    pub struct WrongOrder;
    ///Custom Error type `WrongOrderType` with signature `WrongOrderType()` and selector `0xbef31f6b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongOrderType", abi = "WrongOrderType()")]
    pub struct WrongOrderType;
    ///Custom Error type `WrongParams` with signature `WrongParams()` and selector `0x5863f789`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongParams", abi = "WrongParams()")]
    pub struct WrongParams;
    ///Custom Error type `WrongTradeType` with signature `WrongTradeType()` and selector `0xd2faaf45`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongTradeType", abi = "WrongTradeType()")]
    pub struct WrongTradeType;
    ///Custom Error type `ZeroAddress` with signature `ZeroAddress()` and selector `0xd92e233d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroAddress", abi = "ZeroAddress()")]
    pub struct ZeroAddress;
    ///Custom Error type `ZeroValue` with signature `ZeroValue()` and selector `0x7c946ed7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroValue", abi = "ZeroValue()")]
    pub struct ZeroValue;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GNSPairsStorageErrors {
        AboveMax(AboveMax),
        AlreadyExists(AlreadyExists),
        BelowMin(BelowMin),
        BlockOrder(BlockOrder),
        DoesntExist(DoesntExist),
        EndLeverageTooHigh(EndLeverageTooHigh),
        EndLiqThresholdTooLow(EndLiqThresholdTooLow),
        FeeNotListed(FeeNotListed),
        GroupNotListed(GroupNotListed),
        InitError(InitError),
        InsufficientBalance(InsufficientBalance),
        InvalidAddresses(InvalidAddresses),
        InvalidCollateralIndex(InvalidCollateralIndex),
        InvalidInputLength(InvalidInputLength),
        MaxLiqSpreadPTooHigh(MaxLiqSpreadPTooHigh),
        NotAllowed(NotAllowed),
        NotAuthorized(NotAuthorized),
        Overflow(Overflow),
        PairAlreadyListed(PairAlreadyListed),
        PairNotListed(PairNotListed),
        Paused(Paused),
        StartLeverageTooLow(StartLeverageTooLow),
        StartLiqThresholdTooHigh(StartLiqThresholdTooHigh),
        UnsupportedChain(UnsupportedChain),
        WrongAccess(WrongAccess),
        WrongFees(WrongFees),
        WrongIndex(WrongIndex),
        WrongLength(WrongLength),
        WrongLeverages(WrongLeverages),
        WrongLiqParamsLeverages(WrongLiqParamsLeverages),
        WrongLiqParamsThresholds(WrongLiqParamsThresholds),
        WrongOrder(WrongOrder),
        WrongOrderType(WrongOrderType),
        WrongParams(WrongParams),
        WrongTradeType(WrongTradeType),
        ZeroAddress(ZeroAddress),
        ZeroValue(ZeroValue),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GNSPairsStorageErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AboveMax as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AboveMax(decoded));
            }
            if let Ok(decoded) = <AlreadyExists as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadyExists(decoded));
            }
            if let Ok(decoded) = <BelowMin as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BelowMin(decoded));
            }
            if let Ok(decoded) = <BlockOrder as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BlockOrder(decoded));
            }
            if let Ok(decoded) = <DoesntExist as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DoesntExist(decoded));
            }
            if let Ok(decoded) =
                <EndLeverageTooHigh as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EndLeverageTooHigh(decoded));
            }
            if let Ok(decoded) =
                <EndLiqThresholdTooLow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EndLiqThresholdTooLow(decoded));
            }
            if let Ok(decoded) = <FeeNotListed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeNotListed(decoded));
            }
            if let Ok(decoded) = <GroupNotListed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GroupNotListed(decoded));
            }
            if let Ok(decoded) = <InitError as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InitError(decoded));
            }
            if let Ok(decoded) =
                <InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientBalance(decoded));
            }
            if let Ok(decoded) = <InvalidAddresses as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidAddresses(decoded));
            }
            if let Ok(decoded) =
                <InvalidCollateralIndex as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidCollateralIndex(decoded));
            }
            if let Ok(decoded) =
                <InvalidInputLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInputLength(decoded));
            }
            if let Ok(decoded) =
                <MaxLiqSpreadPTooHigh as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxLiqSpreadPTooHigh(decoded));
            }
            if let Ok(decoded) = <NotAllowed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotAllowed(decoded));
            }
            if let Ok(decoded) = <NotAuthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotAuthorized(decoded));
            }
            if let Ok(decoded) = <Overflow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Overflow(decoded));
            }
            if let Ok(decoded) = <PairAlreadyListed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PairAlreadyListed(decoded));
            }
            if let Ok(decoded) = <PairNotListed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PairNotListed(decoded));
            }
            if let Ok(decoded) = <Paused as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) =
                <StartLeverageTooLow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StartLeverageTooLow(decoded));
            }
            if let Ok(decoded) =
                <StartLiqThresholdTooHigh as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StartLiqThresholdTooHigh(decoded));
            }
            if let Ok(decoded) = <UnsupportedChain as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnsupportedChain(decoded));
            }
            if let Ok(decoded) = <WrongAccess as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongAccess(decoded));
            }
            if let Ok(decoded) = <WrongFees as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongFees(decoded));
            }
            if let Ok(decoded) = <WrongIndex as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongIndex(decoded));
            }
            if let Ok(decoded) = <WrongLength as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongLength(decoded));
            }
            if let Ok(decoded) = <WrongLeverages as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongLeverages(decoded));
            }
            if let Ok(decoded) =
                <WrongLiqParamsLeverages as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WrongLiqParamsLeverages(decoded));
            }
            if let Ok(decoded) =
                <WrongLiqParamsThresholds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WrongLiqParamsThresholds(decoded));
            }
            if let Ok(decoded) = <WrongOrder as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongOrder(decoded));
            }
            if let Ok(decoded) = <WrongOrderType as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongOrderType(decoded));
            }
            if let Ok(decoded) = <WrongParams as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongParams(decoded));
            }
            if let Ok(decoded) = <WrongTradeType as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongTradeType(decoded));
            }
            if let Ok(decoded) = <ZeroAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroAddress(decoded));
            }
            if let Ok(decoded) = <ZeroValue as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroValue(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GNSPairsStorageErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AboveMax(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AlreadyExists(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BelowMin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BlockOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DoesntExist(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EndLeverageTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EndLiqThresholdTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeNotListed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GroupNotListed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAddresses(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidCollateralIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInputLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxLiqSpreadPTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotAllowed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotAuthorized(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Overflow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairAlreadyListed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairNotListed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StartLeverageTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartLiqThresholdTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnsupportedChain(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongAccess(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongLeverages(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongLiqParamsLeverages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrongLiqParamsThresholds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrongOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongOrderType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongTradeType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroValue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for GNSPairsStorageErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AboveMax as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <BelowMin as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <BlockOrder as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <DoesntExist as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <EndLeverageTooHigh as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <EndLiqThresholdTooLow as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <FeeNotListed as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <GroupNotListed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InitError as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InsufficientBalance as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidAddresses as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCollateralIndex as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidInputLength as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <MaxLiqSpreadPTooHigh as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NotAllowed as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NotAuthorized as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Overflow as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PairAlreadyListed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <PairNotListed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Paused as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <StartLeverageTooLow as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <StartLiqThresholdTooHigh as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <UnsupportedChain as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <WrongAccess as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <WrongFees as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <WrongIndex as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <WrongLength as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <WrongLeverages as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WrongLiqParamsLeverages as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <WrongLiqParamsThresholds as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <WrongOrder as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <WrongOrderType as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <WrongParams as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <WrongTradeType as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <ZeroAddress as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <ZeroValue as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for GNSPairsStorageErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AboveMax(element) => ::core::fmt::Display::fmt(element, f),
                Self::AlreadyExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::BelowMin(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlockOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::DoesntExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::EndLeverageTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::EndLiqThresholdTooLow(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeNotListed(element) => ::core::fmt::Display::fmt(element, f),
                Self::GroupNotListed(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitError(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidCollateralIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInputLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxLiqSpreadPTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotAuthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::Overflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairAlreadyListed(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairNotListed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartLeverageTooLow(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartLiqThresholdTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsupportedChain(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongAccess(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongLeverages(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongLiqParamsLeverages(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongLiqParamsThresholds(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongOrderType(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongTradeType(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for GNSPairsStorageErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AboveMax> for GNSPairsStorageErrors {
        fn from(value: AboveMax) -> Self {
            Self::AboveMax(value)
        }
    }
    impl ::core::convert::From<AlreadyExists> for GNSPairsStorageErrors {
        fn from(value: AlreadyExists) -> Self {
            Self::AlreadyExists(value)
        }
    }
    impl ::core::convert::From<BelowMin> for GNSPairsStorageErrors {
        fn from(value: BelowMin) -> Self {
            Self::BelowMin(value)
        }
    }
    impl ::core::convert::From<BlockOrder> for GNSPairsStorageErrors {
        fn from(value: BlockOrder) -> Self {
            Self::BlockOrder(value)
        }
    }
    impl ::core::convert::From<DoesntExist> for GNSPairsStorageErrors {
        fn from(value: DoesntExist) -> Self {
            Self::DoesntExist(value)
        }
    }
    impl ::core::convert::From<EndLeverageTooHigh> for GNSPairsStorageErrors {
        fn from(value: EndLeverageTooHigh) -> Self {
            Self::EndLeverageTooHigh(value)
        }
    }
    impl ::core::convert::From<EndLiqThresholdTooLow> for GNSPairsStorageErrors {
        fn from(value: EndLiqThresholdTooLow) -> Self {
            Self::EndLiqThresholdTooLow(value)
        }
    }
    impl ::core::convert::From<FeeNotListed> for GNSPairsStorageErrors {
        fn from(value: FeeNotListed) -> Self {
            Self::FeeNotListed(value)
        }
    }
    impl ::core::convert::From<GroupNotListed> for GNSPairsStorageErrors {
        fn from(value: GroupNotListed) -> Self {
            Self::GroupNotListed(value)
        }
    }
    impl ::core::convert::From<InitError> for GNSPairsStorageErrors {
        fn from(value: InitError) -> Self {
            Self::InitError(value)
        }
    }
    impl ::core::convert::From<InsufficientBalance> for GNSPairsStorageErrors {
        fn from(value: InsufficientBalance) -> Self {
            Self::InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<InvalidAddresses> for GNSPairsStorageErrors {
        fn from(value: InvalidAddresses) -> Self {
            Self::InvalidAddresses(value)
        }
    }
    impl ::core::convert::From<InvalidCollateralIndex> for GNSPairsStorageErrors {
        fn from(value: InvalidCollateralIndex) -> Self {
            Self::InvalidCollateralIndex(value)
        }
    }
    impl ::core::convert::From<InvalidInputLength> for GNSPairsStorageErrors {
        fn from(value: InvalidInputLength) -> Self {
            Self::InvalidInputLength(value)
        }
    }
    impl ::core::convert::From<MaxLiqSpreadPTooHigh> for GNSPairsStorageErrors {
        fn from(value: MaxLiqSpreadPTooHigh) -> Self {
            Self::MaxLiqSpreadPTooHigh(value)
        }
    }
    impl ::core::convert::From<NotAllowed> for GNSPairsStorageErrors {
        fn from(value: NotAllowed) -> Self {
            Self::NotAllowed(value)
        }
    }
    impl ::core::convert::From<NotAuthorized> for GNSPairsStorageErrors {
        fn from(value: NotAuthorized) -> Self {
            Self::NotAuthorized(value)
        }
    }
    impl ::core::convert::From<Overflow> for GNSPairsStorageErrors {
        fn from(value: Overflow) -> Self {
            Self::Overflow(value)
        }
    }
    impl ::core::convert::From<PairAlreadyListed> for GNSPairsStorageErrors {
        fn from(value: PairAlreadyListed) -> Self {
            Self::PairAlreadyListed(value)
        }
    }
    impl ::core::convert::From<PairNotListed> for GNSPairsStorageErrors {
        fn from(value: PairNotListed) -> Self {
            Self::PairNotListed(value)
        }
    }
    impl ::core::convert::From<Paused> for GNSPairsStorageErrors {
        fn from(value: Paused) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<StartLeverageTooLow> for GNSPairsStorageErrors {
        fn from(value: StartLeverageTooLow) -> Self {
            Self::StartLeverageTooLow(value)
        }
    }
    impl ::core::convert::From<StartLiqThresholdTooHigh> for GNSPairsStorageErrors {
        fn from(value: StartLiqThresholdTooHigh) -> Self {
            Self::StartLiqThresholdTooHigh(value)
        }
    }
    impl ::core::convert::From<UnsupportedChain> for GNSPairsStorageErrors {
        fn from(value: UnsupportedChain) -> Self {
            Self::UnsupportedChain(value)
        }
    }
    impl ::core::convert::From<WrongAccess> for GNSPairsStorageErrors {
        fn from(value: WrongAccess) -> Self {
            Self::WrongAccess(value)
        }
    }
    impl ::core::convert::From<WrongFees> for GNSPairsStorageErrors {
        fn from(value: WrongFees) -> Self {
            Self::WrongFees(value)
        }
    }
    impl ::core::convert::From<WrongIndex> for GNSPairsStorageErrors {
        fn from(value: WrongIndex) -> Self {
            Self::WrongIndex(value)
        }
    }
    impl ::core::convert::From<WrongLength> for GNSPairsStorageErrors {
        fn from(value: WrongLength) -> Self {
            Self::WrongLength(value)
        }
    }
    impl ::core::convert::From<WrongLeverages> for GNSPairsStorageErrors {
        fn from(value: WrongLeverages) -> Self {
            Self::WrongLeverages(value)
        }
    }
    impl ::core::convert::From<WrongLiqParamsLeverages> for GNSPairsStorageErrors {
        fn from(value: WrongLiqParamsLeverages) -> Self {
            Self::WrongLiqParamsLeverages(value)
        }
    }
    impl ::core::convert::From<WrongLiqParamsThresholds> for GNSPairsStorageErrors {
        fn from(value: WrongLiqParamsThresholds) -> Self {
            Self::WrongLiqParamsThresholds(value)
        }
    }
    impl ::core::convert::From<WrongOrder> for GNSPairsStorageErrors {
        fn from(value: WrongOrder) -> Self {
            Self::WrongOrder(value)
        }
    }
    impl ::core::convert::From<WrongOrderType> for GNSPairsStorageErrors {
        fn from(value: WrongOrderType) -> Self {
            Self::WrongOrderType(value)
        }
    }
    impl ::core::convert::From<WrongParams> for GNSPairsStorageErrors {
        fn from(value: WrongParams) -> Self {
            Self::WrongParams(value)
        }
    }
    impl ::core::convert::From<WrongTradeType> for GNSPairsStorageErrors {
        fn from(value: WrongTradeType) -> Self {
            Self::WrongTradeType(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for GNSPairsStorageErrors {
        fn from(value: ZeroAddress) -> Self {
            Self::ZeroAddress(value)
        }
    }
    impl ::core::convert::From<ZeroValue> for GNSPairsStorageErrors {
        fn from(value: ZeroValue) -> Self {
            Self::ZeroValue(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "AccessControlUpdated",
        abi = "AccessControlUpdated(address,uint8,bool)"
    )]
    pub struct AccessControlUpdatedFilter {
        pub target: ::ethers::core::types::Address,
        pub role: u8,
        pub access: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "AddressesUpdated",
        abi = "AddressesUpdated((address,address,address))"
    )]
    pub struct AddressesUpdatedFilter {
        pub addresses: Addresses,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "FeeAdded",
        abi = "FeeAdded(uint256,(uint40,uint40,uint40,uint32,uint104))"
    )]
    pub struct FeeAddedFilter {
        pub index: ::ethers::core::types::U256,
        pub fee_group: FeeGroup,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "FeeUpdated",
        abi = "FeeUpdated(uint256,(uint40,uint40,uint40,uint32,uint104))"
    )]
    pub struct FeeUpdatedFilter {
        pub index: ::ethers::core::types::U256,
        pub fee_group: FeeGroup,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GlobalTradeFeeParamsUpdated",
        abi = "GlobalTradeFeeParamsUpdated((uint24,uint24,uint24,uint24,uint24,uint136))"
    )]
    pub struct GlobalTradeFeeParamsUpdatedFilter {
        pub fee_params: GlobalTradeFeeParams,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "GroupAdded", abi = "GroupAdded(uint256,string)")]
    pub struct GroupAddedFilter {
        pub index: ::ethers::core::types::U256,
        pub name: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GroupLiquidationParamsUpdated",
        abi = "GroupLiquidationParamsUpdated(uint256,(uint40,uint40,uint40,uint24,uint24))"
    )]
    pub struct GroupLiquidationParamsUpdatedFilter {
        pub index: ::ethers::core::types::U256,
        pub params: GroupLiquidationParams,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "GroupUpdated", abi = "GroupUpdated(uint256)")]
    pub struct GroupUpdatedFilter {
        pub index: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PairAdded", abi = "PairAdded(uint256,string,string)")]
    pub struct PairAddedFilter {
        pub index: ::ethers::core::types::U256,
        pub from: ::std::string::String,
        pub to: ::std::string::String,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "PairCustomMaxLeverageUpdated",
        abi = "PairCustomMaxLeverageUpdated(uint256,uint256)"
    )]
    pub struct PairCustomMaxLeverageUpdatedFilter {
        #[ethevent(indexed)]
        pub index: ::ethers::core::types::U256,
        pub max_leverage: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "PairUpdated", abi = "PairUpdated(uint256)")]
    pub struct PairUpdatedFilter {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GNSPairsStorageEvents {
        AccessControlUpdatedFilter(AccessControlUpdatedFilter),
        AddressesUpdatedFilter(AddressesUpdatedFilter),
        FeeAddedFilter(FeeAddedFilter),
        FeeUpdatedFilter(FeeUpdatedFilter),
        GlobalTradeFeeParamsUpdatedFilter(GlobalTradeFeeParamsUpdatedFilter),
        GroupAddedFilter(GroupAddedFilter),
        GroupLiquidationParamsUpdatedFilter(GroupLiquidationParamsUpdatedFilter),
        GroupUpdatedFilter(GroupUpdatedFilter),
        InitializedFilter(InitializedFilter),
        PairAddedFilter(PairAddedFilter),
        PairCustomMaxLeverageUpdatedFilter(PairCustomMaxLeverageUpdatedFilter),
        PairUpdatedFilter(PairUpdatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for GNSPairsStorageEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AccessControlUpdatedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::AccessControlUpdatedFilter(decoded));
            }
            if let Ok(decoded) = AddressesUpdatedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::AddressesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = FeeAddedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::FeeAddedFilter(decoded));
            }
            if let Ok(decoded) = FeeUpdatedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::FeeUpdatedFilter(decoded));
            }
            if let Ok(decoded) = GlobalTradeFeeParamsUpdatedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::GlobalTradeFeeParamsUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = GroupAddedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::GroupAddedFilter(decoded));
            }
            if let Ok(decoded) = GroupLiquidationParamsUpdatedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::GroupLiquidationParamsUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = GroupUpdatedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::GroupUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = PairAddedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::PairAddedFilter(decoded));
            }
            if let Ok(decoded) = PairCustomMaxLeverageUpdatedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::PairCustomMaxLeverageUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PairUpdatedFilter::decode_log(log) {
                return Ok(GNSPairsStorageEvents::PairUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GNSPairsStorageEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessControlUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressesUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GlobalTradeFeeParamsUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GroupAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GroupLiquidationParamsUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GroupUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairCustomMaxLeverageUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PairUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccessControlUpdatedFilter> for GNSPairsStorageEvents {
        fn from(value: AccessControlUpdatedFilter) -> Self {
            Self::AccessControlUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<AddressesUpdatedFilter> for GNSPairsStorageEvents {
        fn from(value: AddressesUpdatedFilter) -> Self {
            Self::AddressesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<FeeAddedFilter> for GNSPairsStorageEvents {
        fn from(value: FeeAddedFilter) -> Self {
            Self::FeeAddedFilter(value)
        }
    }
    impl ::core::convert::From<FeeUpdatedFilter> for GNSPairsStorageEvents {
        fn from(value: FeeUpdatedFilter) -> Self {
            Self::FeeUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<GlobalTradeFeeParamsUpdatedFilter> for GNSPairsStorageEvents {
        fn from(value: GlobalTradeFeeParamsUpdatedFilter) -> Self {
            Self::GlobalTradeFeeParamsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<GroupAddedFilter> for GNSPairsStorageEvents {
        fn from(value: GroupAddedFilter) -> Self {
            Self::GroupAddedFilter(value)
        }
    }
    impl ::core::convert::From<GroupLiquidationParamsUpdatedFilter> for GNSPairsStorageEvents {
        fn from(value: GroupLiquidationParamsUpdatedFilter) -> Self {
            Self::GroupLiquidationParamsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<GroupUpdatedFilter> for GNSPairsStorageEvents {
        fn from(value: GroupUpdatedFilter) -> Self {
            Self::GroupUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for GNSPairsStorageEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<PairAddedFilter> for GNSPairsStorageEvents {
        fn from(value: PairAddedFilter) -> Self {
            Self::PairAddedFilter(value)
        }
    }
    impl ::core::convert::From<PairCustomMaxLeverageUpdatedFilter> for GNSPairsStorageEvents {
        fn from(value: PairCustomMaxLeverageUpdatedFilter) -> Self {
            Self::PairCustomMaxLeverageUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PairUpdatedFilter> for GNSPairsStorageEvents {
        fn from(value: PairUpdatedFilter) -> Self {
            Self::PairUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `addFees` function with signature `addFees((uint40,uint40,uint40,uint32,uint104)[])` and selector `0x0e5c75af`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "addFees",
        abi = "addFees((uint40,uint40,uint40,uint32,uint104)[])"
    )]
    pub struct AddFeesCall {
        pub fees: ::std::vec::Vec<FeeGroup>,
    }
    ///Container type for all input parameters for the `addGroups` function with signature `addGroups((string,bytes32,uint256,uint256)[])` and selector `0x60283cba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "addGroups",
        abi = "addGroups((string,bytes32,uint256,uint256)[])"
    )]
    pub struct AddGroupsCall {
        pub groups: ::std::vec::Vec<Group>,
    }
    ///Container type for all input parameters for the `addPairs` function with signature `addPairs((string,string,(address,address,uint8,uint256),uint256,uint256,uint256)[])` and selector `0xdb7c3f9d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "addPairs",
        abi = "addPairs((string,string,(address,address,uint8,uint256),uint256,uint256,uint256)[])"
    )]
    pub struct AddPairsCall {
        pub pairs: ::std::vec::Vec<Pair>,
    }
    ///Container type for all input parameters for the `fees` function with signature `fees(uint256)` and selector `0x4acc79ed`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "fees", abi = "fees(uint256)")]
    pub struct FeesCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `feesCount` function with signature `feesCount()` and selector `0x658de48a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "feesCount", abi = "feesCount()")]
    pub struct FeesCountCall;
    ///Container type for all input parameters for the `getAllPairsRestrictedMaxLeverage` function with signature `getAllPairsRestrictedMaxLeverage()` and selector `0x678b3fb0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getAllPairsRestrictedMaxLeverage",
        abi = "getAllPairsRestrictedMaxLeverage()"
    )]
    pub struct GetAllPairsRestrictedMaxLeverageCall;
    ///Container type for all input parameters for the `getGlobalTradeFeeParams` function with signature `getGlobalTradeFeeParams()` and selector `0x48fc3ef3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getGlobalTradeFeeParams", abi = "getGlobalTradeFeeParams()")]
    pub struct GetGlobalTradeFeeParamsCall;
    ///Container type for all input parameters for the `getGroupLiquidationParams` function with signature `getGroupLiquidationParams(uint256)` and selector `0x3572929c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getGroupLiquidationParams",
        abi = "getGroupLiquidationParams(uint256)"
    )]
    pub struct GetGroupLiquidationParamsCall {
        pub group_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPairLiquidationParams` function with signature `getPairLiquidationParams(uint256)` and selector `0x6633ced6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getPairLiquidationParams",
        abi = "getPairLiquidationParams(uint256)"
    )]
    pub struct GetPairLiquidationParamsCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `groups` function with signature `groups(uint256)` and selector `0x96324bd4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "groups", abi = "groups(uint256)")]
    pub struct GroupsCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `groupsCount` function with signature `groupsCount()` and selector `0x885e2750`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "groupsCount", abi = "groupsCount()")]
    pub struct GroupsCountCall;
    ///Container type for all input parameters for the `initializeGroupLiquidationParams` function with signature `initializeGroupLiquidationParams((uint40,uint40,uint40,uint24,uint24)[])` and selector `0x85d4390e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "initializeGroupLiquidationParams",
        abi = "initializeGroupLiquidationParams((uint40,uint40,uint40,uint24,uint24)[])"
    )]
    pub struct InitializeGroupLiquidationParamsCall {
        pub group_liquidation_params: ::std::vec::Vec<GroupLiquidationParams>,
    }
    ///Container type for all input parameters for the `initializeNewFees` function with signature `initializeNewFees((uint24,uint24,uint24,uint24,uint24,uint136))` and selector `0xc85ac748`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "initializeNewFees",
        abi = "initializeNewFees((uint24,uint24,uint24,uint24,uint24,uint136))"
    )]
    pub struct InitializeNewFeesCall {
        pub trade_fee_params: GlobalTradeFeeParams,
    }
    ///Container type for all input parameters for the `isPairIndexListed` function with signature `isPairIndexListed(uint256)` and selector `0x281b7ead`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isPairIndexListed", abi = "isPairIndexListed(uint256)")]
    pub struct IsPairIndexListedCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isPairListed` function with signature `isPairListed(string,string)` and selector `0x1628bfeb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isPairListed", abi = "isPairListed(string,string)")]
    pub struct IsPairListedCall {
        pub from: ::std::string::String,
        pub to: ::std::string::String,
    }
    ///Container type for all input parameters for the `pairCustomMaxLeverage` function with signature `pairCustomMaxLeverage(uint256)` and selector `0x24a96865`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pairCustomMaxLeverage", abi = "pairCustomMaxLeverage(uint256)")]
    pub struct PairCustomMaxLeverageCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairJob` function with signature `pairJob(uint256)` and selector `0x302f81fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pairJob", abi = "pairJob(uint256)")]
    pub struct PairJobCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairMaxLeverage` function with signature `pairMaxLeverage(uint256)` and selector `0x281b693c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pairMaxLeverage", abi = "pairMaxLeverage(uint256)")]
    pub struct PairMaxLeverageCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairMinFeeUsd` function with signature `pairMinFeeUsd(uint256)` and selector `0x8078bfbe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pairMinFeeUsd", abi = "pairMinFeeUsd(uint256)")]
    pub struct PairMinFeeUsdCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairMinLeverage` function with signature `pairMinLeverage(uint256)` and selector `0x59a992d0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pairMinLeverage", abi = "pairMinLeverage(uint256)")]
    pub struct PairMinLeverageCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairMinPositionSizeUsd` function with signature `pairMinPositionSizeUsd(uint256)` and selector `0x5e26ff4e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "pairMinPositionSizeUsd",
        abi = "pairMinPositionSizeUsd(uint256)"
    )]
    pub struct PairMinPositionSizeUsdCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairOraclePositionSizeFeeP` function with signature `pairOraclePositionSizeFeeP(uint256)` and selector `0x0cf34255`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "pairOraclePositionSizeFeeP",
        abi = "pairOraclePositionSizeFeeP(uint256)"
    )]
    pub struct PairOraclePositionSizeFeePCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairSpreadP` function with signature `pairSpreadP(uint256)` and selector `0xa1d54e9b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pairSpreadP", abi = "pairSpreadP(uint256)")]
    pub struct PairSpreadPCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairTotalLiqCollateralFeeP` function with signature `pairTotalLiqCollateralFeeP(uint256)` and selector `0xfbf3f911`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "pairTotalLiqCollateralFeeP",
        abi = "pairTotalLiqCollateralFeeP(uint256)"
    )]
    pub struct PairTotalLiqCollateralFeePCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairTotalPositionSizeFeeP` function with signature `pairTotalPositionSizeFeeP(uint256)` and selector `0xc927b4b6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "pairTotalPositionSizeFeeP",
        abi = "pairTotalPositionSizeFeeP(uint256)"
    )]
    pub struct PairTotalPositionSizeFeePCall {
        pub pair_index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairs` function with signature `pairs(uint256)` and selector `0xb91ac788`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pairs", abi = "pairs(uint256)")]
    pub struct PairsCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pairsCount` function with signature `pairsCount()` and selector `0xb81b2b71`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pairsCount", abi = "pairsCount()")]
    pub struct PairsCountCall;
    ///Container type for all input parameters for the `setGlobalTradeFeeParams` function with signature `setGlobalTradeFeeParams((uint24,uint24,uint24,uint24,uint24,uint136))` and selector `0x5f57a073`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setGlobalTradeFeeParams",
        abi = "setGlobalTradeFeeParams((uint24,uint24,uint24,uint24,uint24,uint136))"
    )]
    pub struct SetGlobalTradeFeeParamsCall {
        pub fee_params: GlobalTradeFeeParams,
    }
    ///Container type for all input parameters for the `setGroupLiquidationParams` function with signature `setGroupLiquidationParams(uint256,(uint40,uint40,uint40,uint24,uint24))` and selector `0xd0cb753e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setGroupLiquidationParams",
        abi = "setGroupLiquidationParams(uint256,(uint40,uint40,uint40,uint24,uint24))"
    )]
    pub struct SetGroupLiquidationParamsCall {
        pub group_index: ::ethers::core::types::U256,
        pub params: GroupLiquidationParams,
    }
    ///Container type for all input parameters for the `setPairCustomMaxLeverages` function with signature `setPairCustomMaxLeverages(uint256[],uint256[])` and selector `0xd79261fd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setPairCustomMaxLeverages",
        abi = "setPairCustomMaxLeverages(uint256[],uint256[])"
    )]
    pub struct SetPairCustomMaxLeveragesCall {
        pub indices: ::std::vec::Vec<::ethers::core::types::U256>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `updateFees` function with signature `updateFees(uint256[],(uint40,uint40,uint40,uint32,uint104)[])` and selector `0x1cf71b9b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "updateFees",
        abi = "updateFees(uint256[],(uint40,uint40,uint40,uint32,uint104)[])"
    )]
    pub struct UpdateFeesCall {
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub fees: ::std::vec::Vec<FeeGroup>,
    }
    ///Container type for all input parameters for the `updateGroups` function with signature `updateGroups(uint256[],(string,bytes32,uint256,uint256)[])` and selector `0x11d79ef5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "updateGroups",
        abi = "updateGroups(uint256[],(string,bytes32,uint256,uint256)[])"
    )]
    pub struct UpdateGroupsCall {
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub groups: ::std::vec::Vec<Group>,
    }
    ///Container type for all input parameters for the `updatePairs` function with signature `updatePairs(uint256[],(string,string,(address,address,uint8,uint256),uint256,uint256,uint256)[])` and selector `0x10efa5d5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "updatePairs",
        abi = "updatePairs(uint256[],(string,string,(address,address,uint8,uint256),uint256,uint256,uint256)[])"
    )]
    pub struct UpdatePairsCall {
        pub pair_indices: ::std::vec::Vec<::ethers::core::types::U256>,
        pub pairs: ::std::vec::Vec<Pair>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GNSPairsStorageCalls {
        AddFees(AddFeesCall),
        AddGroups(AddGroupsCall),
        AddPairs(AddPairsCall),
        Fees(FeesCall),
        FeesCount(FeesCountCall),
        GetAllPairsRestrictedMaxLeverage(GetAllPairsRestrictedMaxLeverageCall),
        GetGlobalTradeFeeParams(GetGlobalTradeFeeParamsCall),
        GetGroupLiquidationParams(GetGroupLiquidationParamsCall),
        GetPairLiquidationParams(GetPairLiquidationParamsCall),
        Groups(GroupsCall),
        GroupsCount(GroupsCountCall),
        InitializeGroupLiquidationParams(InitializeGroupLiquidationParamsCall),
        InitializeNewFees(InitializeNewFeesCall),
        IsPairIndexListed(IsPairIndexListedCall),
        IsPairListed(IsPairListedCall),
        PairCustomMaxLeverage(PairCustomMaxLeverageCall),
        PairJob(PairJobCall),
        PairMaxLeverage(PairMaxLeverageCall),
        PairMinFeeUsd(PairMinFeeUsdCall),
        PairMinLeverage(PairMinLeverageCall),
        PairMinPositionSizeUsd(PairMinPositionSizeUsdCall),
        PairOraclePositionSizeFeeP(PairOraclePositionSizeFeePCall),
        PairSpreadP(PairSpreadPCall),
        PairTotalLiqCollateralFeeP(PairTotalLiqCollateralFeePCall),
        PairTotalPositionSizeFeeP(PairTotalPositionSizeFeePCall),
        Pairs(PairsCall),
        PairsCount(PairsCountCall),
        SetGlobalTradeFeeParams(SetGlobalTradeFeeParamsCall),
        SetGroupLiquidationParams(SetGroupLiquidationParamsCall),
        SetPairCustomMaxLeverages(SetPairCustomMaxLeveragesCall),
        UpdateFees(UpdateFeesCall),
        UpdateGroups(UpdateGroupsCall),
        UpdatePairs(UpdatePairsCall),
    }
    impl ::ethers::core::abi::AbiDecode for GNSPairsStorageCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddFees(decoded));
            }
            if let Ok(decoded) = <AddGroupsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddGroups(decoded));
            }
            if let Ok(decoded) = <AddPairsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddPairs(decoded));
            }
            if let Ok(decoded) = <FeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fees(decoded));
            }
            if let Ok(decoded) = <FeesCountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeesCount(decoded));
            }
            if let Ok(decoded) =
                <GetAllPairsRestrictedMaxLeverageCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetAllPairsRestrictedMaxLeverage(decoded));
            }
            if let Ok(decoded) =
                <GetGlobalTradeFeeParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetGlobalTradeFeeParams(decoded));
            }
            if let Ok(decoded) =
                <GetGroupLiquidationParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetGroupLiquidationParams(decoded));
            }
            if let Ok(decoded) =
                <GetPairLiquidationParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPairLiquidationParams(decoded));
            }
            if let Ok(decoded) = <GroupsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Groups(decoded));
            }
            if let Ok(decoded) = <GroupsCountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GroupsCount(decoded));
            }
            if let Ok(decoded) =
                <InitializeGroupLiquidationParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::InitializeGroupLiquidationParams(decoded));
            }
            if let Ok(decoded) =
                <InitializeNewFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InitializeNewFees(decoded));
            }
            if let Ok(decoded) =
                <IsPairIndexListedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsPairIndexListed(decoded));
            }
            if let Ok(decoded) = <IsPairListedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsPairListed(decoded));
            }
            if let Ok(decoded) =
                <PairCustomMaxLeverageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PairCustomMaxLeverage(decoded));
            }
            if let Ok(decoded) = <PairJobCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PairJob(decoded));
            }
            if let Ok(decoded) =
                <PairMaxLeverageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PairMaxLeverage(decoded));
            }
            if let Ok(decoded) = <PairMinFeeUsdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PairMinFeeUsd(decoded));
            }
            if let Ok(decoded) =
                <PairMinLeverageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PairMinLeverage(decoded));
            }
            if let Ok(decoded) =
                <PairMinPositionSizeUsdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PairMinPositionSizeUsd(decoded));
            }
            if let Ok(decoded) =
                <PairOraclePositionSizeFeePCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PairOraclePositionSizeFeeP(decoded));
            }
            if let Ok(decoded) = <PairSpreadPCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PairSpreadP(decoded));
            }
            if let Ok(decoded) =
                <PairTotalLiqCollateralFeePCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PairTotalLiqCollateralFeeP(decoded));
            }
            if let Ok(decoded) =
                <PairTotalPositionSizeFeePCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PairTotalPositionSizeFeeP(decoded));
            }
            if let Ok(decoded) = <PairsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pairs(decoded));
            }
            if let Ok(decoded) = <PairsCountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PairsCount(decoded));
            }
            if let Ok(decoded) =
                <SetGlobalTradeFeeParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetGlobalTradeFeeParams(decoded));
            }
            if let Ok(decoded) =
                <SetGroupLiquidationParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetGroupLiquidationParams(decoded));
            }
            if let Ok(decoded) =
                <SetPairCustomMaxLeveragesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPairCustomMaxLeverages(decoded));
            }
            if let Ok(decoded) = <UpdateFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateFees(decoded));
            }
            if let Ok(decoded) = <UpdateGroupsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateGroups(decoded));
            }
            if let Ok(decoded) = <UpdatePairsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdatePairs(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GNSPairsStorageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddGroups(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddPairs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeesCount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllPairsRestrictedMaxLeverage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGlobalTradeFeeParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetGroupLiquidationParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPairLiquidationParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Groups(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GroupsCount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InitializeGroupLiquidationParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitializeNewFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsPairIndexListed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsPairListed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairCustomMaxLeverage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PairJob(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairMaxLeverage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairMinFeeUsd(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairMinLeverage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairMinPositionSizeUsd(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PairOraclePositionSizeFeeP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PairSpreadP(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairTotalLiqCollateralFeeP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PairTotalPositionSizeFeeP(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pairs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairsCount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetGlobalTradeFeeParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetGroupLiquidationParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPairCustomMaxLeverages(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateGroups(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdatePairs(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GNSPairsStorageCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddGroups(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddPairs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fees(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllPairsRestrictedMaxLeverage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetGlobalTradeFeeParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetGroupLiquidationParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPairLiquidationParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Groups(element) => ::core::fmt::Display::fmt(element, f),
                Self::GroupsCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializeGroupLiquidationParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializeNewFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPairIndexListed(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPairListed(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairCustomMaxLeverage(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairJob(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairMaxLeverage(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairMinFeeUsd(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairMinLeverage(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairMinPositionSizeUsd(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairOraclePositionSizeFeeP(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairSpreadP(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairTotalLiqCollateralFeeP(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairTotalPositionSizeFeeP(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pairs(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairsCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGlobalTradeFeeParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGroupLiquidationParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPairCustomMaxLeverages(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateGroups(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePairs(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddFeesCall> for GNSPairsStorageCalls {
        fn from(value: AddFeesCall) -> Self {
            Self::AddFees(value)
        }
    }
    impl ::core::convert::From<AddGroupsCall> for GNSPairsStorageCalls {
        fn from(value: AddGroupsCall) -> Self {
            Self::AddGroups(value)
        }
    }
    impl ::core::convert::From<AddPairsCall> for GNSPairsStorageCalls {
        fn from(value: AddPairsCall) -> Self {
            Self::AddPairs(value)
        }
    }
    impl ::core::convert::From<FeesCall> for GNSPairsStorageCalls {
        fn from(value: FeesCall) -> Self {
            Self::Fees(value)
        }
    }
    impl ::core::convert::From<FeesCountCall> for GNSPairsStorageCalls {
        fn from(value: FeesCountCall) -> Self {
            Self::FeesCount(value)
        }
    }
    impl ::core::convert::From<GetAllPairsRestrictedMaxLeverageCall> for GNSPairsStorageCalls {
        fn from(value: GetAllPairsRestrictedMaxLeverageCall) -> Self {
            Self::GetAllPairsRestrictedMaxLeverage(value)
        }
    }
    impl ::core::convert::From<GetGlobalTradeFeeParamsCall> for GNSPairsStorageCalls {
        fn from(value: GetGlobalTradeFeeParamsCall) -> Self {
            Self::GetGlobalTradeFeeParams(value)
        }
    }
    impl ::core::convert::From<GetGroupLiquidationParamsCall> for GNSPairsStorageCalls {
        fn from(value: GetGroupLiquidationParamsCall) -> Self {
            Self::GetGroupLiquidationParams(value)
        }
    }
    impl ::core::convert::From<GetPairLiquidationParamsCall> for GNSPairsStorageCalls {
        fn from(value: GetPairLiquidationParamsCall) -> Self {
            Self::GetPairLiquidationParams(value)
        }
    }
    impl ::core::convert::From<GroupsCall> for GNSPairsStorageCalls {
        fn from(value: GroupsCall) -> Self {
            Self::Groups(value)
        }
    }
    impl ::core::convert::From<GroupsCountCall> for GNSPairsStorageCalls {
        fn from(value: GroupsCountCall) -> Self {
            Self::GroupsCount(value)
        }
    }
    impl ::core::convert::From<InitializeGroupLiquidationParamsCall> for GNSPairsStorageCalls {
        fn from(value: InitializeGroupLiquidationParamsCall) -> Self {
            Self::InitializeGroupLiquidationParams(value)
        }
    }
    impl ::core::convert::From<InitializeNewFeesCall> for GNSPairsStorageCalls {
        fn from(value: InitializeNewFeesCall) -> Self {
            Self::InitializeNewFees(value)
        }
    }
    impl ::core::convert::From<IsPairIndexListedCall> for GNSPairsStorageCalls {
        fn from(value: IsPairIndexListedCall) -> Self {
            Self::IsPairIndexListed(value)
        }
    }
    impl ::core::convert::From<IsPairListedCall> for GNSPairsStorageCalls {
        fn from(value: IsPairListedCall) -> Self {
            Self::IsPairListed(value)
        }
    }
    impl ::core::convert::From<PairCustomMaxLeverageCall> for GNSPairsStorageCalls {
        fn from(value: PairCustomMaxLeverageCall) -> Self {
            Self::PairCustomMaxLeverage(value)
        }
    }
    impl ::core::convert::From<PairJobCall> for GNSPairsStorageCalls {
        fn from(value: PairJobCall) -> Self {
            Self::PairJob(value)
        }
    }
    impl ::core::convert::From<PairMaxLeverageCall> for GNSPairsStorageCalls {
        fn from(value: PairMaxLeverageCall) -> Self {
            Self::PairMaxLeverage(value)
        }
    }
    impl ::core::convert::From<PairMinFeeUsdCall> for GNSPairsStorageCalls {
        fn from(value: PairMinFeeUsdCall) -> Self {
            Self::PairMinFeeUsd(value)
        }
    }
    impl ::core::convert::From<PairMinLeverageCall> for GNSPairsStorageCalls {
        fn from(value: PairMinLeverageCall) -> Self {
            Self::PairMinLeverage(value)
        }
    }
    impl ::core::convert::From<PairMinPositionSizeUsdCall> for GNSPairsStorageCalls {
        fn from(value: PairMinPositionSizeUsdCall) -> Self {
            Self::PairMinPositionSizeUsd(value)
        }
    }
    impl ::core::convert::From<PairOraclePositionSizeFeePCall> for GNSPairsStorageCalls {
        fn from(value: PairOraclePositionSizeFeePCall) -> Self {
            Self::PairOraclePositionSizeFeeP(value)
        }
    }
    impl ::core::convert::From<PairSpreadPCall> for GNSPairsStorageCalls {
        fn from(value: PairSpreadPCall) -> Self {
            Self::PairSpreadP(value)
        }
    }
    impl ::core::convert::From<PairTotalLiqCollateralFeePCall> for GNSPairsStorageCalls {
        fn from(value: PairTotalLiqCollateralFeePCall) -> Self {
            Self::PairTotalLiqCollateralFeeP(value)
        }
    }
    impl ::core::convert::From<PairTotalPositionSizeFeePCall> for GNSPairsStorageCalls {
        fn from(value: PairTotalPositionSizeFeePCall) -> Self {
            Self::PairTotalPositionSizeFeeP(value)
        }
    }
    impl ::core::convert::From<PairsCall> for GNSPairsStorageCalls {
        fn from(value: PairsCall) -> Self {
            Self::Pairs(value)
        }
    }
    impl ::core::convert::From<PairsCountCall> for GNSPairsStorageCalls {
        fn from(value: PairsCountCall) -> Self {
            Self::PairsCount(value)
        }
    }
    impl ::core::convert::From<SetGlobalTradeFeeParamsCall> for GNSPairsStorageCalls {
        fn from(value: SetGlobalTradeFeeParamsCall) -> Self {
            Self::SetGlobalTradeFeeParams(value)
        }
    }
    impl ::core::convert::From<SetGroupLiquidationParamsCall> for GNSPairsStorageCalls {
        fn from(value: SetGroupLiquidationParamsCall) -> Self {
            Self::SetGroupLiquidationParams(value)
        }
    }
    impl ::core::convert::From<SetPairCustomMaxLeveragesCall> for GNSPairsStorageCalls {
        fn from(value: SetPairCustomMaxLeveragesCall) -> Self {
            Self::SetPairCustomMaxLeverages(value)
        }
    }
    impl ::core::convert::From<UpdateFeesCall> for GNSPairsStorageCalls {
        fn from(value: UpdateFeesCall) -> Self {
            Self::UpdateFees(value)
        }
    }
    impl ::core::convert::From<UpdateGroupsCall> for GNSPairsStorageCalls {
        fn from(value: UpdateGroupsCall) -> Self {
            Self::UpdateGroups(value)
        }
    }
    impl ::core::convert::From<UpdatePairsCall> for GNSPairsStorageCalls {
        fn from(value: UpdatePairsCall) -> Self {
            Self::UpdatePairs(value)
        }
    }
    ///Container type for all return fields from the `fees` function with signature `fees(uint256)` and selector `0x4acc79ed`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeesReturn(pub FeeGroup);
    ///Container type for all return fields from the `feesCount` function with signature `feesCount()` and selector `0x658de48a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeesCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getAllPairsRestrictedMaxLeverage` function with signature `getAllPairsRestrictedMaxLeverage()` and selector `0x678b3fb0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAllPairsRestrictedMaxLeverageReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `getGlobalTradeFeeParams` function with signature `getGlobalTradeFeeParams()` and selector `0x48fc3ef3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetGlobalTradeFeeParamsReturn(pub GlobalTradeFeeParams);
    ///Container type for all return fields from the `getGroupLiquidationParams` function with signature `getGroupLiquidationParams(uint256)` and selector `0x3572929c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetGroupLiquidationParamsReturn(pub GroupLiquidationParams);
    ///Container type for all return fields from the `getPairLiquidationParams` function with signature `getPairLiquidationParams(uint256)` and selector `0x6633ced6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPairLiquidationParamsReturn(pub GroupLiquidationParams);
    ///Container type for all return fields from the `groups` function with signature `groups(uint256)` and selector `0x96324bd4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GroupsReturn(pub Group);
    ///Container type for all return fields from the `groupsCount` function with signature `groupsCount()` and selector `0x885e2750`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GroupsCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isPairIndexListed` function with signature `isPairIndexListed(uint256)` and selector `0x281b7ead`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsPairIndexListedReturn(pub bool);
    ///Container type for all return fields from the `isPairListed` function with signature `isPairListed(string,string)` and selector `0x1628bfeb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsPairListedReturn(pub bool);
    ///Container type for all return fields from the `pairCustomMaxLeverage` function with signature `pairCustomMaxLeverage(uint256)` and selector `0x24a96865`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairCustomMaxLeverageReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pairJob` function with signature `pairJob(uint256)` and selector `0x302f81fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairJobReturn(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all return fields from the `pairMaxLeverage` function with signature `pairMaxLeverage(uint256)` and selector `0x281b693c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairMaxLeverageReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pairMinFeeUsd` function with signature `pairMinFeeUsd(uint256)` and selector `0x8078bfbe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairMinFeeUsdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pairMinLeverage` function with signature `pairMinLeverage(uint256)` and selector `0x59a992d0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairMinLeverageReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pairMinPositionSizeUsd` function with signature `pairMinPositionSizeUsd(uint256)` and selector `0x5e26ff4e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairMinPositionSizeUsdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pairOraclePositionSizeFeeP` function with signature `pairOraclePositionSizeFeeP(uint256)` and selector `0x0cf34255`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairOraclePositionSizeFeePReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pairSpreadP` function with signature `pairSpreadP(uint256)` and selector `0xa1d54e9b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairSpreadPReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pairTotalLiqCollateralFeeP` function with signature `pairTotalLiqCollateralFeeP(uint256)` and selector `0xfbf3f911`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairTotalLiqCollateralFeePReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pairTotalPositionSizeFeeP` function with signature `pairTotalPositionSizeFeeP(uint256)` and selector `0xc927b4b6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairTotalPositionSizeFeePReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pairs` function with signature `pairs(uint256)` and selector `0xb91ac788`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairsReturn(pub Pair);
    ///Container type for all return fields from the `pairsCount` function with signature `pairsCount()` and selector `0xb81b2b71`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairsCountReturn(pub ::ethers::core::types::U256);
    ///`Addresses(address,address,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Addresses {
        pub gns: ::ethers::core::types::Address,
        pub gns_staking: ::ethers::core::types::Address,
        pub treasury: ::ethers::core::types::Address,
    }
    ///`FeeGroup(uint40,uint40,uint40,uint32,uint104)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FeeGroup {
        pub total_position_size_fee_p: u64,
        pub total_liq_collateral_fee_p: u64,
        pub oracle_position_size_fee_p: u64,
        pub min_position_size_usd: u32,
        pub placeholder: u128,
    }
    ///`Feed(address,address,uint8,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Feed {
        pub feed_1: ::ethers::core::types::Address,
        pub feed_2: ::ethers::core::types::Address,
        pub feed_calculation: u8,
        pub max_deviation_p: ::ethers::core::types::U256,
    }
    ///`GlobalTradeFeeParams(uint24,uint24,uint24,uint24,uint24,uint136)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GlobalTradeFeeParams {
        pub referral_fee_p: u32,
        pub gov_fee_p: u32,
        pub trigger_order_fee_p: u32,
        pub gns_otc_fee_p: u32,
        pub g_token_fee_p: u32,
        pub placeholder: ::ethers::core::types::U256,
    }
    ///`Group(string,bytes32,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Group {
        pub name: ::std::string::String,
        pub job: [u8; 32],
        pub min_leverage: ::ethers::core::types::U256,
        pub max_leverage: ::ethers::core::types::U256,
    }
    ///`GroupLiquidationParams(uint40,uint40,uint40,uint24,uint24)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GroupLiquidationParams {
        pub max_liq_spread_p: u64,
        pub start_liq_threshold_p: u64,
        pub end_liq_threshold_p: u64,
        pub start_leverage: u32,
        pub end_leverage: u32,
    }
    ///`Pair(string,string,(address,address,uint8,uint256),uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Pair {
        pub from: ::std::string::String,
        pub to: ::std::string::String,
        pub feed: Feed,
        pub spread_p: ::ethers::core::types::U256,
        pub group_index: ::ethers::core::types::U256,
        pub fee_index: ::ethers::core::types::U256,
    }
}
