//  Copyright (c) 2019 Alain Brenzikofer
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.


use rstd::prelude::*;
// Encoding library
use parity_codec::Encode;

// Enables access to the runtime storage
use srml_support::{StorageValue, StorageMap, Parameter, dispatch::Result};

// Enables us to do hashing
use runtime_primitives::traits::{Zero, Hash, As, Member, SimpleArithmetic};

// Enables access to account balances and interacting with signed messages
use {balances, system::{self, ensure_signed}};

use parity_codec::Codec;

use timestamp::OnTimestampSet;
use Balances;
use runtime_io;

// list all traits that define types we will need
pub trait Trait: balances::Trait + timestamp::Trait {
  type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}


decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    // entry points
		fn register(origin) -> Result {
			let sender = ensure_signed(origin)?;
      Self::register_account_id(&sender);
      // debug only: reward for registration
      Self::reward_registrant(&sender);
      //println!("[ceremonies] register was called.");
			Ok(())
		}
		fn witness(_origin) -> Result {
			Ok(())
		}
 		fn on_finalise(n: T::BlockNumber) {
      let p = Self::ceremony_period();
      if (n % p).is_zero() {
        runtime_io::print("blocknumber is divisible by CeremonyPeriod");
        //<CeremonyRegistry<T>>::for_each_tuple(
        //  runtime_io::print
        //  ) 
      } 
    }

	}
}

decl_storage! {
  trait Store for Module<T: Trait> as Ceremonies {
    pub LastCeremony get(last_ceremony) config(): Option<T::BlockNumber>;
    CeremonyPeriod get(ceremony_period) config(): T::BlockNumber = runtime_primitives::traits::One::one();  
    pub WitnessingPeriod get(witnessing_period) config(): Option<T::BlockNumber>; //FIXME: should be simple u32
    pub CeremonyRegistry get(ceremony_registry): map T::AccountId => i32;
    pub Witnesses get(witnesses): map T::AccountId => T::AccountId;
    pub LuckyOne get(luckyone): T::AccountId;
  }
}

decl_event!(
    pub enum Event {
        Registered(u32),
    }
);

impl<T: Trait> Module<T> {

	// PUBLIC IMMUTABLES

	// PUBLIC MUTABLES (DANGEROUS)
  pub fn register_account_id(who: &T::AccountId) -> Result {
    <CeremonyRegistry<T>>::insert(who,1);
    Ok(())
  }

  // PRIVATE
//  fn on_timestamp_set<H: HandleReport>(now: T::Moment, slot_duration: T::Moment) {
//    //println!("[ceremonies] on_timestamp_set was called.")
//    Ok(())
//  }
  fn reward_registrant(who: &T::AccountId) -> Result {
    let value = T::Balance::sa(100);
    <balances::Module<T>>::reward(who, value);
    Ok(())
  }
}


impl<T: Trait> OnTimestampSet<T::Moment> for Module<T> {
	fn on_timestamp_set(moment: T::Moment) {
    runtime_io::print("ceremonies::on_timestamp_set was called!");
// doesnt compile
//    let luckyone = <LuckyOne<T>>::AccountId;
//		Self::reward_registrant(&luckyone);
	}
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
