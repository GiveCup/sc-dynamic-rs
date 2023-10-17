#![no_std]

multiversx_sc::imports!();

mod nft_module;

#[multiversx_sc::contract]
pub trait Contract:
    nft_module::NftModule {
    #[init]
    fn init(&self) {}

}
