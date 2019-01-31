use srml_support::{StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
use runtime_io;

pub trait Trait: system::Trait {
    type Event: From<Event> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    fn set_value(_origin, value: u32) -> Result {
      <Value<T>>::put(value);
      Ok(())
    }
    fn set_account_value(origin, value: u32) -> Result {
        let sender = ensure_signed(origin)?;
        <Registry<T>>::insert(sender.clone(), value);
        runtime_io::print("runtime_example::set_account_value was called!");
        Self::deposit_event(Event::Registered(value));
        Ok(())
    }
    fn deposit_event() = default;

  }
}

decl_storage! {
  trait Store for Module<T: Trait> as RuntimeExampleStorage {
    Value: u32;
    Registry: map T::AccountId => u32;
  }
}

decl_event!(
    pub enum Event {
        Registered(u32),
    }
);