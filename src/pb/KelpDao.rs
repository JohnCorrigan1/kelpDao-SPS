// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetDeposits {
    #[prost(message, repeated, tag="1")]
    pub asset_deposits: ::prost::alloc::vec::Vec<AssetDeposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetDeposit {
    #[prost(string, tag="1")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub asset: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub asset_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub deposit_amount: ::prost::alloc::string::String,
    #[prost(double, tag="5")]
    pub deposit_amount_readable: f64,
    #[prost(string, tag="6")]
    pub rseth_mint_amount: ::prost::alloc::string::String,
    #[prost(double, tag="7")]
    pub rseth_mint_amount_readable: f64,
    #[prost(string, tag="8")]
    pub referral_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub block_number: u64,
    #[prost(string, tag="10")]
    pub trx: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(uint64, tag="12")]
    pub ordinal: u64,
}
// @@protoc_insertion_point(module)
