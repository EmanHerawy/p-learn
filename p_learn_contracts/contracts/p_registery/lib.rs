#![cfg_attr(not(feature = "std"), no_std, no_main)]
 
#[openbrush::implementation(AccessControl)]

#[openbrush::contract]
mod p_registry {
     use openbrush::{
        // modifiers,
        traits::Storage,
    };
    use ink::storage::Mapping;
    use p_learn_levels::p_learn_levels::LevelsRef;
 
    #[ink(storage)]
    #[derive( Storage)]
    pub struct Contract {
         #[storage_field]
      pub  levels:LevelsRef,
        /// Stores a `mapping` for last level per player on the storage.
       pub  player_last_level: Mapping<AccountId, u32>,
       pub  proofs: Mapping<Hash, bool>,
         #[storage_field]
       pub access: access_control::Data,
    }
    

    impl Contract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
      pub fn new(owner : AccountId,other_contract_code_hash: Hash) -> Self {
        // instantiate the contract 
            let nft = LevelsRef::new(owner)
            .code_hash(other_contract_code_hash)
            .endowment(0)
            .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
            .instantiate();           
             let mut instance=Self {
                levels: nft,
                proofs: Mapping::new(),
                player_last_level: Mapping::new(),
                access: access_control::Data::default(),
            };
            let caller = instance.env().caller();
          access_control::Internal::_init_with_admin(&mut instance, Some(caller));
          instance

        }

      

      
    }

    }
