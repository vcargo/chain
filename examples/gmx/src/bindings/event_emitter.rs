pub use event_emitter::*;
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
pub mod event_emitter {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_roleStore"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract RoleStore"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("emitDataLog1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitDataLog1"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emitDataLog2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitDataLog2"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emitDataLog3"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitDataLog3"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic3"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emitDataLog4"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitDataLog4"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic3"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic4"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emitEventLog"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitEventLog"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::String,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventUtils.EventLogData",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emitEventLog1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitEventLog1"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::String,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventUtils.EventLogData",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("emitEventLog2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("emitEventLog2"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("topic2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("eventData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::String,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct EventUtils.EventLogData",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("roleStore"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("roleStore"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract RoleStore"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EventLog"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EventLog"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventNameHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::String,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EventLog1"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EventLog1"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventNameHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("topic1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::String,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EventLog2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EventLog2"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventName"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventNameHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("topic1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("topic2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("eventData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                                        ::std::boxed::Box::new(
                                                                            ::ethers::core::abi::ethabi::ParamType::String,
                                                                        ),
                                                                    ),
                                                                ],
                                                            ),
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("role"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EVENTEMITTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x1F\xCF8\x03\x80a\x1F\xCF\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x1F>a\0\x91`\09`\0\x81\x81`\x97\x01Ra\x03h\x01Ra\x1F>`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80c$\xDE\x01\xE4\x14a\0}W\x80cJJ{\x04\x14a\0\x92W\x80cc\xD1cc\x14a\0\xD5W\x80c\x90lI\xF6\x14a\0\xE8W\x80c\xB3\xAC\x1C8\x14a\0\xFBW\x80c\xDD\xA0\xDB2\x14a\x01\x0EW\x80c\xEE(\x8C\xE8\x14a\x01!W\x80c\xF9\xD5\xC0\xEA\x14a\x014W[`\0\x80\xFD[a\0\x90a\0\x8B6`\x04a\x12|V[a\x01GV[\0[a\0\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\x90a\0\xE36`\x04a\x12\xE8V[a\x01\xEEV[a\0\x90a\0\xF66`\x04a\x13^V[a\x02YV[a\0\x90a\x01\t6`\x04a\x13\xC1V[a\x02\xC0V[a\0\x90a\x01\x1C6`\x04a\x14\x0EV[a\x02\xE4V[a\0\x90a\x01/6`\x04a\x14SV[a\x03\x06V[a\0\x90a\x01B6`\x04a\x14\xB6V[a\x03,V[a\x01\x97`@Q` \x01a\x01Y\x90a\x14\xF2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x03LV[\x81\x83`@Qa\x01\xA6\x91\x90a\x15:V[`@Q\x80\x91\x03\x90 \x7F\x13zD\x06|\x89a\xCD~\x1D\x87oGT\xA5\xA3\xA7Y\x89\xB4U/\x18C\xFCi\xC3\xB3r\xDE\xF1`3\x86\x85`@Qa\x01\xE1\x93\x92\x91\x90a\x1D\xD2V[`@Q\x80\x91\x03\x90\xA3PPPV[a\x02\0`@Q` \x01a\x01Y\x90a\x14\xF2V[\x81\x83\x85`@Qa\x02\x10\x91\x90a\x15:V[`@Q\x80\x91\x03\x90 \x7FF\x8A%\xA7\xBAbL\xEE\xA6\xE5@\xADoI\x17\x1BRI[d\x84\x17\xAE\x91\xBC\xA2\x16v\xD8\xA2M\xC53\x88\x86`@Qa\x02K\x93\x92\x91\x90a\x1D\xD2V[`@Q\x80\x91\x03\x90\xA4PPPPV[a\x02k`@Q` \x01a\x01Y\x90a\x14\xF2V[\x81`@Qa\x02y\x91\x90a\x15:V[`@Q\x80\x91\x03\x90 \x7F~;\xDE+\xA7\xAC\xA4\xA8I\x96\x08\xCAW\xF3\xB0\xC1\xC1\xC9:\xCEc\xFF\xD3t\x1A\x9F\xAB AF\xFC\x9A3\x84\x84`@Qa\x02\xB4\x93\x92\x91\x90a\x1D\xD2V[`@Q\x80\x91\x03\x90\xA2PPV[a\x02\xD2`@Q` \x01a\x01Y\x90a\x14\xF2V[\x80Q\x82\x84\x86\x83` \x86\x01\xA3PPPPPV[a\x02\xF6`@Q` \x01a\x01Y\x90a\x14\xF2V[\x80Q\x82\x84\x82` \x85\x01\xA2PPPPV[a\x03\x18`@Q` \x01a\x01Y\x90a\x14\xF2V[\x80Q\x82\x84\x86\x88\x84` \x87\x01\xA4PPPPPPV[a\x03>`@Q` \x01a\x01Y\x90a\x14\xF2V[\x80Q\x82\x81` \x84\x01\xA1PPPV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDB\x91\x90a\x1E\xB8V[a\x04\x05W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x03\xFC\x92\x91\x90a\x1E\xDCV[`@Q\x80\x91\x03\x90\xFD[PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04AWa\x04Aa\x04\tV[`@R\x90V[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04AWa\x04Aa\x04\tV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04\x91Wa\x04\x91a\x04\tV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x04\xAAW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x04\xC3Wa\x04\xC3a\x04\tV[a\x04\xD6`\x1F\x82\x01`\x1F\x19\x16` \x01a\x04iV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x04\xEBW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x05!Wa\x05!a\x04\tV[P`\x05\x1B` \x01\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05BW`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x05XW`\0\x80\xFD[\x815` a\x05ma\x05h\x83a\x05\x08V[a\x04iV[\x82\x81R`\x05\x92\x83\x1B\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x05\x8CW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x06\x88W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x05\xB0W`\0\x80\x81\xFD[\x90\x89\x01\x90`@\x82\x8C\x03`\x1F\x19\x01\x81\x13\x15a\x05\xCAW`\0\x80\x81\xFD[a\x05\xD2a\x04\x1FV[\x88\x84\x015\x83\x81\x11\x15a\x05\xE4W`\0\x80\x81\xFD[a\x05\xF2\x8E\x8B\x83\x88\x01\x01a\x04\x99V[\x82RP\x81\x84\x015\x83\x81\x11\x15a\x06\x07W`\0\x80\x81\xFD[\x80\x85\x01\x94PP\x8C`?\x85\x01\x12a\x06\x1FW`\0\x92P\x82\x83\xFD[\x88\x84\x015\x92Pa\x061a\x05h\x84a\x05\x08V[\x83\x81R\x92\x86\x1B\x84\x01\x82\x01\x92\x89\x81\x01\x90\x8E\x85\x11\x15a\x06NW`\0\x80\x81\xFD[\x94\x83\x01\x94[\x84\x86\x10\x15a\x06sWa\x06d\x86a\x05+V[\x82R\x94\x8A\x01\x94\x90\x8A\x01\x90a\x06SV[\x82\x8B\x01RP\x87RPPP\x92\x84\x01\x92\x84\x01a\x05\x90V[P\x90\x97\x96PPPPPPPV[`\0`@\x80\x83\x85\x03\x12\x15a\x06\xA8W`\0\x80\xFD[a\x06\xB0a\x04\x1FV[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x06\xC9W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x06\xDDW`\0\x80\xFD[\x815` a\x06\xEDa\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x07\x0CW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x07\x86W\x805\x86\x81\x11\x15a\x07(W`\0\x80\x81\xFD[\x87\x01\x80\x8C\x03`\x1F\x19\x01\x89\x13\x15a\x07>W`\0\x80\x81\xFD[a\x07Fa\x04\x1FV[\x85\x82\x015\x88\x81\x11\x15a\x07XW`\0\x80\x81\xFD[a\x07f\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82RPa\x07t\x8A\x83\x01a\x05+V[\x81\x87\x01R\x84RP\x91\x83\x01\x91\x83\x01a\x07\x10V[P\x87RP\x86\x81\x015\x94P\x82\x85\x11\x15a\x07\x9DW`\0\x80\xFD[a\x07\xA9\x88\x86\x89\x01a\x05GV[\x81\x87\x01RPPPPP\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x07\xC9W`\0\x80\xFD[\x815` a\x07\xD9a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x83\x1B\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x07\xF8W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x06\x88W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x08\x1CW`\0\x80\x81\xFD[\x90\x89\x01\x90`@\x82\x8C\x03`\x1F\x19\x01\x81\x13\x15a\x086W`\0\x80\x81\xFD[a\x08>a\x04\x1FV[\x88\x84\x015\x83\x81\x11\x15a\x08PW`\0\x80\x81\xFD[a\x08^\x8E\x8B\x83\x88\x01\x01a\x04\x99V[\x82RP\x81\x84\x015\x83\x81\x11\x15a\x08sW`\0\x80\x81\xFD[\x80\x85\x01\x94PP\x8C`?\x85\x01\x12a\x08\x8BW`\0\x92P\x82\x83\xFD[\x88\x84\x015\x92Pa\x08\x9Da\x05h\x84a\x05\x08V[\x83\x81R\x92\x86\x1B\x84\x01\x82\x01\x92\x89\x81\x01\x90\x8E\x85\x11\x15a\x08\xBAW`\0\x80\x81\xFD[\x94\x83\x01\x94[\x84\x86\x10\x15a\x08\xD8W\x855\x82R\x94\x8A\x01\x94\x90\x8A\x01\x90a\x08\xBFV[\x82\x8B\x01RP\x87RPPP\x92\x84\x01\x92\x84\x01a\x07\xFCV[`\0`@\x80\x83\x85\x03\x12\x15a\t\0W`\0\x80\xFD[a\t\x08a\x04\x1FV[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\t!W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\t5W`\0\x80\xFD[\x815` a\tEa\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\tdW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\t\xD6W\x805\x86\x81\x11\x15a\t\x80W`\0\x80\x81\xFD[\x87\x01\x80\x8C\x03`\x1F\x19\x01\x89\x13\x15a\t\x96W`\0\x80\x81\xFD[a\t\x9Ea\x04\x1FV[\x85\x82\x015\x88\x81\x11\x15a\t\xB0W`\0\x80\x81\xFD[a\t\xBE\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82RP\x90\x89\x015\x85\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\thV[P\x87RP\x86\x81\x015\x94P\x82\x85\x11\x15a\t\xEDW`\0\x80\xFD[a\x07\xA9\x88\x86\x89\x01a\x07\xB8V[\x80\x15\x15\x81\x14a\n\x07W`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\n\x1BW`\0\x80\xFD[\x815` a\n+a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x83\x1B\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\nJW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x06\x88W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\nnW`\0\x80\x81\xFD[\x90\x89\x01\x90`@\x82\x8C\x03`\x1F\x19\x01\x81\x13\x15a\n\x88W`\0\x80\x81\xFD[a\n\x90a\x04\x1FV[\x88\x84\x015\x83\x81\x11\x15a\n\xA2W`\0\x80\x81\xFD[a\n\xB0\x8E\x8B\x83\x88\x01\x01a\x04\x99V[\x82RP\x81\x84\x015\x83\x81\x11\x15a\n\xC5W`\0\x80\x81\xFD[\x80\x85\x01\x94PP\x8C`?\x85\x01\x12a\n\xDDW`\0\x92P\x82\x83\xFD[\x88\x84\x015\x92Pa\n\xEFa\x05h\x84a\x05\x08V[\x83\x81R\x92\x86\x1B\x84\x01\x82\x01\x92\x89\x81\x01\x90\x8E\x85\x11\x15a\x0B\x0CW`\0\x80\x81\xFD[\x94\x83\x01\x94[\x84\x86\x10\x15a\x0B6W\x855\x93Pa\x0B&\x84a\t\xF9V[\x83\x82R\x94\x8A\x01\x94\x90\x8A\x01\x90a\x0B\x11V[\x82\x8B\x01RP\x87RPPP\x92\x84\x01\x92\x84\x01a\nNV[`\0`@\x80\x83\x85\x03\x12\x15a\x0B^W`\0\x80\xFD[a\x0Bfa\x04\x1FV[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0B\x7FW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B\x93W`\0\x80\xFD[\x815` a\x0B\xA3a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x0B\xC2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0CAW\x805\x86\x81\x11\x15a\x0B\xDEW`\0\x80\x81\xFD[\x87\x01\x80\x8C\x03`\x1F\x19\x01\x89\x13\x15a\x0B\xF4W`\0\x80\x81\xFD[a\x0B\xFCa\x04\x1FV[\x85\x82\x015\x88\x81\x11\x15a\x0C\x0EW`\0\x80\x81\xFD[a\x0C\x1C\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82RP\x90\x89\x015\x90a\x0C-\x82a\t\xF9V[\x80\x86\x01\x91\x90\x91R\x83R\x91\x83\x01\x91\x83\x01a\x0B\xC6V[P\x87RP\x86\x81\x015\x94P\x82\x85\x11\x15a\x0CXW`\0\x80\xFD[a\x07\xA9\x88\x86\x89\x01a\n\nV[`\0\x82`\x1F\x83\x01\x12a\x0CuW`\0\x80\xFD[\x815` a\x0C\x85a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0C\xA4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\r\xADW`\x01`\x01`@\x1B\x03\x815\x81\x10\x15a\x0C\xC6W`\0\x80\xFD[\x815\x88\x01`@\x81\x8B\x03`\x1F\x19\x01\x12\x15a\x0C\xDEW`\0\x80\xFD[a\x0C\xE6a\x04\x1FV[\x86\x82\x015\x83\x81\x11\x15a\x0C\xF7W`\0\x80\xFD[a\r\x05\x8C\x89\x83\x86\x01\x01a\x04\x99V[\x82RP`@\x82\x015\x83\x81\x11\x15a\r\x1AW`\0\x80\xFD[\x80\x83\x01\x92PP\x8A`?\x83\x01\x12a\r/W`\0\x80\xFD[\x86\x82\x015a\r?a\x05h\x82a\x05\x08V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01`@\x01\x90\x88\x81\x01\x90\x8D\x83\x11\x15a\r_W`\0\x80\xFD[`@\x85\x01[\x83\x81\x10\x15a\r\x97W\x86\x815\x11\x15a\rzW`\0\x80\xFD[a\r\x8A\x8F`@\x835\x89\x01\x01a\x04\x99V[\x83R\x91\x8A\x01\x91\x8A\x01a\rdV[P\x83\x8A\x01RPP\x85RPP\x91\x83\x01\x91\x83\x01a\x0C\xA8V[P\x96\x95PPPPPPV[`\0`@\x80\x83\x85\x03\x12\x15a\r\xCBW`\0\x80\xFD[a\r\xD3a\x04\x1FV[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\r\xECW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0E\0W`\0\x80\xFD[\x815` a\x0E\x10a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x0E/W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0E\xBFW\x805\x86\x81\x11\x15a\x0EKW`\0\x80\x81\xFD[\x87\x01\x80\x8C\x03`\x1F\x19\x01\x89\x13\x15a\x0EaW`\0\x80\x81\xFD[a\x0Eia\x04\x1FV[\x85\x82\x015\x88\x81\x11\x15a\x0E{W`\0\x80\x81\xFD[a\x0E\x89\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82RP\x89\x82\x015\x88\x81\x11\x15a\x0E\x9EW`\0\x80\x81\xFD[a\x0E\xAC\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82\x88\x01RP\x84RP\x91\x83\x01\x91\x83\x01a\x0E3V[P\x87RP\x86\x81\x015\x94P\x82\x85\x11\x15a\x0E\xD6W`\0\x80\xFD[a\x07\xA9\x88\x86\x89\x01a\x0CdV[`\0\x82`\x1F\x83\x01\x12a\x0E\xF3W`\0\x80\xFD[\x815` a\x0F\x03a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0F\"W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\r\xADW`\x01`\x01`@\x1B\x03\x815\x81\x10\x15a\x0FDW`\0\x80\xFD[\x815\x88\x01`@\x81\x8B\x03`\x1F\x19\x01\x12\x15a\x0F\\W`\0\x80\xFD[a\x0Fda\x04\x1FV[\x86\x82\x015\x83\x81\x11\x15a\x0FuW`\0\x80\xFD[a\x0F\x83\x8C\x89\x83\x86\x01\x01a\x04\x99V[\x82RP`@\x82\x015\x83\x81\x11\x15a\x0F\x98W`\0\x80\xFD[\x80\x83\x01\x92PP\x8A`?\x83\x01\x12a\x0F\xADW`\0\x80\xFD[\x86\x82\x015a\x0F\xBDa\x05h\x82a\x05\x08V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01`@\x01\x90\x88\x81\x01\x90\x8D\x83\x11\x15a\x0F\xDDW`\0\x80\xFD[`@\x85\x01[\x83\x81\x10\x15a\x10\x15W\x86\x815\x11\x15a\x0F\xF8W`\0\x80\xFD[a\x10\x08\x8F`@\x835\x89\x01\x01a\x04\x99V[\x83R\x91\x8A\x01\x91\x8A\x01a\x0F\xE2V[P\x83\x8A\x01RPP\x85RPP\x91\x83\x01\x91\x83\x01a\x0F&V[`\0`@\x80\x83\x85\x03\x12\x15a\x10>W`\0\x80\xFD[a\x10Fa\x04\x1FV[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10_W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x10sW`\0\x80\xFD[\x815` a\x10\x83a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x10\xA2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x112W\x805\x86\x81\x11\x15a\x10\xBEW`\0\x80\x81\xFD[\x87\x01\x80\x8C\x03`\x1F\x19\x01\x89\x13\x15a\x10\xD4W`\0\x80\x81\xFD[a\x10\xDCa\x04\x1FV[\x85\x82\x015\x88\x81\x11\x15a\x10\xEEW`\0\x80\x81\xFD[a\x10\xFC\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82RP\x89\x82\x015\x88\x81\x11\x15a\x11\x11W`\0\x80\x81\xFD[a\x11\x1F\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82\x88\x01RP\x84RP\x91\x83\x01\x91\x83\x01a\x10\xA6V[P\x87RP\x86\x81\x015\x94P\x82\x85\x11\x15a\x11IW`\0\x80\xFD[a\x07\xA9\x88\x86\x89\x01a\x0E\xE2V[`\0`\xE0\x82\x84\x03\x12\x15a\x11gW`\0\x80\xFD[a\x11oa\x04GV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\x88W`\0\x80\xFD[a\x11\x94\x85\x83\x86\x01a\x06\x95V[\x83R` \x84\x015\x91P\x80\x82\x11\x15a\x11\xAAW`\0\x80\xFD[a\x11\xB6\x85\x83\x86\x01a\x08\xEDV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x11\xCFW`\0\x80\xFD[a\x11\xDB\x85\x83\x86\x01a\x08\xEDV[`@\x84\x01R``\x84\x015\x91P\x80\x82\x11\x15a\x11\xF4W`\0\x80\xFD[a\x12\0\x85\x83\x86\x01a\x0BKV[``\x84\x01R`\x80\x84\x015\x91P\x80\x82\x11\x15a\x12\x19W`\0\x80\xFD[a\x12%\x85\x83\x86\x01a\x08\xEDV[`\x80\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15a\x12>W`\0\x80\xFD[a\x12J\x85\x83\x86\x01a\r\xB8V[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15a\x12cW`\0\x80\xFD[Pa\x12p\x84\x82\x85\x01a\x10+V[`\xC0\x83\x01RP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\x91W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x12\xA8W`\0\x80\xFD[a\x12\xB4\x87\x83\x88\x01a\x04\x99V[\x94P` \x86\x015\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x12\xD1W`\0\x80\xFD[Pa\x12\xDE\x86\x82\x87\x01a\x11UV[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\xFEW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\x15W`\0\x80\xFD[a\x13!\x88\x83\x89\x01a\x04\x99V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x13EW`\0\x80\xFD[Pa\x13R\x87\x82\x88\x01a\x11UV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x13qW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\x88W`\0\x80\xFD[a\x13\x94\x86\x83\x87\x01a\x04\x99V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x13\xAAW`\0\x80\xFD[Pa\x13\xB7\x85\x82\x86\x01a\x11UV[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x13\xD7W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x02W`\0\x80\xFD[a\x13R\x87\x82\x88\x01a\x04\x99V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14#W`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14GW`\0\x80\xFD[a\x12\xDE\x86\x82\x87\x01a\x04\x99V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x14kW`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x9DW`\0\x80\xFD[a\x14\xA9\x88\x82\x89\x01a\x04\x99V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xC9W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xE6W`\0\x80\xFD[a\x13\xB7\x85\x82\x86\x01a\x04\x99V[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`\0[\x83\x81\x10\x15a\x151W\x81\x81\x01Q\x83\x82\x01R` \x01a\x15\x19V[PP`\0\x91\x01RV[`\0\x82Qa\x15L\x81\x84` \x87\x01a\x15\x16V[\x91\x90\x91\x01\x92\x91PPV[`\0\x81Q\x80\x84Ra\x15n\x81` \x86\x01` \x86\x01a\x15\x16V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x16\x1DW\x83\x85\x03\x8AR\x82Q`@\x81Q\x81\x88Ra\x15\xC3\x82\x89\x01\x82a\x15VV[\x92\x89\x01Q\x88\x84\x03\x89\x8B\x01R\x80Q\x80\x85R\x90\x8A\x01\x93\x86\x93P\x8A\x01\x91P[\x80\x83\x10\x15a\x16\x08W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x82R\x92\x89\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x89\x01\x90a\x15\xDFV[P\x9B\x88\x01\x9B\x96PPP\x91\x85\x01\x91`\x01\x01a\x15\xA1V[P\x92\x98\x97PPPPPPPPV[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x16\xA2W\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x16|\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x97\x84\x01\x97\x90\x97R\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x16XV[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x15\x82V[\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x16\x1DW\x83\x85\x03\x8AR\x82Q`@\x81Q\x81\x88Ra\x17\x08\x82\x89\x01\x82a\x15VV[\x92\x89\x01Q\x88\x84\x03\x89\x8B\x01R\x80Q\x80\x85R\x90\x8A\x01\x93\x86\x93P\x8A\x01\x91P[\x80\x83\x10\x15a\x17DW\x83Q\x82R\x92\x89\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x89\x01\x90a\x17$V[P\x9B\x88\x01\x9B\x96PPP\x91\x85\x01\x91`\x01\x01a\x16\xE6V[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x17\xC7W\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x17\xAA\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x97\x84\x01\x97\x90\x97R\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x17\x86V[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x16\xC7V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x16\x1DW\x83\x85\x03\x8AR\x82Q`@\x81Q\x81\x88Ra\x18$\x82\x89\x01\x82a\x15VV[\x92\x89\x01Q\x88\x84\x03\x89\x8B\x01R\x80Q\x80\x85R\x90\x8A\x01\x93\x86\x93P\x8A\x01\x91P[\x80\x83\x10\x15a\x18`W\x83Q\x82R\x92\x89\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x89\x01\x90a\x18@V[P\x9B\x88\x01\x9B\x96PPP\x91\x85\x01\x91`\x01\x01a\x18\x02V[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x18\xE3W\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x18\xC6\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x97\x84\x01\x97\x90\x97R\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x18\xA2V[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x17\xE3V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x16\x1DW\x83\x85\x03\x8AR\x82Q`@\x81Q\x81\x88Ra\x19@\x82\x89\x01\x82a\x15VV[\x92\x89\x01Q\x88\x84\x03\x89\x8B\x01R\x80Q\x80\x85R\x90\x8A\x01\x93\x86\x93P\x8A\x01\x91P[\x80\x83\x10\x15a\x19~W\x83Q\x15\x15\x82R\x92\x89\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x89\x01\x90a\x19\\V[P\x9B\x88\x01\x9B\x96PPP\x91\x85\x01\x91`\x01\x01a\x19\x1EV[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x1A\x03W\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x19\xE4\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x15\x15\x97\x84\x01\x97\x90\x97R\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x19\xC0V[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x18\xFFV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x16\x1DW\x83\x85\x03\x8AR\x82Q`@\x81Q\x81\x88Ra\x1A`\x82\x89\x01\x82a\x15VV[\x92\x89\x01Q\x88\x84\x03\x89\x8B\x01R\x80Q\x80\x85R\x90\x8A\x01\x93\x86\x93P\x8A\x01\x91P[\x80\x83\x10\x15a\x1A\x9CW\x83Q\x82R\x92\x89\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x89\x01\x90a\x1A|V[P\x9B\x88\x01\x9B\x96PPP\x91\x85\x01\x91`\x01\x01a\x1A>V[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x1B\x1FW\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x1B\x02\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x97\x84\x01\x97\x90\x97R\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x1A\xDEV[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x1A\x1FV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P`\x05\x91P\x83\x82\x1B\x81\x01\x83\x87\x01`\0\x80[\x87\x81\x10\x15a\x1B\xE6W\x84\x84\x03\x8BR\x82Q`@\x81Q\x81\x87Ra\x1B}\x82\x88\x01\x82a\x15VV[\x92\x8A\x01Q\x87\x84\x03\x88\x8C\x01R\x80Q\x80\x85R\x90\x8B\x01\x93\x92P\x8A\x83\x01\x91P\x80\x8A\x1B\x83\x01\x8B\x01\x86[\x82\x81\x10\x15a\x1B\xCFW`\x1F\x19\x85\x83\x03\x01\x84Ra\x1B\xBD\x82\x87Qa\x15VV[\x95\x8D\x01\x95\x93\x8D\x01\x93\x91P`\x01\x01a\x1B\xA1V[P\x9E\x8B\x01\x9E\x97PPP\x93\x88\x01\x93PP`\x01\x01a\x1B[V[P\x91\x99\x98PPPPPPPPPV[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x1CrW\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x1CF\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x88\x83\x03\x89\x86\x01R\x91\x90Pa\x1C^\x81\x83a\x15VV[\x97PPP\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x1C\"V[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x1B;V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P`\x05\x91P\x83\x82\x1B\x81\x01\x83\x87\x01`\0\x80[\x87\x81\x10\x15a\x1B\xE6W\x84\x84\x03\x8BR\x82Q`@\x81Q\x81\x87Ra\x1C\xD0\x82\x88\x01\x82a\x15VV[\x92\x8A\x01Q\x87\x84\x03\x88\x8C\x01R\x80Q\x80\x85R\x90\x8B\x01\x93\x92P\x8A\x83\x01\x91P\x80\x8A\x1B\x83\x01\x8B\x01\x86[\x82\x81\x10\x15a\x1D\"W`\x1F\x19\x85\x83\x03\x01\x84Ra\x1D\x10\x82\x87Qa\x15VV[\x95\x8D\x01\x95\x93\x8D\x01\x93\x91P`\x01\x01a\x1C\xF4V[P\x9E\x8B\x01\x9E\x97PPP\x93\x88\x01\x93PP`\x01\x01a\x1C\xAEV[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x1D\xB6W\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x1D\x8A\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x88\x83\x03\x89\x86\x01R\x91\x90Pa\x1D\xA2\x81\x83a\x15VV[\x97PPP\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x1DfV[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x1C\x8EV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R`\0\x90a\x1D\xF6\x90\x83\x01\x85a\x15VV[\x82\x81\x03`@\x84\x01R\x83Q`\xE0\x82Ra\x1E\x11`\xE0\x83\x01\x82a\x16+V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01Ra\x1E*\x82\x82a\x17YV[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra\x1ED\x82\x82a\x18uV[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01Ra\x1E^\x82\x82a\x19\x93V[\x91PP`\x80\x85\x01Q\x82\x82\x03`\x80\x84\x01Ra\x1Ex\x82\x82a\x1A\xB1V[\x91PP`\xA0\x85\x01Q\x82\x82\x03`\xA0\x84\x01Ra\x1E\x92\x82\x82a\x1B\xF5V[\x91PP`\xC0\x85\x01Q\x82\x82\x03`\xC0\x84\x01Ra\x1E\xAC\x82\x82a\x1D9V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1E\xCAW`\0\x80\xFD[\x81Qa\x1E\xD5\x81a\t\xF9V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x1F\0\x90\x83\x01\x84a\x15VV[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xD3s\x13S\xAF\xC5Eb\xDE\xAB\xB0E\xFA\\\xE9_\x9E\xFD\xD0\xB3z\x17\xD2\x86_\x0F*/\x0C#\x13\x07dsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static EVENTEMITTER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0xW`\x005`\xE0\x1C\x80c$\xDE\x01\xE4\x14a\0}W\x80cJJ{\x04\x14a\0\x92W\x80cc\xD1cc\x14a\0\xD5W\x80c\x90lI\xF6\x14a\0\xE8W\x80c\xB3\xAC\x1C8\x14a\0\xFBW\x80c\xDD\xA0\xDB2\x14a\x01\x0EW\x80c\xEE(\x8C\xE8\x14a\x01!W\x80c\xF9\xD5\xC0\xEA\x14a\x014W[`\0\x80\xFD[a\0\x90a\0\x8B6`\x04a\x12|V[a\x01GV[\0[a\0\xB9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\x90a\0\xE36`\x04a\x12\xE8V[a\x01\xEEV[a\0\x90a\0\xF66`\x04a\x13^V[a\x02YV[a\0\x90a\x01\t6`\x04a\x13\xC1V[a\x02\xC0V[a\0\x90a\x01\x1C6`\x04a\x14\x0EV[a\x02\xE4V[a\0\x90a\x01/6`\x04a\x14SV[a\x03\x06V[a\0\x90a\x01B6`\x04a\x14\xB6V[a\x03,V[a\x01\x97`@Q` \x01a\x01Y\x90a\x14\xF2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q\x80`@\x01`@R\x80`\n\x81R` \x01i!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B\x81RPa\x03LV[\x81\x83`@Qa\x01\xA6\x91\x90a\x15:V[`@Q\x80\x91\x03\x90 \x7F\x13zD\x06|\x89a\xCD~\x1D\x87oGT\xA5\xA3\xA7Y\x89\xB4U/\x18C\xFCi\xC3\xB3r\xDE\xF1`3\x86\x85`@Qa\x01\xE1\x93\x92\x91\x90a\x1D\xD2V[`@Q\x80\x91\x03\x90\xA3PPPV[a\x02\0`@Q` \x01a\x01Y\x90a\x14\xF2V[\x81\x83\x85`@Qa\x02\x10\x91\x90a\x15:V[`@Q\x80\x91\x03\x90 \x7FF\x8A%\xA7\xBAbL\xEE\xA6\xE5@\xADoI\x17\x1BRI[d\x84\x17\xAE\x91\xBC\xA2\x16v\xD8\xA2M\xC53\x88\x86`@Qa\x02K\x93\x92\x91\x90a\x1D\xD2V[`@Q\x80\x91\x03\x90\xA4PPPPV[a\x02k`@Q` \x01a\x01Y\x90a\x14\xF2V[\x81`@Qa\x02y\x91\x90a\x15:V[`@Q\x80\x91\x03\x90 \x7F~;\xDE+\xA7\xAC\xA4\xA8I\x96\x08\xCAW\xF3\xB0\xC1\xC1\xC9:\xCEc\xFF\xD3t\x1A\x9F\xAB AF\xFC\x9A3\x84\x84`@Qa\x02\xB4\x93\x92\x91\x90a\x1D\xD2V[`@Q\x80\x91\x03\x90\xA2PPV[a\x02\xD2`@Q` \x01a\x01Y\x90a\x14\xF2V[\x80Q\x82\x84\x86\x83` \x86\x01\xA3PPPPPV[a\x02\xF6`@Q` \x01a\x01Y\x90a\x14\xF2V[\x80Q\x82\x84\x82` \x85\x01\xA2PPPPV[a\x03\x18`@Q` \x01a\x01Y\x90a\x14\xF2V[\x80Q\x82\x84\x86\x88\x84` \x87\x01\xA4PPPPPPV[a\x03>`@Q` \x01a\x01Y\x90a\x14\xF2V[\x80Q\x82\x81` \x84\x01\xA1PPPV[`@Qc\xACJ\xB3\xFB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xACJ\xB3\xFB\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDB\x91\x90a\x1E\xB8V[a\x04\x05W3\x81`@Qc\xA3[\x15\x0B`\xE0\x1B\x81R`\x04\x01a\x03\xFC\x92\x91\x90a\x1E\xDCV[`@Q\x80\x91\x03\x90\xFD[PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04AWa\x04Aa\x04\tV[`@R\x90V[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04AWa\x04Aa\x04\tV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x04\x91Wa\x04\x91a\x04\tV[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x04\xAAW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x04\xC3Wa\x04\xC3a\x04\tV[a\x04\xD6`\x1F\x82\x01`\x1F\x19\x16` \x01a\x04iV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x04\xEBW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x05!Wa\x05!a\x04\tV[P`\x05\x1B` \x01\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05BW`\0\x80\xFD[\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\x05XW`\0\x80\xFD[\x815` a\x05ma\x05h\x83a\x05\x08V[a\x04iV[\x82\x81R`\x05\x92\x83\x1B\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x05\x8CW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x06\x88W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x05\xB0W`\0\x80\x81\xFD[\x90\x89\x01\x90`@\x82\x8C\x03`\x1F\x19\x01\x81\x13\x15a\x05\xCAW`\0\x80\x81\xFD[a\x05\xD2a\x04\x1FV[\x88\x84\x015\x83\x81\x11\x15a\x05\xE4W`\0\x80\x81\xFD[a\x05\xF2\x8E\x8B\x83\x88\x01\x01a\x04\x99V[\x82RP\x81\x84\x015\x83\x81\x11\x15a\x06\x07W`\0\x80\x81\xFD[\x80\x85\x01\x94PP\x8C`?\x85\x01\x12a\x06\x1FW`\0\x92P\x82\x83\xFD[\x88\x84\x015\x92Pa\x061a\x05h\x84a\x05\x08V[\x83\x81R\x92\x86\x1B\x84\x01\x82\x01\x92\x89\x81\x01\x90\x8E\x85\x11\x15a\x06NW`\0\x80\x81\xFD[\x94\x83\x01\x94[\x84\x86\x10\x15a\x06sWa\x06d\x86a\x05+V[\x82R\x94\x8A\x01\x94\x90\x8A\x01\x90a\x06SV[\x82\x8B\x01RP\x87RPPP\x92\x84\x01\x92\x84\x01a\x05\x90V[P\x90\x97\x96PPPPPPPV[`\0`@\x80\x83\x85\x03\x12\x15a\x06\xA8W`\0\x80\xFD[a\x06\xB0a\x04\x1FV[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x06\xC9W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x06\xDDW`\0\x80\xFD[\x815` a\x06\xEDa\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x07\x0CW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x07\x86W\x805\x86\x81\x11\x15a\x07(W`\0\x80\x81\xFD[\x87\x01\x80\x8C\x03`\x1F\x19\x01\x89\x13\x15a\x07>W`\0\x80\x81\xFD[a\x07Fa\x04\x1FV[\x85\x82\x015\x88\x81\x11\x15a\x07XW`\0\x80\x81\xFD[a\x07f\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82RPa\x07t\x8A\x83\x01a\x05+V[\x81\x87\x01R\x84RP\x91\x83\x01\x91\x83\x01a\x07\x10V[P\x87RP\x86\x81\x015\x94P\x82\x85\x11\x15a\x07\x9DW`\0\x80\xFD[a\x07\xA9\x88\x86\x89\x01a\x05GV[\x81\x87\x01RPPPPP\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x07\xC9W`\0\x80\xFD[\x815` a\x07\xD9a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x83\x1B\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\x07\xF8W`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x06\x88W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x08\x1CW`\0\x80\x81\xFD[\x90\x89\x01\x90`@\x82\x8C\x03`\x1F\x19\x01\x81\x13\x15a\x086W`\0\x80\x81\xFD[a\x08>a\x04\x1FV[\x88\x84\x015\x83\x81\x11\x15a\x08PW`\0\x80\x81\xFD[a\x08^\x8E\x8B\x83\x88\x01\x01a\x04\x99V[\x82RP\x81\x84\x015\x83\x81\x11\x15a\x08sW`\0\x80\x81\xFD[\x80\x85\x01\x94PP\x8C`?\x85\x01\x12a\x08\x8BW`\0\x92P\x82\x83\xFD[\x88\x84\x015\x92Pa\x08\x9Da\x05h\x84a\x05\x08V[\x83\x81R\x92\x86\x1B\x84\x01\x82\x01\x92\x89\x81\x01\x90\x8E\x85\x11\x15a\x08\xBAW`\0\x80\x81\xFD[\x94\x83\x01\x94[\x84\x86\x10\x15a\x08\xD8W\x855\x82R\x94\x8A\x01\x94\x90\x8A\x01\x90a\x08\xBFV[\x82\x8B\x01RP\x87RPPP\x92\x84\x01\x92\x84\x01a\x07\xFCV[`\0`@\x80\x83\x85\x03\x12\x15a\t\0W`\0\x80\xFD[a\t\x08a\x04\x1FV[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\t!W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\t5W`\0\x80\xFD[\x815` a\tEa\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\tdW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\t\xD6W\x805\x86\x81\x11\x15a\t\x80W`\0\x80\x81\xFD[\x87\x01\x80\x8C\x03`\x1F\x19\x01\x89\x13\x15a\t\x96W`\0\x80\x81\xFD[a\t\x9Ea\x04\x1FV[\x85\x82\x015\x88\x81\x11\x15a\t\xB0W`\0\x80\x81\xFD[a\t\xBE\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82RP\x90\x89\x015\x85\x82\x01R\x83R\x91\x83\x01\x91\x83\x01a\thV[P\x87RP\x86\x81\x015\x94P\x82\x85\x11\x15a\t\xEDW`\0\x80\xFD[a\x07\xA9\x88\x86\x89\x01a\x07\xB8V[\x80\x15\x15\x81\x14a\n\x07W`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\n\x1BW`\0\x80\xFD[\x815` a\n+a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x83\x1B\x85\x01\x82\x01\x92\x82\x82\x01\x91\x90\x87\x85\x11\x15a\nJW`\0\x80\xFD[\x83\x87\x01[\x85\x81\x10\x15a\x06\x88W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\nnW`\0\x80\x81\xFD[\x90\x89\x01\x90`@\x82\x8C\x03`\x1F\x19\x01\x81\x13\x15a\n\x88W`\0\x80\x81\xFD[a\n\x90a\x04\x1FV[\x88\x84\x015\x83\x81\x11\x15a\n\xA2W`\0\x80\x81\xFD[a\n\xB0\x8E\x8B\x83\x88\x01\x01a\x04\x99V[\x82RP\x81\x84\x015\x83\x81\x11\x15a\n\xC5W`\0\x80\x81\xFD[\x80\x85\x01\x94PP\x8C`?\x85\x01\x12a\n\xDDW`\0\x92P\x82\x83\xFD[\x88\x84\x015\x92Pa\n\xEFa\x05h\x84a\x05\x08V[\x83\x81R\x92\x86\x1B\x84\x01\x82\x01\x92\x89\x81\x01\x90\x8E\x85\x11\x15a\x0B\x0CW`\0\x80\x81\xFD[\x94\x83\x01\x94[\x84\x86\x10\x15a\x0B6W\x855\x93Pa\x0B&\x84a\t\xF9V[\x83\x82R\x94\x8A\x01\x94\x90\x8A\x01\x90a\x0B\x11V[\x82\x8B\x01RP\x87RPPP\x92\x84\x01\x92\x84\x01a\nNV[`\0`@\x80\x83\x85\x03\x12\x15a\x0B^W`\0\x80\xFD[a\x0Bfa\x04\x1FV[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0B\x7FW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B\x93W`\0\x80\xFD[\x815` a\x0B\xA3a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x0B\xC2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0CAW\x805\x86\x81\x11\x15a\x0B\xDEW`\0\x80\x81\xFD[\x87\x01\x80\x8C\x03`\x1F\x19\x01\x89\x13\x15a\x0B\xF4W`\0\x80\x81\xFD[a\x0B\xFCa\x04\x1FV[\x85\x82\x015\x88\x81\x11\x15a\x0C\x0EW`\0\x80\x81\xFD[a\x0C\x1C\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82RP\x90\x89\x015\x90a\x0C-\x82a\t\xF9V[\x80\x86\x01\x91\x90\x91R\x83R\x91\x83\x01\x91\x83\x01a\x0B\xC6V[P\x87RP\x86\x81\x015\x94P\x82\x85\x11\x15a\x0CXW`\0\x80\xFD[a\x07\xA9\x88\x86\x89\x01a\n\nV[`\0\x82`\x1F\x83\x01\x12a\x0CuW`\0\x80\xFD[\x815` a\x0C\x85a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0C\xA4W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\r\xADW`\x01`\x01`@\x1B\x03\x815\x81\x10\x15a\x0C\xC6W`\0\x80\xFD[\x815\x88\x01`@\x81\x8B\x03`\x1F\x19\x01\x12\x15a\x0C\xDEW`\0\x80\xFD[a\x0C\xE6a\x04\x1FV[\x86\x82\x015\x83\x81\x11\x15a\x0C\xF7W`\0\x80\xFD[a\r\x05\x8C\x89\x83\x86\x01\x01a\x04\x99V[\x82RP`@\x82\x015\x83\x81\x11\x15a\r\x1AW`\0\x80\xFD[\x80\x83\x01\x92PP\x8A`?\x83\x01\x12a\r/W`\0\x80\xFD[\x86\x82\x015a\r?a\x05h\x82a\x05\x08V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01`@\x01\x90\x88\x81\x01\x90\x8D\x83\x11\x15a\r_W`\0\x80\xFD[`@\x85\x01[\x83\x81\x10\x15a\r\x97W\x86\x815\x11\x15a\rzW`\0\x80\xFD[a\r\x8A\x8F`@\x835\x89\x01\x01a\x04\x99V[\x83R\x91\x8A\x01\x91\x8A\x01a\rdV[P\x83\x8A\x01RPP\x85RPP\x91\x83\x01\x91\x83\x01a\x0C\xA8V[P\x96\x95PPPPPPV[`\0`@\x80\x83\x85\x03\x12\x15a\r\xCBW`\0\x80\xFD[a\r\xD3a\x04\x1FV[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\r\xECW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0E\0W`\0\x80\xFD[\x815` a\x0E\x10a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x0E/W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0E\xBFW\x805\x86\x81\x11\x15a\x0EKW`\0\x80\x81\xFD[\x87\x01\x80\x8C\x03`\x1F\x19\x01\x89\x13\x15a\x0EaW`\0\x80\x81\xFD[a\x0Eia\x04\x1FV[\x85\x82\x015\x88\x81\x11\x15a\x0E{W`\0\x80\x81\xFD[a\x0E\x89\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82RP\x89\x82\x015\x88\x81\x11\x15a\x0E\x9EW`\0\x80\x81\xFD[a\x0E\xAC\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82\x88\x01RP\x84RP\x91\x83\x01\x91\x83\x01a\x0E3V[P\x87RP\x86\x81\x015\x94P\x82\x85\x11\x15a\x0E\xD6W`\0\x80\xFD[a\x07\xA9\x88\x86\x89\x01a\x0CdV[`\0\x82`\x1F\x83\x01\x12a\x0E\xF3W`\0\x80\xFD[\x815` a\x0F\x03a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0F\"W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\r\xADW`\x01`\x01`@\x1B\x03\x815\x81\x10\x15a\x0FDW`\0\x80\xFD[\x815\x88\x01`@\x81\x8B\x03`\x1F\x19\x01\x12\x15a\x0F\\W`\0\x80\xFD[a\x0Fda\x04\x1FV[\x86\x82\x015\x83\x81\x11\x15a\x0FuW`\0\x80\xFD[a\x0F\x83\x8C\x89\x83\x86\x01\x01a\x04\x99V[\x82RP`@\x82\x015\x83\x81\x11\x15a\x0F\x98W`\0\x80\xFD[\x80\x83\x01\x92PP\x8A`?\x83\x01\x12a\x0F\xADW`\0\x80\xFD[\x86\x82\x015a\x0F\xBDa\x05h\x82a\x05\x08V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01`@\x01\x90\x88\x81\x01\x90\x8D\x83\x11\x15a\x0F\xDDW`\0\x80\xFD[`@\x85\x01[\x83\x81\x10\x15a\x10\x15W\x86\x815\x11\x15a\x0F\xF8W`\0\x80\xFD[a\x10\x08\x8F`@\x835\x89\x01\x01a\x04\x99V[\x83R\x91\x8A\x01\x91\x8A\x01a\x0F\xE2V[P\x83\x8A\x01RPP\x85RPP\x91\x83\x01\x91\x83\x01a\x0F&V[`\0`@\x80\x83\x85\x03\x12\x15a\x10>W`\0\x80\xFD[a\x10Fa\x04\x1FV[\x91P\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10_W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x10sW`\0\x80\xFD[\x815` a\x10\x83a\x05h\x83a\x05\x08V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x10\xA2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x112W\x805\x86\x81\x11\x15a\x10\xBEW`\0\x80\x81\xFD[\x87\x01\x80\x8C\x03`\x1F\x19\x01\x89\x13\x15a\x10\xD4W`\0\x80\x81\xFD[a\x10\xDCa\x04\x1FV[\x85\x82\x015\x88\x81\x11\x15a\x10\xEEW`\0\x80\x81\xFD[a\x10\xFC\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82RP\x89\x82\x015\x88\x81\x11\x15a\x11\x11W`\0\x80\x81\xFD[a\x11\x1F\x8E\x88\x83\x86\x01\x01a\x04\x99V[\x82\x88\x01RP\x84RP\x91\x83\x01\x91\x83\x01a\x10\xA6V[P\x87RP\x86\x81\x015\x94P\x82\x85\x11\x15a\x11IW`\0\x80\xFD[a\x07\xA9\x88\x86\x89\x01a\x0E\xE2V[`\0`\xE0\x82\x84\x03\x12\x15a\x11gW`\0\x80\xFD[a\x11oa\x04GV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11\x88W`\0\x80\xFD[a\x11\x94\x85\x83\x86\x01a\x06\x95V[\x83R` \x84\x015\x91P\x80\x82\x11\x15a\x11\xAAW`\0\x80\xFD[a\x11\xB6\x85\x83\x86\x01a\x08\xEDV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x11\xCFW`\0\x80\xFD[a\x11\xDB\x85\x83\x86\x01a\x08\xEDV[`@\x84\x01R``\x84\x015\x91P\x80\x82\x11\x15a\x11\xF4W`\0\x80\xFD[a\x12\0\x85\x83\x86\x01a\x0BKV[``\x84\x01R`\x80\x84\x015\x91P\x80\x82\x11\x15a\x12\x19W`\0\x80\xFD[a\x12%\x85\x83\x86\x01a\x08\xEDV[`\x80\x84\x01R`\xA0\x84\x015\x91P\x80\x82\x11\x15a\x12>W`\0\x80\xFD[a\x12J\x85\x83\x86\x01a\r\xB8V[`\xA0\x84\x01R`\xC0\x84\x015\x91P\x80\x82\x11\x15a\x12cW`\0\x80\xFD[Pa\x12p\x84\x82\x85\x01a\x10+V[`\xC0\x83\x01RP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12\x91W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x12\xA8W`\0\x80\xFD[a\x12\xB4\x87\x83\x88\x01a\x04\x99V[\x94P` \x86\x015\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x12\xD1W`\0\x80\xFD[Pa\x12\xDE\x86\x82\x87\x01a\x11UV[\x91PP\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\xFEW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\x15W`\0\x80\xFD[a\x13!\x88\x83\x89\x01a\x04\x99V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x13EW`\0\x80\xFD[Pa\x13R\x87\x82\x88\x01a\x11UV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x13qW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\x88W`\0\x80\xFD[a\x13\x94\x86\x83\x87\x01a\x04\x99V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x13\xAAW`\0\x80\xFD[Pa\x13\xB7\x85\x82\x86\x01a\x11UV[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x13\xD7W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x02W`\0\x80\xFD[a\x13R\x87\x82\x88\x01a\x04\x99V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14#W`\0\x80\xFD[\x835\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14GW`\0\x80\xFD[a\x12\xDE\x86\x82\x87\x01a\x04\x99V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x14kW`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\x9DW`\0\x80\xFD[a\x14\xA9\x88\x82\x89\x01a\x04\x99V[\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\xC9W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xE6W`\0\x80\xFD[a\x13\xB7\x85\x82\x86\x01a\x04\x99V[` \x80\x82R`\n\x90\x82\x01Ri!\xA7\xA7*)'\xA6&\"\xA9`\xB1\x1B`@\x82\x01R``\x01\x90V[`\0[\x83\x81\x10\x15a\x151W\x81\x81\x01Q\x83\x82\x01R` \x01a\x15\x19V[PP`\0\x91\x01RV[`\0\x82Qa\x15L\x81\x84` \x87\x01a\x15\x16V[\x91\x90\x91\x01\x92\x91PPV[`\0\x81Q\x80\x84Ra\x15n\x81` \x86\x01` \x86\x01a\x15\x16V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x16\x1DW\x83\x85\x03\x8AR\x82Q`@\x81Q\x81\x88Ra\x15\xC3\x82\x89\x01\x82a\x15VV[\x92\x89\x01Q\x88\x84\x03\x89\x8B\x01R\x80Q\x80\x85R\x90\x8A\x01\x93\x86\x93P\x8A\x01\x91P[\x80\x83\x10\x15a\x16\x08W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x82R\x92\x89\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x89\x01\x90a\x15\xDFV[P\x9B\x88\x01\x9B\x96PPP\x91\x85\x01\x91`\x01\x01a\x15\xA1V[P\x92\x98\x97PPPPPPPPV[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x16\xA2W\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x16|\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x97\x84\x01\x97\x90\x97R\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x16XV[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x15\x82V[\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x16\x1DW\x83\x85\x03\x8AR\x82Q`@\x81Q\x81\x88Ra\x17\x08\x82\x89\x01\x82a\x15VV[\x92\x89\x01Q\x88\x84\x03\x89\x8B\x01R\x80Q\x80\x85R\x90\x8A\x01\x93\x86\x93P\x8A\x01\x91P[\x80\x83\x10\x15a\x17DW\x83Q\x82R\x92\x89\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x89\x01\x90a\x17$V[P\x9B\x88\x01\x9B\x96PPP\x91\x85\x01\x91`\x01\x01a\x16\xE6V[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x17\xC7W\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x17\xAA\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x97\x84\x01\x97\x90\x97R\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x17\x86V[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x16\xC7V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x16\x1DW\x83\x85\x03\x8AR\x82Q`@\x81Q\x81\x88Ra\x18$\x82\x89\x01\x82a\x15VV[\x92\x89\x01Q\x88\x84\x03\x89\x8B\x01R\x80Q\x80\x85R\x90\x8A\x01\x93\x86\x93P\x8A\x01\x91P[\x80\x83\x10\x15a\x18`W\x83Q\x82R\x92\x89\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x89\x01\x90a\x18@V[P\x9B\x88\x01\x9B\x96PPP\x91\x85\x01\x91`\x01\x01a\x18\x02V[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x18\xE3W\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x18\xC6\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x97\x84\x01\x97\x90\x97R\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x18\xA2V[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x17\xE3V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x16\x1DW\x83\x85\x03\x8AR\x82Q`@\x81Q\x81\x88Ra\x19@\x82\x89\x01\x82a\x15VV[\x92\x89\x01Q\x88\x84\x03\x89\x8B\x01R\x80Q\x80\x85R\x90\x8A\x01\x93\x86\x93P\x8A\x01\x91P[\x80\x83\x10\x15a\x19~W\x83Q\x15\x15\x82R\x92\x89\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x89\x01\x90a\x19\\V[P\x9B\x88\x01\x9B\x96PPP\x91\x85\x01\x91`\x01\x01a\x19\x1EV[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x1A\x03W\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x19\xE4\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x15\x15\x97\x84\x01\x97\x90\x97R\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x19\xC0V[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x18\xFFV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P\x83`\x05\x1B\x81\x01\x91P\x82\x86\x01`\0\x80[\x86\x81\x10\x15a\x16\x1DW\x83\x85\x03\x8AR\x82Q`@\x81Q\x81\x88Ra\x1A`\x82\x89\x01\x82a\x15VV[\x92\x89\x01Q\x88\x84\x03\x89\x8B\x01R\x80Q\x80\x85R\x90\x8A\x01\x93\x86\x93P\x8A\x01\x91P[\x80\x83\x10\x15a\x1A\x9CW\x83Q\x82R\x92\x89\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x89\x01\x90a\x1A|V[P\x9B\x88\x01\x9B\x96PPP\x91\x85\x01\x91`\x01\x01a\x1A>V[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x1B\x1FW\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x1B\x02\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x97\x84\x01\x97\x90\x97R\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x1A\xDEV[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x1A\x1FV[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P`\x05\x91P\x83\x82\x1B\x81\x01\x83\x87\x01`\0\x80[\x87\x81\x10\x15a\x1B\xE6W\x84\x84\x03\x8BR\x82Q`@\x81Q\x81\x87Ra\x1B}\x82\x88\x01\x82a\x15VV[\x92\x8A\x01Q\x87\x84\x03\x88\x8C\x01R\x80Q\x80\x85R\x90\x8B\x01\x93\x92P\x8A\x83\x01\x91P\x80\x8A\x1B\x83\x01\x8B\x01\x86[\x82\x81\x10\x15a\x1B\xCFW`\x1F\x19\x85\x83\x03\x01\x84Ra\x1B\xBD\x82\x87Qa\x15VV[\x95\x8D\x01\x95\x93\x8D\x01\x93\x91P`\x01\x01a\x1B\xA1V[P\x9E\x8B\x01\x9E\x97PPP\x93\x88\x01\x93PP`\x01\x01a\x1B[V[P\x91\x99\x98PPPPPPPPPV[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x1CrW\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x1CF\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x88\x83\x03\x89\x86\x01R\x91\x90Pa\x1C^\x81\x83a\x15VV[\x97PPP\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x1C\"V[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x1B;V[`\0\x81Q\x80\x84R` \x80\x85\x01\x80\x81\x96P`\x05\x91P\x83\x82\x1B\x81\x01\x83\x87\x01`\0\x80[\x87\x81\x10\x15a\x1B\xE6W\x84\x84\x03\x8BR\x82Q`@\x81Q\x81\x87Ra\x1C\xD0\x82\x88\x01\x82a\x15VV[\x92\x8A\x01Q\x87\x84\x03\x88\x8C\x01R\x80Q\x80\x85R\x90\x8B\x01\x93\x92P\x8A\x83\x01\x91P\x80\x8A\x1B\x83\x01\x8B\x01\x86[\x82\x81\x10\x15a\x1D\"W`\x1F\x19\x85\x83\x03\x01\x84Ra\x1D\x10\x82\x87Qa\x15VV[\x95\x8D\x01\x95\x93\x8D\x01\x93\x91P`\x01\x01a\x1C\xF4V[P\x9E\x8B\x01\x9E\x97PPP\x93\x88\x01\x93PP`\x01\x01a\x1C\xAEV[`\0`@\x80\x84\x01\x83Q\x82\x86R\x81\x81Q\x80\x84R``\x88\x01\x91P``\x81`\x05\x1B\x89\x01\x01\x93P` \x80\x84\x01\x93P`\0[\x82\x81\x10\x15a\x1D\xB6W\x89\x86\x03`_\x19\x01\x84R\x84Q\x80Q\x88\x88Ra\x1D\x8A\x89\x89\x01\x82a\x15VV[\x91\x84\x01Q\x88\x83\x03\x89\x86\x01R\x91\x90Pa\x1D\xA2\x81\x83a\x15VV[\x97PPP\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x1DfV[P\x80\x88\x01Q\x95P\x88\x85\x03\x81\x8A\x01RPPPPa\x16\xBE\x81\x83a\x1C\x8EV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R`\0\x90a\x1D\xF6\x90\x83\x01\x85a\x15VV[\x82\x81\x03`@\x84\x01R\x83Q`\xE0\x82Ra\x1E\x11`\xE0\x83\x01\x82a\x16+V[\x90P` \x85\x01Q\x82\x82\x03` \x84\x01Ra\x1E*\x82\x82a\x17YV[\x91PP`@\x85\x01Q\x82\x82\x03`@\x84\x01Ra\x1ED\x82\x82a\x18uV[\x91PP``\x85\x01Q\x82\x82\x03``\x84\x01Ra\x1E^\x82\x82a\x19\x93V[\x91PP`\x80\x85\x01Q\x82\x82\x03`\x80\x84\x01Ra\x1Ex\x82\x82a\x1A\xB1V[\x91PP`\xA0\x85\x01Q\x82\x82\x03`\xA0\x84\x01Ra\x1E\x92\x82\x82a\x1B\xF5V[\x91PP`\xC0\x85\x01Q\x82\x82\x03`\xC0\x84\x01Ra\x1E\xAC\x82\x82a\x1D9V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1E\xCAW`\0\x80\xFD[\x81Qa\x1E\xD5\x81a\t\xF9V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x1F\0\x90\x83\x01\x84a\x15VV[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xD3s\x13S\xAF\xC5Eb\xDE\xAB\xB0E\xFA\\\xE9_\x9E\xFD\xD0\xB3z\x17\xD2\x86_\x0F*/\x0C#\x13\x07dsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static EVENTEMITTER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct EventEmitter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for EventEmitter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for EventEmitter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for EventEmitter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for EventEmitter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(EventEmitter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> EventEmitter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                EVENTEMITTER_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                EVENTEMITTER_ABI.clone(),
                EVENTEMITTER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `emitDataLog1` (0xf9d5c0ea) function
        pub fn emit_data_log_1(
            &self,
            topic_1: [u8; 32],
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 213, 192, 234], (topic_1, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitDataLog2` (0xdda0db32) function
        pub fn emit_data_log_2(
            &self,
            topic_1: [u8; 32],
            topic_2: [u8; 32],
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 160, 219, 50], (topic_1, topic_2, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitDataLog3` (0xb3ac1c38) function
        pub fn emit_data_log_3(
            &self,
            topic_1: [u8; 32],
            topic_2: [u8; 32],
            topic_3: [u8; 32],
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 172, 28, 56], (topic_1, topic_2, topic_3, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitDataLog4` (0xee288ce8) function
        pub fn emit_data_log_4(
            &self,
            topic_1: [u8; 32],
            topic_2: [u8; 32],
            topic_3: [u8; 32],
            topic_4: [u8; 32],
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [238, 40, 140, 232],
                    (topic_1, topic_2, topic_3, topic_4, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitEventLog` (0x906c49f6) function
        pub fn emit_event_log(
            &self,
            event_name: ::std::string::String,
            event_data: EventLogData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 108, 73, 246], (event_name, event_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitEventLog1` (0x24de01e4) function
        pub fn emit_event_log_1(
            &self,
            event_name: ::std::string::String,
            topic_1: [u8; 32],
            event_data: EventLogData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 222, 1, 228], (event_name, topic_1, event_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `emitEventLog2` (0x63d16363) function
        pub fn emit_event_log_2(
            &self,
            event_name: ::std::string::String,
            topic_1: [u8; 32],
            topic_2: [u8; 32],
            event_data: EventLogData,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [99, 209, 99, 99],
                    (event_name, topic_1, topic_2, event_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `roleStore` (0x4a4a7b04) function
        pub fn role_store(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([74, 74, 123, 4], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EventLog` event
        pub fn event_log_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EventLogFilter> {
            self.0.event()
        }
        ///Gets the contract's `EventLog1` event
        pub fn event_log_1_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EventLog1Filter> {
            self.0.event()
        }
        ///Gets the contract's `EventLog2` event
        pub fn event_log_2_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EventLog2Filter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EventEmitterEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for EventEmitter<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Unauthorized` with signature `Unauthorized(address,string)` and selector `0xa35b150b`
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
    #[etherror(name = "Unauthorized", abi = "Unauthorized(address,string)")]
    pub struct Unauthorized {
        pub msg_sender: ::ethers::core::types::Address,
        pub role: ::std::string::String,
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
        name = "EventLog",
        abi = "EventLog(address,string,string,(((string,address)[],(string,address[])[]),((string,uint256)[],(string,uint256[])[]),((string,int256)[],(string,int256[])[]),((string,bool)[],(string,bool[])[]),((string,bytes32)[],(string,bytes32[])[]),((string,bytes)[],(string,bytes[])[]),((string,string)[],(string,string[])[])))"
    )]
    pub struct EventLogFilter {
        pub msg_sender: ::ethers::core::types::Address,
        pub event_name: ::std::string::String,
        #[ethevent(indexed)]
        pub event_name_hash: ::ethers::core::types::H256,
        pub event_data: EventLogData,
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
        name = "EventLog1",
        abi = "EventLog1(address,string,string,bytes32,(((string,address)[],(string,address[])[]),((string,uint256)[],(string,uint256[])[]),((string,int256)[],(string,int256[])[]),((string,bool)[],(string,bool[])[]),((string,bytes32)[],(string,bytes32[])[]),((string,bytes)[],(string,bytes[])[]),((string,string)[],(string,string[])[])))"
    )]
    pub struct EventLog1Filter {
        pub msg_sender: ::ethers::core::types::Address,
        pub event_name: ::std::string::String,
        #[ethevent(indexed)]
        pub event_name_hash: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub topic_1: [u8; 32],
        pub event_data: EventLogData,
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
        name = "EventLog2",
        abi = "EventLog2(address,string,string,bytes32,bytes32,(((string,address)[],(string,address[])[]),((string,uint256)[],(string,uint256[])[]),((string,int256)[],(string,int256[])[]),((string,bool)[],(string,bool[])[]),((string,bytes32)[],(string,bytes32[])[]),((string,bytes)[],(string,bytes[])[]),((string,string)[],(string,string[])[])))"
    )]
    pub struct EventLog2Filter {
        pub msg_sender: ::ethers::core::types::Address,
        pub event_name: ::std::string::String,
        #[ethevent(indexed)]
        pub event_name_hash: ::ethers::core::types::H256,
        #[ethevent(indexed)]
        pub topic_1: [u8; 32],
        #[ethevent(indexed)]
        pub topic_2: [u8; 32],
        pub event_data: EventLogData,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum EventEmitterEvents {
        EventLogFilter(EventLogFilter),
        EventLog1Filter(EventLog1Filter),
        EventLog2Filter(EventLog2Filter),
    }
    impl ::ethers::contract::EthLogDecode for EventEmitterEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EventLogFilter::decode_log(log) {
                return Ok(EventEmitterEvents::EventLogFilter(decoded));
            }
            if let Ok(decoded) = EventLog1Filter::decode_log(log) {
                return Ok(EventEmitterEvents::EventLog1Filter(decoded));
            }
            if let Ok(decoded) = EventLog2Filter::decode_log(log) {
                return Ok(EventEmitterEvents::EventLog2Filter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for EventEmitterEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EventLogFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EventLog1Filter(element) => ::core::fmt::Display::fmt(element, f),
                Self::EventLog2Filter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EventLogFilter> for EventEmitterEvents {
        fn from(value: EventLogFilter) -> Self {
            Self::EventLogFilter(value)
        }
    }
    impl ::core::convert::From<EventLog1Filter> for EventEmitterEvents {
        fn from(value: EventLog1Filter) -> Self {
            Self::EventLog1Filter(value)
        }
    }
    impl ::core::convert::From<EventLog2Filter> for EventEmitterEvents {
        fn from(value: EventLog2Filter) -> Self {
            Self::EventLog2Filter(value)
        }
    }
    ///Container type for all input parameters for the `emitDataLog1` function with signature `emitDataLog1(bytes32,bytes)` and selector `0xf9d5c0ea`
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
    #[ethcall(name = "emitDataLog1", abi = "emitDataLog1(bytes32,bytes)")]
    pub struct EmitDataLog1Call {
        pub topic_1: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `emitDataLog2` function with signature `emitDataLog2(bytes32,bytes32,bytes)` and selector `0xdda0db32`
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
    #[ethcall(name = "emitDataLog2", abi = "emitDataLog2(bytes32,bytes32,bytes)")]
    pub struct EmitDataLog2Call {
        pub topic_1: [u8; 32],
        pub topic_2: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `emitDataLog3` function with signature `emitDataLog3(bytes32,bytes32,bytes32,bytes)` and selector `0xb3ac1c38`
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
        name = "emitDataLog3",
        abi = "emitDataLog3(bytes32,bytes32,bytes32,bytes)"
    )]
    pub struct EmitDataLog3Call {
        pub topic_1: [u8; 32],
        pub topic_2: [u8; 32],
        pub topic_3: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `emitDataLog4` function with signature `emitDataLog4(bytes32,bytes32,bytes32,bytes32,bytes)` and selector `0xee288ce8`
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
        name = "emitDataLog4",
        abi = "emitDataLog4(bytes32,bytes32,bytes32,bytes32,bytes)"
    )]
    pub struct EmitDataLog4Call {
        pub topic_1: [u8; 32],
        pub topic_2: [u8; 32],
        pub topic_3: [u8; 32],
        pub topic_4: [u8; 32],
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `emitEventLog` function with signature `emitEventLog(string,(((string,address)[],(string,address[])[]),((string,uint256)[],(string,uint256[])[]),((string,int256)[],(string,int256[])[]),((string,bool)[],(string,bool[])[]),((string,bytes32)[],(string,bytes32[])[]),((string,bytes)[],(string,bytes[])[]),((string,string)[],(string,string[])[])))` and selector `0x906c49f6`
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
        name = "emitEventLog",
        abi = "emitEventLog(string,(((string,address)[],(string,address[])[]),((string,uint256)[],(string,uint256[])[]),((string,int256)[],(string,int256[])[]),((string,bool)[],(string,bool[])[]),((string,bytes32)[],(string,bytes32[])[]),((string,bytes)[],(string,bytes[])[]),((string,string)[],(string,string[])[])))"
    )]
    pub struct EmitEventLogCall {
        pub event_name: ::std::string::String,
        pub event_data: EventLogData,
    }
    ///Container type for all input parameters for the `emitEventLog1` function with signature `emitEventLog1(string,bytes32,(((string,address)[],(string,address[])[]),((string,uint256)[],(string,uint256[])[]),((string,int256)[],(string,int256[])[]),((string,bool)[],(string,bool[])[]),((string,bytes32)[],(string,bytes32[])[]),((string,bytes)[],(string,bytes[])[]),((string,string)[],(string,string[])[])))` and selector `0x24de01e4`
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
        name = "emitEventLog1",
        abi = "emitEventLog1(string,bytes32,(((string,address)[],(string,address[])[]),((string,uint256)[],(string,uint256[])[]),((string,int256)[],(string,int256[])[]),((string,bool)[],(string,bool[])[]),((string,bytes32)[],(string,bytes32[])[]),((string,bytes)[],(string,bytes[])[]),((string,string)[],(string,string[])[])))"
    )]
    pub struct EmitEventLog1Call {
        pub event_name: ::std::string::String,
        pub topic_1: [u8; 32],
        pub event_data: EventLogData,
    }
    ///Container type for all input parameters for the `emitEventLog2` function with signature `emitEventLog2(string,bytes32,bytes32,(((string,address)[],(string,address[])[]),((string,uint256)[],(string,uint256[])[]),((string,int256)[],(string,int256[])[]),((string,bool)[],(string,bool[])[]),((string,bytes32)[],(string,bytes32[])[]),((string,bytes)[],(string,bytes[])[]),((string,string)[],(string,string[])[])))` and selector `0x63d16363`
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
        name = "emitEventLog2",
        abi = "emitEventLog2(string,bytes32,bytes32,(((string,address)[],(string,address[])[]),((string,uint256)[],(string,uint256[])[]),((string,int256)[],(string,int256[])[]),((string,bool)[],(string,bool[])[]),((string,bytes32)[],(string,bytes32[])[]),((string,bytes)[],(string,bytes[])[]),((string,string)[],(string,string[])[])))"
    )]
    pub struct EmitEventLog2Call {
        pub event_name: ::std::string::String,
        pub topic_1: [u8; 32],
        pub topic_2: [u8; 32],
        pub event_data: EventLogData,
    }
    ///Container type for all input parameters for the `roleStore` function with signature `roleStore()` and selector `0x4a4a7b04`
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
    #[ethcall(name = "roleStore", abi = "roleStore()")]
    pub struct RoleStoreCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum EventEmitterCalls {
        EmitDataLog1(EmitDataLog1Call),
        EmitDataLog2(EmitDataLog2Call),
        EmitDataLog3(EmitDataLog3Call),
        EmitDataLog4(EmitDataLog4Call),
        EmitEventLog(EmitEventLogCall),
        EmitEventLog1(EmitEventLog1Call),
        EmitEventLog2(EmitEventLog2Call),
        RoleStore(RoleStoreCall),
    }
    impl ::ethers::core::abi::AbiDecode for EventEmitterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <EmitDataLog1Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmitDataLog1(decoded));
            }
            if let Ok(decoded) = <EmitDataLog2Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmitDataLog2(decoded));
            }
            if let Ok(decoded) = <EmitDataLog3Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmitDataLog3(decoded));
            }
            if let Ok(decoded) = <EmitDataLog4Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmitDataLog4(decoded));
            }
            if let Ok(decoded) = <EmitEventLogCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmitEventLog(decoded));
            }
            if let Ok(decoded) = <EmitEventLog1Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmitEventLog1(decoded));
            }
            if let Ok(decoded) = <EmitEventLog2Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EmitEventLog2(decoded));
            }
            if let Ok(decoded) = <RoleStoreCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RoleStore(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EventEmitterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::EmitDataLog1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmitDataLog2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmitDataLog3(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmitDataLog4(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmitEventLog(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmitEventLog1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmitEventLog2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RoleStore(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for EventEmitterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EmitDataLog1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitDataLog2(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitDataLog3(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitDataLog4(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitEventLog(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitEventLog1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmitEventLog2(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoleStore(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EmitDataLog1Call> for EventEmitterCalls {
        fn from(value: EmitDataLog1Call) -> Self {
            Self::EmitDataLog1(value)
        }
    }
    impl ::core::convert::From<EmitDataLog2Call> for EventEmitterCalls {
        fn from(value: EmitDataLog2Call) -> Self {
            Self::EmitDataLog2(value)
        }
    }
    impl ::core::convert::From<EmitDataLog3Call> for EventEmitterCalls {
        fn from(value: EmitDataLog3Call) -> Self {
            Self::EmitDataLog3(value)
        }
    }
    impl ::core::convert::From<EmitDataLog4Call> for EventEmitterCalls {
        fn from(value: EmitDataLog4Call) -> Self {
            Self::EmitDataLog4(value)
        }
    }
    impl ::core::convert::From<EmitEventLogCall> for EventEmitterCalls {
        fn from(value: EmitEventLogCall) -> Self {
            Self::EmitEventLog(value)
        }
    }
    impl ::core::convert::From<EmitEventLog1Call> for EventEmitterCalls {
        fn from(value: EmitEventLog1Call) -> Self {
            Self::EmitEventLog1(value)
        }
    }
    impl ::core::convert::From<EmitEventLog2Call> for EventEmitterCalls {
        fn from(value: EmitEventLog2Call) -> Self {
            Self::EmitEventLog2(value)
        }
    }
    impl ::core::convert::From<RoleStoreCall> for EventEmitterCalls {
        fn from(value: RoleStoreCall) -> Self {
            Self::RoleStore(value)
        }
    }
    ///Container type for all return fields from the `roleStore` function with signature `roleStore()` and selector `0x4a4a7b04`
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
    pub struct RoleStoreReturn(pub ::ethers::core::types::Address);
    ///`AddressArrayKeyValue(string,address[])`
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
    pub struct AddressArrayKeyValue {
        pub key: ::std::string::String,
        pub value: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///`AddressItems((string,address)[],(string,address[])[])`
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
    pub struct AddressItems {
        pub items: ::std::vec::Vec<AddressKeyValue>,
        pub array_items: ::std::vec::Vec<AddressArrayKeyValue>,
    }
    ///`AddressKeyValue(string,address)`
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
    pub struct AddressKeyValue {
        pub key: ::std::string::String,
        pub value: ::ethers::core::types::Address,
    }
    ///`BoolArrayKeyValue(string,bool[])`
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
    pub struct BoolArrayKeyValue {
        pub key: ::std::string::String,
        pub value: ::std::vec::Vec<bool>,
    }
    ///`BoolItems((string,bool)[],(string,bool[])[])`
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
    pub struct BoolItems {
        pub items: ::std::vec::Vec<BoolKeyValue>,
        pub array_items: ::std::vec::Vec<BoolArrayKeyValue>,
    }
    ///`BoolKeyValue(string,bool)`
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
    pub struct BoolKeyValue {
        pub key: ::std::string::String,
        pub value: bool,
    }
    ///`Bytes32ArrayKeyValue(string,bytes32[])`
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
    pub struct Bytes32ArrayKeyValue {
        pub key: ::std::string::String,
        pub value: ::std::vec::Vec<[u8; 32]>,
    }
    ///`Bytes32Items((string,bytes32)[],(string,bytes32[])[])`
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
    pub struct Bytes32Items {
        pub items: ::std::vec::Vec<Bytes32KeyValue>,
        pub array_items: ::std::vec::Vec<Bytes32ArrayKeyValue>,
    }
    ///`Bytes32KeyValue(string,bytes32)`
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
    pub struct Bytes32KeyValue {
        pub key: ::std::string::String,
        pub value: [u8; 32],
    }
    ///`BytesArrayKeyValue(string,bytes[])`
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
    pub struct BytesArrayKeyValue {
        pub key: ::std::string::String,
        pub value: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///`BytesItems((string,bytes)[],(string,bytes[])[])`
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
    pub struct BytesItems {
        pub items: ::std::vec::Vec<BytesKeyValue>,
        pub array_items: ::std::vec::Vec<BytesArrayKeyValue>,
    }
    ///`BytesKeyValue(string,bytes)`
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
    pub struct BytesKeyValue {
        pub key: ::std::string::String,
        pub value: ::ethers::core::types::Bytes,
    }
    ///`EventLogData(((string,address)[],(string,address[])[]),((string,uint256)[],(string,uint256[])[]),((string,int256)[],(string,int256[])[]),((string,bool)[],(string,bool[])[]),((string,bytes32)[],(string,bytes32[])[]),((string,bytes)[],(string,bytes[])[]),((string,string)[],(string,string[])[]))`
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
    pub struct EventLogData {
        pub address_items: AddressItems,
        pub uint_items: UintItems,
        pub int_items: IntItems,
        pub bool_items: BoolItems,
        pub bytes_32_items: Bytes32Items,
        pub bytes_items: BytesItems,
        pub string_items: StringItems,
    }
    ///`IntArrayKeyValue(string,int256[])`
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
    pub struct IntArrayKeyValue {
        pub key: ::std::string::String,
        pub value: ::std::vec::Vec<::ethers::core::types::I256>,
    }
    ///`IntItems((string,int256)[],(string,int256[])[])`
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
    pub struct IntItems {
        pub items: ::std::vec::Vec<IntKeyValue>,
        pub array_items: ::std::vec::Vec<IntArrayKeyValue>,
    }
    ///`IntKeyValue(string,int256)`
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
    pub struct IntKeyValue {
        pub key: ::std::string::String,
        pub value: ::ethers::core::types::I256,
    }
    ///`StringArrayKeyValue(string,string[])`
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
    pub struct StringArrayKeyValue {
        pub key: ::std::string::String,
        pub value: ::std::vec::Vec<::std::string::String>,
    }
    ///`StringItems((string,string)[],(string,string[])[])`
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
    pub struct StringItems {
        pub items: ::std::vec::Vec<StringKeyValue>,
        pub array_items: ::std::vec::Vec<StringArrayKeyValue>,
    }
    ///`StringKeyValue(string,string)`
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
    pub struct StringKeyValue {
        pub key: ::std::string::String,
        pub value: ::std::string::String,
    }
    ///`UintArrayKeyValue(string,uint256[])`
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
    pub struct UintArrayKeyValue {
        pub key: ::std::string::String,
        pub value: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///`UintItems((string,uint256)[],(string,uint256[])[])`
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
    pub struct UintItems {
        pub items: ::std::vec::Vec<UintKeyValue>,
        pub array_items: ::std::vec::Vec<UintArrayKeyValue>,
    }
    ///`UintKeyValue(string,uint256)`
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
    pub struct UintKeyValue {
        pub key: ::std::string::String,
        pub value: ::ethers::core::types::U256,
    }
}
