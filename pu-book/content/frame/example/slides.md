---
title: Example Building a Pallet
description: Build a new pallet following the instructions from substrate.
duration: 1 hour
---

# Create a new project

``` sh
git clone node template
# Remane to workshop-node-template
mv substrate-node-template workshop-node-template
cd workshop-node-template
git switch -c build-substrate-workshop
cargo build --release
```

- In subsequent steps, the workshop-node-template directory is used to refer to the root directory for the node.

---

# Creating a new pallet

``` sh
cd substrate-node-template/pallets/
cargo new collectibles
cd collectibles
#Rename src-main.rs to src/lib.rs
mv src/main.rs src/lib.rs
```

- With these comands the porject files are in place to our `collectibles` pallet.
- Let's fix a issue is cargo warned you about by adding the new project to the current workspace.

---

# Add the pallet to the workspace

``` rust
// workshop-node-template/Cargo.toml 
[workspace]
members = [
    "node", 
    "pallets/collectibles",
    "pallets/template",
    "runtime",
]
```

- On the `workshop-node-template` add to the `Cargo.toml` the new `pallets/collectibles` pallet as a member of the workspace.

---

# Prepare the project manifest

- In this workshop, the collectibles module is going to be part of the Substrate runtime and its Cargo.toml file needs to define some modules it depends on.
  - `frame_system` provides core functionality for working with common data structures and primitives so they are available to all of the pallets that need them, enabling new pallets to be easily integrated into a runtime and to interact with each other.
  - `frame_support` provides core support services for handling function calls that are dispatched to the runtime, defining storage structures, preparing events and errors and core utilities.

---

# Prepare the project manifest

```rust
// pallets/collectibles/Cargo.toml
[dependencies]
frame-support = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-v1.9.0"}
frame-system = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-v1.9.0" }
```

- Add `frame-support` and `frame-system` to the dependencies on `pallets/collectibles/Cargo.toml`
---

note : 
Changed the versions to a updated one, because the version on the example didn't worked.

# Prepare the project manifest

- The `collectibles` module requires packages to support the type encoding and decoding required to minimize network traffic for the blockchain. To support encoding and decoding in the SCALE format, the `collectibles` module needs access to the `codec` and `scale-info` packages. 

---

# Prepare the project manifest

```rust
// pallets/collectibles/Cargo.toml
codec = { package = "parity-scale-codec", version = "3.0.0", default-features = false, features = ["derive",] }
scale-info = { version = "2.1.1", default-features = false, features = ["derive"] }
```

- Add the `codec` and `scale-info` to the dependencies.
---

# Prepare the project manifest

```rust
// pallets/collectibles/Cargo.toml
[features]
default = ["std"]
std = [
  "frame-support/std",
  "frame-system/std",
  "codec/std",
  "scale-info/std",
]
```

- Add the standard features for these packages to the Cargo.toml file to support the native binary target for the runtime.
- You now have the bare minimum of package dependencies that your pallet requires specified in the Cargo.toml manifest. 

---

# Prepare common code sections

<div style="font-size: 0.80em;">

- Prepare the scaffolding for the Substrate collectibles pallet by adding the following common set of marco declarations to the src/lib.rs file:

```rust
//pallets/collectibles/src/lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);
	
    #[pallet::config]
    pub trait Config: frame_system::Config {
    }
}
```

Note: 

