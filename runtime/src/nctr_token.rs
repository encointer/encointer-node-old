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


// initialise with:
// post({sender: runtime.indices.ss58Decode('F7Gh'), call: calls.demo.setPayment(1000)}).tie(console.log)

use parity_codec::Encode;
use support::{decl_module, decl_storage, decl_event, StorageValue, Parameter, dispatch::Result};
use runtime_primitives::traits::{Hash, Zero, SimpleArithmetic, As, StaticLookup, Member, CheckedAdd, CheckedSub}; 
use balances;
use system::{self, IsDeadAccount, OnNewAccount, ensure_signed};
use rstd::prelude::*;
use runtime_io;

pub trait Trait: balances::Trait {}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		
		/// Transfer some liquid free balance to another staker.
		pub fn transfer(
			origin,
			dest: <T::Lookup as StaticLookup>::Source,
			//#[compact] value: T::Balance
            value: T::Balance
		) {
            runtime_io::print("nctr_token::transfer() was called!");

			let transactor = ensure_signed(origin)?;

			let dest = T::Lookup::lookup(dest)?;
			let from_balance = <balances::Module<T>>::free_balance(&transactor);
			let to_balance = <balances::Module<T>>::free_balance(&dest);
			let would_create = to_balance.is_zero();
			let fee = if would_create { Self::creation_fee(value) } else { Self::transfer_fee(value) };
			let liability = match value.checked_add(&fee) {
				Some(l) => l,
				None => return Err("got overflow after adding a fee to value"),
			};

			let new_from_balance = match from_balance.checked_sub(&liability) {
				Some(b) => b,
				None => return Err("balance too low to send value"),
			};
			if would_create && value < <balances::Module<T>>::existential_deposit() {
				return Err("value too low to create account");
			}
			T::EnsureAccountLiquid::ensure_account_liquid(&transactor)?;

			// NOTE: total stake being stored in the same type means that this could never overflow
			// but better to be safe than sorry.
			let new_to_balance = match to_balance.checked_add(&value) {
				Some(b) => b,
				None => return Err("destination balance too high to receive value"),
			};

			if transactor != dest {
				<balances::Module<T>>::set_free_balance(&transactor, new_from_balance);
				<balances::Module<T>>::decrease_total_stake_by(fee);
				<balances::Module<T>>::set_free_balance_creating(&dest, new_to_balance);
				//TODO <balances::Module<T>>::deposit_event(RawEvent::Transfer(transactor, dest, value, fee));
			}
		}
       
 	}
}

decl_storage! {
	trait Store for Module<T: Trait> as NctrToken {
  		pub TransactionPropFee get(transaction_prop_fee) config(): T::Balance;
	}
}

impl<T: Trait> Module<T> {
    fn transfer_fee(value: T::Balance) -> T::Balance{
        value / Self::transaction_prop_fee()
    } 
    fn creation_fee(value: T::Balance) -> T::Balance{
        Self::transfer_fee(value)
    } 

} 