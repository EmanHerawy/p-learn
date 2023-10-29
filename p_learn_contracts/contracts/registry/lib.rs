#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::registry::{
    Levels,
    LevelsRef,
};
        
#[openbrush::implementation(PSP37, PSP37Batch, PSP37Burnable,  AccessControl)]
#[openbrush::contract]
pub mod registry {
        use openbrush::{
        traits::Storage,
    };
      use ink::storage::Mapping;
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Levels {
     #[storage_field]
         psp37: psp37::Data,
        #[storage_field]
        access: access_control::Data,
        /// Stores a `mapping` for last level per player on the storage.
        player_last_level: Mapping<AccountId, u32>,
         proofs: Mapping<Hash, bool>,
    }

    impl Levels {
       #[ink(constructor)]
              pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = instance.env().caller();
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
           
            instance
        }
        /// Dummy implementation for validating the proof and mining the new level nft 
          /// TODO: implement zkp based solution 
      #[ink(message)]
      pub fn unlock_level(&mut self, proof: Hash, to:AccountId) {
        assert!(self.proofs.get(&proof).unwrap_or_default()==false,"used proof is not accessible");
        let player_level= self.player_last_level.get(&to).unwrap_or_default();
        // mint 
         let mut instance = Self::default();
        //fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error>;
        let mut ids_amounts: Vec<(Id, Balance)> = Vec::new();
         ids_amounts.push((psp37::Id::U32(player_level+1),1));
         // handel the result later on after hackathon
      let _=  psp37::Internal::_mint_to(&mut instance, to, ids_amounts);
        self.player_last_level.insert(to, &(player_level+1));
        self.proofs.insert(proof, &true);
      }
      #[ink(message)]
      pub fn get_player_level(&self, player:AccountId) -> u32{
        self.player_last_level.get(&player).unwrap_or_default()
      }

      #[ink(message)]
      pub fn get_proof(&self, proof:Hash) -> bool{
        self.proofs.get(&proof).unwrap_or_default()
      }   
    }
}