#![cfg_attr(not(feature = "std"), no_std, no_main)]

        
#[openbrush::implementation(PSP37, PSP37Batch, PSP37Burnable, PSP37Mintable, PSP37Enumerable,AccessControl)]
#[openbrush::contract]
pub mod p_learn_levels {
        use openbrush::{
        modifiers,
        traits::Storage,
    };
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Levels {
     #[storage_field]
        psp37: psp37::Data,
        #[storage_field]
        enumerable: enumerable::Data,
         #[storage_field]
        access: access_control::Data,
    }
     const MINTER: RoleType = ink::selector_id!("MINTER");
   #[default_impl(PSP37Burnable)]
    #[modifiers(only_role(MINTER))]
    fn burn() {}

    #[default_impl(PSP37Mintable)]
    #[modifiers(only_role(MINTER))]
    fn mint() {}
    impl Levels {
       #[ink(constructor)]
              pub fn new(minter: AccountId) -> Self {
            let mut instance = Self::default();

            let caller = instance.env().caller();
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            // We grant minter role to caller in constructor, so he can mint/burn tokens
            AccessControl::grant_role(&mut instance, MINTER, Some(caller)).expect("Should grant MINTER role");
            AccessControl::grant_role(&mut instance, MINTER, Some(minter)).expect("Should grant MINTER role");

            instance
        }
    }
}