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
use runtime_primitives::traits::Hash;
use support::{decl_module, decl_storage, decl_event, StorageValue, StorageVec, StorageList, dispatch::Result};
use system::ensure_signed;
use runtime_io;


//use multihash::{encode, decode, Hash, Multihash, to_hex};
//use cid::{Cid, Codec, Version, Prefix};

pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    fn ipfs_get() {  //let addr = <IpfsAddr<T>>;
        // write data to ipfs
        //let data = b"awesome test content\n";
        //let h = multihash::encode(multihash::Hash::SHA2256, data).unwrap();
        //let cid = Cid::new(Codec::Raw, Version::V1, &h);
        //runtime_io::print(cid.to_string());
    }
    fn ipfs_push(addr: Vec<u8>, data: Vec<u8>) {
        //let mut cid = Vec<u8>::new();
        //encode(data, &mut cid);
        //<Value<T>>::put(value);
    }

    fn deposit_event() = default;

  }
}

impl<T: Trait> Module<T> {
    fn encode(data: Vec<u8>, cid: &mut Vec<u8>) -> Result {
        // hardcoded Blake2s multihash
        let size = 32;
        let code = 0x41;
        let mut output = Vec::new();
        output.resize(2 + size as usize, 0);
        output[0] = code;
        output[1] = size;
        // substrate is using Blake2s by default
        let _digest = <<T as system::Trait>::Hashing as Hash>::hash(&data);
        //*cid = _digest;
        Ok(())
        
    } 

} 

decl_storage! {
  trait Store for Module<T: Trait> as IpfsStorage {
    IpfsAddr: Vec<u8>;
    IpfsData: Vec<u8>;
    Value: u32;
  }
}


decl_event!(
    pub enum Event {
        Registered(u32),
    }
);
