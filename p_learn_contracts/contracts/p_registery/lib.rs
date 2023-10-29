#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[openbrush::implementation(AccessControl)]

#[openbrush::contract]
mod p_registry {
     use openbrush::{
        // modifiers,
        traits::Storage,
    };
    use ink::storage::Mapping;
    // use ink::ink_primitives::AccountId;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        /// Stores a `mapping` for last level per player on the storage.
         player_last_level: Mapping<AccountId, u32>,
         #[storage_field]
        access: access_control::Data,
    }

    impl Contract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
             let mut instance = Self::default();

            let caller = instance.env().caller();
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
              instance
        }

      
    }

    }