`#[pallet::generate_store(pub(super) trait Store)]` Removed [link to explanation](https://substrate.stackexchange.com/questions/8659/pallet-generate-store-is-deprecated-how-to-avoid-this-warning-and-compile-the-co)

---

# Shared and custom types

- Now that you have the bare bones of the pallet in place, you’re ready to start customizing the code to implement the application-specific interfaces it requires. This is where application design comes into play. Because FRAME is modular and takes advantage of the flexibility that Rust traits and generic types provide, often you can find the interfaces you need are already provided in frame_system, frame_support, or in other predefined pallets and you can import them directly into your pallet.

---

# Import and declare external interfaces

- For the collectibles pallet, you want:
  - Keep track of who owns which collectibles
  - Means of transferring collectibles from one account to another
  - Make the collectibles unique by incorporating a random value

- These features are already defined as traits in the `frame_support`. In `frame_support` the traits are:
  - Currency to enable access to account balances, transfer operations, and the Balance type.
  - Randomness to enable access to an on-chain random value.

---

# Import and declare external interfaces REMOVER

You might remember that Rust traits enable you to define functionality for a particular type that can be shared with other types. To take advantage of this, you can import the Currency and Randomness traits from the `frame_support` module, then define them as types and specify how they behave in the context of the collectibles pallet.

In addition to the Currency and Randomness traits, the collectibles pallet requires an interface to specify the maximum number of collectible assets a single user can own. For this interface, the collectibles pallet defines a Get<u32> trait that fetches a u32 value to specify the MaximumOwned constant.

---

# Import and declare external interfaces

By including these external interfaces in the configuration of the collectibles pallet, the collectibles pallet will be able to:

  -  Access and manipulate user accounts and balances.
  -  Generate on-chain randomness.
  -  Set a limit on the number of collectibles an single user can owns.

---

# Import and declare external interfaces

```rust
// pallets/collectibles/src/lib.rs
use frame_support::traits::{Currency, Randomness};
```

- Import the Currency and Randomness traits from the `frame_support` module into your project.

---

# Import and declare external interfaces

```rust
// pallets/collectibles/src/lib.rs
	#[pallet::config]
	
	pub trait Config: frame_system::Config {
		
		type Currency: Currency<Self::AccountId>;
		type CollectionRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
	
		#[pallet::constant]
		type MaximumOwned: Get<u32>;
	}
```

- Update the collectibles Config trait to declare the Currency, Randomness, and Get<u32> traits.

Note: 

- The example on the [tutorial](https://docs.substrate.io/tutorials/collectibles-workshop/04-import-traits/) uses `Self::BlockNumber` but they change it in this [commit](https://github.com/paritytech/polkadot-sdk/commit/5e7b27e98c0b9dacd4b6f0724fd6335106d94101) and now it need to be `BlockNumberFor<Self>`
---

# Import and declare external interfaces

Verify that your program compiles by running the following command:

```
cargo build --package collectibles
```

---

# Add custom types

- Substrate supports all the primitive types available in Rust—for example, bool, u8, u32, and other common types. 
- Substrate also provides several common custom types that are specific to Substrate such as, `AccountId`, `BlockNumber`, and `Hash`that are available for you to use through the imported`frame_system`and`frame_support`modules. 


---

# Add custom types

- You have already imported some external interfaces for the `collectibles` pallet to use. Now, you can define a few custom properties to describe the collectibles. To define these custom properties, you'll add two custom types:
  - An enumerated data type to list the possible variants for the `Color` property.
  - A structure (struct) data type to group the attributes of a `Collectible`.

---

# Enumerated variants

```rust
// pallets/collectibles/src/lib.rs
 pub enum Color {
            Red,
            Yellow,
            Blue,
            Green
        }
```

---

# Enumerated variants

- The Collectible struct consists of the following:

  - unique_id is an unsigned integer of 16 bytes to ensure that each collectible is a unique entity in the blockchain.
  - price as an Option that returns Some(value) if a price is set or None to indicate that the collectible isn't for sale.
  - color for the variant of the custom Color type for the collectible.
  - owner to identify the account that owns the collectible.

---

# Enumerated variants

```rust
// pallets/collectibles/src/lib.rs
type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
```
- Because we've imported the Currency trait, we can also use the `Balance` and `AccountId` types from the Currency interface in the `collectibles` pallet.

- Create a type alias for the `Balance` type called `BalanceOf`

---

# Enumerated variants

Use `BalanceOf` and `AccountId` in the Collectible structure:

```rust
// pallets/collectibles/src/lib.rs
pub struct Collectible<T: Config> {
    // Unsigned integers of 16 bytes to represent a unique identifier
    pub unique_id: [u8; 16],
    // `None` assumes not for sale
    pub price: Option<BalanceOf<T>>,
    pub color: Color,
    pub owner: T::AccountId,
}
```
---

# Enumerated variants
You can check that the code compiles by running the following command:

```
cargo build --package collectibles
```

---

# Implement required traits

- There are several traits that Substrate requires for every data type. For example, every data type must implement the Encode and Decode traits that enable data to be serialized and deserialized so that it can be efficiently transferred over the network. 
- You can use the `#[derive]` macro to implement all the traits the pallet expects from your custom types. 

```rust
// pallets/collectibles/src/lib.rs
// Add above Color and Collectible
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
```

---

# Add custom storage items

- For the collectibles pallet to be useful, it needs to store information about the number of collectibles created and who owns each collectible. 
- After you decide the information you want to store, you need to decide how it should be stored

---

# Add custom storage items

- For the workshop, you'll create three custom storage items to track the state:

  - A simple single value `CollectiblesCount` to keep track of the total number of collectibles in the pallet.
  - A simple map of key-value pairs `CollectiblesMap` to map the properties associated with each collectible to its unique identifier.
  - A simple map of key-value pairs `OwnerOfCollectibles` to map collectibles to the user account that owns them.

- For a closer look at the storage architecture and abstractions that Substrate uses, see [State transitions and storage](https://docs.substrate.io/learn/state-transitions-and-storage/).

---

# Store a single value

- The FRAME storage module provides a `StorageValue` trait to store single values in the runtime.

- In this example, it is using a StorageValue for the `CollectiblesCount` to keep track of the total number of collectibles in the pallet. 

``` rust
#[pallet::storage]
pub(super) type CollectiblesCount<T: Config> = StorageValue<_, u64, ValueQuery>;
```

- The `StorageValue` keeps track of a 64-bit unsigned integer (u64) value that is incremented each time you generate a new collectible, up to maximum of 18446744073709551615 unique collectibles.

---

# Store a single value

- The `ValueQuery` in this declaration specifies what a query should return if there's no value in storage. 
- There are three possible settings for handling what the query returns: 
  - `OptionQuery` 
  - `ResultQuery`
  - `ValueQuery` 
- We use `ValueQuery` here so that if there is no value in storage, for example when you first start the network, the query should return the value zero (0) rather than an `OptionQuery` value of `None` or a `ResultQuery` value of `Err`.

---

# Map collectibles to their properties

- The FRAME storage module provides a `StorageMap` trait to store single key maps in the runtime. 
- A `StorageMap` named `CollectiblesMap` maps each collectible to its unique information. The key for the CollectiblesMap map is the `unique_id` of the collectible.

```
/// Maps the Collectible struct to the unique_id.
#[pallet::storage]
pub(super) type CollectibleMap<T: Config> = StorageMap<_, Twox64Concat, [u8; 16], Collectible<T>>;
```

---

# Map collectibles to their properties

- The Twox64Concat in this declaration specifies the hashing algorithm to use to create this storage value. 
- By allowing you to specify the hashing algorithm to use, storage maps allow you to control the level of security appropriate to the type of information being stored. For example, you might choose a more performant but less secure hashing algorithm to store information about collectibles and a less performant but more secure hashing algorithm to store more sensitive information.

---

# Map owners to their collectibles

- A StorageMap named `OwnerOfCollectibles` maps user accounts to the collectibles they own.

```rust
/// Track the collectibles owned by each account.
#[pallet::storage]
pub(super) type OwnerOfCollectibles<T: Config> = StorageMap<
	_,
	Twox64Concat,
	T::AccountId,
	BoundedVec<[u8; 16], T::MaximumOwned>,
	ValueQuery,
>;
```

---

# Map owners to their collectibles

- The key for this storage map is a user account: `T::AccountID`. 
- The value for this storage map is a `BoundedVec` data type with the `unique_id` of each collectible that each user account owns. 
- This map makes it easy to look up each individual collectible for its information since the `unique_id` is used as the key for the `collectiblesMap` map. 
- By using a `BoundedVec`, you can ensure that each storage item has a maximum length, which is important for managing limits within the runtime.

---

# Map owners to their collectibles

In order for the code to compile, you'll need to annotate the Collectible data structure with this derive macro:

``` rust 
// add above the struct Collectible
#[scale_info(skip_type_params(T))]
```

---

# Map owners to their collectibles

Verify that your program compiles by running the following command:

```
cargo build --package collectibles
```

---

# Create basic functions

<div style="font-size: 0.90em;">

- The only function you need to expose to users for this workshop is the `create_collectible` function. 
- This function enables users to create new unique collectibles that are stored in the `CollectibleMap` and added to the `OwnerOfCollectibles` map. 
- At a high level, you'll need to write functions to perform the following tasks:
  - Create a unique identifier for each collectible and not allow duplicates.
  - Limit the number of collectibles each account can own to manage the storage each user is allowed to consume.
  - Ensure the total number of collectibles doesn't exceed the maximum allowed by the `u64` data type.
  - Allow users to generate new collectibles.

---

# Create basic functions

- In addition to the basic functionality required to generate new collectibles, your pallet should provide some basic event and error handling. 
- With this in mind, you'll also need to:
  - Create custom error messages to report what happened if something goes wrong.
  - Create a custom event to signal when a new collectible was successfully created.
- Errors and events are fairly straightforward, so let's start with those declarations first.

---

# Add custom errors

- Here are some potential errors that the `create_collectible` function should address:

  - `DuplicateCollectible` - thrown when the collectible item trying to be created already exists.
  - `MaximumCollectiblesOwned`- thrown when an account exceeds the maximum amount of collectibles a single account can hold.
  - `BoundsOverflow` - thrown if the supply of collectibles exceeds the u64 limit.

---

# Add custom errors

- Add the `#[pallet::error]` macro after the storage macros you previously defined.

``` rust
#[pallet::error]
pub enum Error<T> {
	/// Each collectible must have a unique identifier
	DuplicateCollectible,
	/// An account can't exceed the `MaximumOwned` constant
	MaximumCollectiblesOwned,
	/// The total supply of collectibles can't exceed the u64 limit
	BoundsOverflow,
}
```

---

# Add an event

- The runtime can emit events to notify front-end applications about the result of a transaction that executed successfully. 

- To add a `CollectibleCreated` event to the runtime:

  - Add the RuntimeEvent from the`frame_system`configuration to the pallet configuration.

``` rust
#[pallet::config]
pub trait Config: frame_system::Config {
	type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}
```

---

# Add an event

- Add the `#[pallet::event]` macro after the error macro you previously defined.

``` rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
	 /// A new collectible was successfully created
	 CollectibleCreated { collectible: [u8; 16], owner: T::AccountId },
}
```

---

# Add an event

Verify that your program compiles by running the following command:

```
cargo build --package collectibles
```

---

# Add internal and callable functions

- With errors and events out of the way, it's time to write the core logic for creating collectibles.

  - Create an internal function that generates the unique_id for new collectibles.

---

```
// Pallet internal functions
impl<T: Config> Pallet<T> {
	// Generates and returns the unique_id and color
	fn gen_unique_id() -> ([u8; 16], Color) {
		// Create randomness
		let random = T::CollectionRandomness::random(&b"unique_id"[..]).0;
		
		// Create randomness payload. Multiple collectibles can be generated in the same block,
		// retaining uniqueness.
		let unique_payload = (
			random,
			frame_system::Pallet::<T>::extrinsic_index().unwrap_or_default(),frame_system::Pallet::<T>::block_number(),
	);
	
	// Turns into a byte array
	let encoded_payload = unique_payload.encode();
	let hash = frame_support::Hashable::blake2_128(&encoded_payload);
	
	// Generate Color 
	if hash[0] % 2 == 0 {
			(hash, Color::Red)
	} else {
			(hash, Color::Yellow)
		} 
	}
}
```

---

<div style="font-size: 0.90em;">

# Add internal and callable functions

- Create an internal function that enables minting new collectibles.

```
    // Function to mint a collectible
	pub fn mint(
		owner: &T::AccountId,
		unique_id: [u8; 16],
		color: Color,
	) -> Result<[u8; 16], DispatchError> {
		// Create a new object
		let collectible = Collectible::<T> { unique_id, price: None, color, owner: owner.clone() };
		
		// Check if the collectible exists in the storage map
		ensure!(!CollectibleMap::<T>::contains_key(&collectible.unique_id), Error::<T>::DuplicateCollectible);
		
		// Check that a new collectible can be created
		let count = CollectiblesCount::<T>::get();
		let new_count = count.checked_add(1).ok_or(Error::<T>::BoundsOverflow)?;
		
		// Append collectible to OwnerOfCollectibles map
		OwnerOfCollectibles::<T>::try_append(&owner, collectible.unique_id)
			.map_err(|_| Error::<T>::MaximumCollectiblesOwned)?;
		
		// Write new collectible to storage and update the count
		CollectibleMap::<T>::insert(collectible.unique_id, collectible);
		CollectiblesCount::<T>::put(new_count);
		
		// Deposit the "CollectibleCreated" event.
		Self::deposit_event(Event::CollectibleCreated { collectible: unique_id, owner: owner.clone() });
		
		// Returns the unique_id of the new collectible if this succeeds
		Ok(unique_id)
	}
```
---

<div style="font-size: 0.90em;">

# Add internal and callable functions

- Create the callable function that uses the internal functions.

```
// Pallet callable functions
#[pallet::call]
impl<T: Config> Pallet<T> {
	 
	 /// Create a new unique collectible.
	 ///
	 /// The actual collectible creation is done in the `mint()` function.
	 #[pallet::weight(0)]
	 pub fn create_collectible(origin: OriginFor<T>) -> DispatchResult {
		  // Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;
			
			// Generate the unique_id and color using a helper function
			let (collectible_gen_unique_id, color) = Self::gen_unique_id();
			
			// Write new collectible to storage by calling helper function
			Self::mint(&sender, collectible_gen_unique_id, color)?;
			
			Ok(())
		}
}
```

---

# Add internal and callable functions

Save your changes and verify that your program compiles by running the following command:

```
cargo build --package collectibles
```

---

# More functions

- To make this pallet more useful and realistic as the foundation for a simple collectible trading application, let's add a few more basic functions to perform the following tasks:

  - Transfer collectibles between accounts.
  - Set the price of a collectible.
  - Buy a collectible.

---

# Transfer

- To allow users to transfer a collectible from one account to another, you need to create a publicly callable function. 
- For this workshop, the public function `transfer` will rely on an internal function called `do_transfer` to perform checks and write changes to storage.
- The internal function includes the following checks to determine whether a collectible can be transferred:
  - A collectible must exist to be transferrable.
  - A collectible cannot be transferred to its current owner.
  - A collectible cannot be transferred to an account that already has the maximum number of collectibles allowed.

---

# Transfer

- If the collectible exists and isn't being transferred to its current owner, the `do_transferinternal` function updates the `OwnerOfCollectibles` storage map to reflect the new owner and writes the change to the underlying database.
- To handle errors and emit events, you'll also need:
  - `NoCollectible`, `NotOwner`, and `TransferToSelf` errors when checks fail.
  - A `TransferSucceeded` event when a transfer is successful.

---

<div style="font-size: 0.90em;">

# Transfer

- The publicly callable function simply checks the origin of the caller and calls the `do_transfer` internal function.

  - Add the `NoCollectible`, `NotOwner`, and `TransferToSelf` variants to the enumeration of errors.

```
// Pallet error messages.
#[pallet::error]
pub enum Error<T> {
	 /// Each collectible must have a unique identifier
	 DuplicateCollectible,
	 /// An account can't exceed the `MaximumOwned` constant
	 MaximumCollectiblesOwned,
	 /// The total supply of collectibles can't exceed the u64 limit
	 BoundsOverflow,
	 /// The collectible doesn't exist
	 NoCollectible,
	 /// You are not the owner
	 NotOwner,
	 /// Trying to transfer a collectible to yourself
	 TransferToSelf,
}
```

---

# Transfer

- Add the TransferSucceeded event to your pallet.

```
// Pallet events.
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
	  /// A new collectible was successfully created.
		CollectibleCreated { collectible: [u8; 16], owner: T::AccountId },
		/// A collectible was successfully transferred.
		TransferSucceeded { from: T::AccountId, to: T::AccountId, collectible: [u8; 16] },
}
```

---

# Transfer

- Create an internal function that enables transferring collectibles.

```
// Update storage to transfer collectible
pub fn do_transfer(
	 collectible_id: [u8; 16],
	 to: T::AccountId,
) -> DispatchResult {
	 // Get the collectible
	 let mut collectible = CollectibleMap::<T>::get(&collectible_id).ok_or(Error::<T>::NoCollectible)?;
	 let from = collectible.owner;
	 
	 ensure!(from != to, Error::<T>::TransferToSelf);
	 let mut from_owned = OwnerOfCollectibles::<T>::get(&from);
	 
	 // Remove collectible from list of owned collectible.
	 if let Some(ind) = from_owned.iter().position(|&id| id == collectible_id) {
		 from_owned.swap_remove(ind);
		} else {
		  return Err(Error::<T>::NoCollectible.into())
		}
			// Add collectible to the list of owned collectibles.
			let mut to_owned = OwnerOfCollectibles::<T>::get(&to);
			to_owned.try_push(collectible_id).map_err(|_id| Error::<T>::MaximumCollectiblesOwned)?;
			
			// Transfer succeeded, update the owner and reset the price to `None`.
			collectible.owner = to.clone();
			collectible.price = None;

			// Write updates to storage
			CollectibleMap::<T>::insert(&collectible_id, collectible);
			OwnerOfCollectibles::<T>::insert(&to, to_owned);
			OwnerOfCollectibles::<T>::insert(&from, from_owned);
			
			Self::deposit_event(Event::TransferSucceeded { from, to, collectible: collectible_id });
		Ok(())
	}
```

---

# Trasnfer

- Create the callable function.

```
/// Transfer a collectible to another account.
/// Any account that holds a collectible can send it to another account. 
/// Transfer resets the price of the collectible, marking it not for sale.
#[pallet::weight(0)]
pub fn transfer(
	  origin: OriginFor<T>,
		to: T::AccountId,
		unique_id: [u8; 16],
) -> DispatchResult {
	// Make sure the caller is from a signed origin
	let from = ensure_signed(origin)?;
	let collectible = CollectibleMap::<T>::get(&unique_id).ok_or(Error::<T>::NoCollectible)?;
	ensure!(collectible.owner == from, Error::<T>::NotOwner);
	Self::do_transfer(unique_id, to)?;
	Ok(())
}
```
---

# Set Price

<div style="font-size: 0.90em;">

- To allow collectible owners to set the price of the collectibles they own, the pallet must provide a function to update the price field in Collectible data structure and emit in event.
- For this function, you can use the `get` and `insert` methods for the `CollectibleMap` storage item to modify and update the Collectible object.
- Like previous functions, you'll want to perform a few checks before you allow the caller to write a new price to storage:

  - The caller must be a signed origin.
  - The collectible must already exist.
  - The caller must be the owner of the collectible.

- If the checks pass, the function writes the new price to storage and emits a `PriceSet` event.

---

# Set Price

- Add the `PriceSet` event to your pallet.

```
// Pallet events
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
	/// A new collectible was successfully created.
	CollectibleCreated { collectible: [u8; 16], owner: T::AccountId },
	/// A collectible was successfully transferred.
	TransferSucceeded { from: T::AccountId, to: T::AccountId, collectible: [u8; 16] },
	/// The price of a collectible was successfully set.
	PriceSet { collectible: [u8; 16], price: Option<BalanceOf<T>> },
	}
```

---

# Set Price

- Add the callable function for setting a price.

```
/// Update the collectible price and write to storage.
#[pallet::weight(0)]
pub fn set_price(
    origin: OriginFor<T>,
    unique_id: [u8; 16],
    new_price: Option<BalanceOf<T>>,
) -> DispatchResult {
    // Make sure the caller is from a signed origin
    let sender = ensure_signed(origin)?;
    // Ensure the collectible exists and is called by the owner
    let mut collectible = CollectibleMap::<T>::get(&unique_id).ok_or(Error::<T>::NoCollectible)?;
    ensure!(collectible.owner == sender, Error::<T>::NotOwner);
    // Set the price in storage
    collectible.price = new_price;
    CollectibleMap::<T>::insert(&unique_id, collectible);

    // Deposit a "PriceSet" event.
    Self::deposit_event(Event::PriceSet { collectible: unique_id, price: new_price });
    Ok(())
}
```

---

# Set Price

- Verify that your program compiles without errors by running the following command:

```
cargo build --package collectibles
```

---

# Buy a collectible

<div style="font-size: 0.85em;">

- To enable users to buy collectibles, you need to expose another callable function `buy_collectible`.

- For this workshop, the internal function is `do_buy_collectible` and it does most of the heavy lifting to determine whether the attempt to purchase a collectible will succeed. For example, the `do_buy_collectible` internal function checks that:
  - The proposed buying price is greater than or equal to the price set for the collectible by its owner and returns the `BidPriceTooLow` error if the proposed price is too low.
  - The collectible is for sale and returns a `NotForSale` error if the collectible price is `None`.
  - The account for the buyer has a free balance available to cover the price set for the collectible.
  - The account for the buyer doesn't already own too many collectibles to receive another collectible.

---

# Buy a collectible

- If all of the checks pass, the `do_buy_collectible` internal function updates account balances and transfers ownership of the collectible using the `Currency` trait's transfer method.

- With most of the work done by the internal function, the publicly exposed `buy_collectible` function simply verifies the account of the function caller and calls the `do_buy_collectible` function.

---

# Buy a collectible

- Add the BidPriceTooLow and NotForSale to the variants for the Error enumerated data type.

```
// Pallet errors
#[pallet::error]
 pub enum Error<T> {
   /// Each collectible must have a unique identifier
   DuplicateCollectible,
   /// An account can't exceed the `MaximumOwned` constant
   MaximumCollectiblesOwned,
   /// The total supply of collectibles can't exceed the u64 limit
   BoundsOverflow,
   /// The collectible doesn't exist
   NoCollectible,
   // You are not the owner
   NotOwner,
   /// Trying to transfer a collectible to yourself
   TransferToSelf,
	/// The bid is lower than the asking price.
	BidPriceTooLow,
	/// The collectible is not for sale.
	NotForSale,
 }
```

---

# Buy a collectible

- Add the Sold event to your Pallet.

```
// Pallet events
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
 pub enum Event<T: Config> {
   /// A new collectible was successfully created.
   CollectibleCreated { collectible: [u8; 16], owner: T::AccountId },
   /// A collectible was successfully transferred.
   TransferSucceeded { from: T::AccountId, to: T::AccountId, collectible: [u8; 16] },
   /// The price of a collectible was successfully set.
   PriceSet { collectible: [u8; 16], price: Option<BalanceOf<T>> },
   /// A collectible was successfully sold.
	Sold { seller: T::AccountId, buyer: T::AccountId, collectible: [u8; 16], price: BalanceOf<T> },
 }
```

---

# Buy a collectible

- Create an internal function to be called when users want to purchase a collectible.

```
// An internal function for purchasing a collectible
pub fn do_buy_collectible(
	  unique_id: [u8; 16],
	  to: T::AccountId,
	  bid_price: BalanceOf<T>,
) -> DispatchResult {

// Get the collectible from the storage map
let mut collectible = CollectibleMap::<T>::get(&unique_id).ok_or(Error::<T>::NoCollectible)?;
let from = collectible.owner;
   ensure!(from != to, Error::<T>::TransferToSelf);
	  let mut from_owned = OwnerOfCollectibles::<T>::get(&from);
	 
// Remove collectible from owned collectibles.
if let Some(ind) = from_owned.iter().position(|&id| id == unique_id) {
		from_owned.swap_remove(ind);
	} else {
		return Err(Error::<T>::NoCollectible.into())
	}
// Add collectible to owned collectible.
let mut to_owned = OwnerOfCollectibles::<T>::get(&to);
to_owned.try_push(unique_id).map_err(|_id| Error::<T>::MaximumCollectiblesOwned)?;
// Mutating state with a balance transfer, so nothing is allowed to fail after this.
if let Some(price) = collectible.price {
		ensure!(bid_price >= price, Error::<T>::BidPriceTooLow);
		// Transfer the amount from buyer to seller
		T::Currency::transfer(&to, &from, price, frame_support::traits::ExistenceRequirement::KeepAlive)?;
		// Deposit sold event
		Self::deposit_event(Event::Sold {
			seller: from.clone(),
			buyer: to.clone(),
			collectible: unique_id,
			price,
		});
} else {
	  return Err(Error::<T>::NotForSale.into())
}

// Transfer succeeded, update the collectible owner and reset the price to `None`.
collectible.owner = to.clone();
collectible.price = None;
// Write updates to storage
CollectibleMap::<T>::insert(&unique_id, collectible);
OwnerOfCollectibles::<T>::insert(&to, to_owned);
OwnerOfCollectibles::<T>::insert(&from, from_owned);
Self::deposit_event(Event::TransferSucceeded { from, to, collectible: unique_id });
Ok(())
}
```

---

# Buy a collectible

- Add a callable function to allow a user to purchase a collectible.

```
/// Buy a collectible. The bid price must be greater than or equal to the price
/// set by the collectible owner.
#[pallet::weight(0)]
pub fn buy_collectible(
	 origin: OriginFor<T>,
	 unique_id: [u8; 16],
	 bid_price: BalanceOf<T>,
) -> DispatchResult {
	 // Make sure the caller is from a signed origin
	 let buyer = ensure_signed(origin)?;
	 // Transfer the collectible from seller to buyer.
	 Self::do_buy_collectible(unique_id, buyer, bid_price)?;
Ok(())
}
```

---

# Buy a collectible

Verify that your program compiles without errors by running the following command:

```
cargo build --package collectibles
```

---

#



---