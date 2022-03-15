use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub ft: FungibleToken,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128) -> Self {
        let mut contract = Contract {
            ft: FungibleToken::new(b"a".to_vec()),
        };
        let amount: Balance = total_supply.into();
        contract.ft.internal_register_account(&owner_id);
        contract.ft.internal_deposit(&owner_id, amount);
        log!("Deposit {} token to {}", amount, owner_id);
        contract
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, ft);
near_contract_standards::impl_fungible_token_storage!(Contract, ft);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        let data_url = "data:image/svg+xml;base64,\
        PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIGZpbGw9Im5vbmUiIHZ\
        pZXdCb3g9IjAgMCAyMDAgMjAwIj48cGF0aCBmaWxsPSIjMUUxRTFGIiBmaWxsLXJ1bGU9Im\
        V2ZW5vZGQiIGQ9Ik0xMTMuMiAxNzAuNDJhODMuNTEgODMuNTEgMCAwIDAgNzAuNy04Mi42I\
        DgzLjUgODMuNSAwIDAgMC01Ljc4LTMwLjYxbDEzLjY4LTIzLjg3aC0yOC4wOGE4My4xIDgz\
        LjEgMCAwIDAtNjMuMi0yOS4wNiA4My4xIDgzLjEgMCAwIDAtNjMuMiAyOS4wNkg4LjU4TDI\
        yLjcgNTcuNzhhODMuNSA4My41IDAgMCAwLTUuNTUgMzAuMDUgODMuNTEgODMuNTEgMCAwID\
        AgNzAuNiA4Mi41N2wxMi43NyAyMi4xMSAxMi42Ny0yMi4xWiIgY2xpcC1ydWxlPSJldmVub\
        2RkIi8+PHBhdGggZmlsbD0iI0VBRUNFRiIgZmlsbC1ydWxlPSJldmVub2RkIiBkPSJNMTAw\
        LjUzIDcuOTJBNzguNjYgNzguNjYgMCAwIDAgNDAuNjEgMzUuNmwtMS4xOCAxLjE3SDE2LjY\
        1TDI4LjIzIDU3LjFsLS42OCAxLjc3Yy0zLjQgOC44Ny02LjA3IDE4LjEtNi4wNyAyOC4yID\
        AgMzkuNzcgMjkuMDMgNzMuMTQgNjYuOTQgNzlsMS44Ni4zIDEwLjM0IDE3Ljc5IDEwLjA2L\
        TE3Ljc4IDEuODctLjNjMzcuOTUtNS44MSA2Ny4wMy0zOC44IDY3LjAzLTc4LjYxIDAtMTAu\
        My0yLjI0LTIwLjEzLTUuNzgtMjkuMTVsLS42OS0xLjc4IDExLjE4LTE5Ljc3aC0yMi42NWw\
        tMS4xOC0xLjE3YTc4LjY2IDc4LjY2IDAgMCAwLTU5LjkzLTI3LjY3Wk0zNS44NCAyOS4wNE\
        E4Ni40OCA4Ni40OCAwIDAgMSAxMDAuNTQgMGE4Ni40OCA4Ni40OCAwIDAgMSA2NC42OCAyO\
        S4wNGgzMi42N0wxODEuODEgNTcuMmMzLjQ2IDkuNDMgNS42NSAxOS42MyA1LjY1IDMwLjI2\
        IDAgNDMuMTItMzEuMDIgNzguOTUtNzEuODMgODYuMTRMMTAwLjY1IDIwMGwtMTUuMy0yNi4\
        0MmMtNDAuNzctNy4yMy03MS43NC00My40NC03MS43NC04Ni41MyAwLTEwLjQgMi42Mi0xOS\
        45OSA1LjkzLTI5LjI1TDMgMjkuMDRoMzIuODRaIiBjbGlwLXJ1bGU9ImV2ZW5vZGQiLz48c\
        GF0aCBmaWxsPSIjRjY4MjFGIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Im0zMS44NSA0NS40\
        MSA2OC43NSAxMjAuNDIgOC4wNi0xNC4xMi0yNy40Ny00OC4xM2gyNC4zTDcyLjMzIDQ1LjR\
        IMzEuODVabTQ5LjE5IDAgNDQuMTUgNzcuMzIgNy42Ny0xMy40NC0yOC4xOC00OS4zNGg4Lj\
        M3bDIzLjk5IDQyLjAyTDE0NSA4OGwtMTYuMDItMjguMDVoMTYuNDJsLTQuNDMgNy43NiA3L\
        jgxIDEzLjcgMjAuNTctMzZIODEuMDRaTTU2LjMgNTkuOTVsMTYuNjIgMjkuMTFoOC42N0w2\
        NC45NyA1OS45NUg1Ni4zWk00NC4yIDg0LjZhNTYuNjUgNTYuNjUgMCAwIDAgMzEuMDUgNTQ\
        uMTJsNi4zMiAxMS4wMWE2NC40NSA2NC40NSAwIDAgMS00NS4yOS02MS43YzAtNC45Ny41Ni\
        05LjggMS42LTE0LjQ0bDYuMzMgMTEuMDJabTc5LjY1LTQ3LjlhNTUuNDYgNTUuNDYgMCAwI\
        DAtMjMuNjYtNS4yNyA1NS40NSA1NS40NSAwIDAgMC0yMy42NSA1LjI2SDYxLjVhNjMuMjYg\
        NjMuMjYgMCAwIDEgMzguNy0xMy4xNmMxNC41NCAwIDI3Ljk1IDQuOSAzOC43IDEzLjE2aC0\
        xNS4wNVoiIGNsaXAtcnVsZT0iZXZlbm9kZCIvPjxwYXRoIGZpbGw9IiNGNjgyMUYiIGQ9Ik\
        00NC4xIDg4LjAyYzAtMS4xNC4wNC0yLjI4LjEtMy40MWwtNi4zLTExLjAyYTY1LjI2IDY1L\
        jI2IDAgMCAwLTEuNiAxNC40MyA2NC40NSA2NC40NSAwIDAgMCA0NS4yOCA2MS43MWwtNi4z\
        MS0xMWE1Ni42NSA1Ni42NSAwIDAgMS0zMS4xNS01MC43Wm0xMTMuMTIgMGMwLTEuMTQtLjI\
        tMi4yOC0uMjctMy40MWw2LjI1LTExLjAyYTY1LjMxIDY1LjMxIDAgMCAxIDEuODQgMTQuND\
        MgNjQuNDUgNjQuNDUgMCAwIDEtNDUuMjkgNjEuNzFsNi4zMS0xMWE1Ni42NSA1Ni42NSAwI\
        DAgMCAzMS4xNi01MC43WiIvPjwvc3ZnPg==";
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: "PembRock".to_string(),
            symbol: "PEM".to_string(),
            icon: Some(data_url.to_string()),
            reference: None,
            reference_hash: None,
            decimals: 18,
        }
    }
}
