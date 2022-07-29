# Substrate

These 5 main things you need to take care:-

1. #[pallet::pallet]
2. #[pallet::config]
3. #[pallet::storage] - 2 types storage value, hashmap
4. #[pallet::event] - show you the result mostly enums
5. #[pallet::error]
** These are similar to decorators in Python


#[pallet::call] - Adding this to make it extrinsics
#[pallet::weight(10_000) + T::DbWeight::get().writes(1)] - estimate gas fee. Note: Here 10_000 is in picoseconds
* Writing for loops is not encouraged on blockchain and writing recursions is completely banned. The longer the function executed, the higher the gas fee will be.
Extrinsics are functions use to interact with the node