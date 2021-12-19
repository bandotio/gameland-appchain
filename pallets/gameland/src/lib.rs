#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use scale_info::TypeInfo;
//use sp_runtime::traits:: Zero;
use sp_std::vec::Vec;

pub use pallet::*;

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, Default)]
pub struct NftInfo {
	pub daily_price: u128,
	pub duration: u128,
	pub collateral: u128,
	#[codec(compact)]
	pub total_amount: u128,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, Default)]
pub struct BorrowInfo {//<T: Config>
	pub borrower: Vec<u8>,//T::AccountId
	pub due_date: u128, //T::BlockNumber
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config +  pallet_ormlnft::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// #[pallet::constant]
		// type GamelandAccount: Get<Self::AccountId>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Deposited(T::AccountId, T::AccountId, (T::ClassId, T::TokenId)),//owner, Gameland_account, token
		Withdrew(T::AccountId, T::AccountId, (T::ClassId, T::TokenId)),//Gameland_account, owner, token
		Rented((T::ClassId, T::TokenId), T::AccountId, T::AccountId),//token, borrower, owner
		Returned((T::ClassId, T::TokenId), T::AccountId, T::AccountId),//token, owner, borrower
		Confiscated((T::ClassId, T::TokenId), T::AccountId),//token, owner
	}

	#[pallet::error]
	pub enum Error<T> {}


	#[pallet::storage]
	#[pallet::getter(fn nft_owner)]//owner->token
	pub type NftOwner<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (T::ClassId, T::TokenId), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nfts)]//token->nft_info
	pub type Nfts<T:Config> = StorageMap<_, Blake2_128Concat, (T::ClassId, T::TokenId), NftInfo, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nft_basic_status)]//owner->nft_info
	pub type NftBasicStatus<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, NftInfo, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nfts_count)]
	pub type NftsCount<T:Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn borrow_or_not)]//token -> bool
	pub type BorrowOrNot<T:Config> = StorageMap<_, Blake2_128Concat, (T::ClassId, T::TokenId), bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn all_nfts)]//tokens
	pub type AllNfts<T:Config> = StorageValue<_, (T::ClassId, T::TokenId), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn borrow_status)]//token -> borrow_info
	pub type BorrowStatus<T:Config> = StorageMap<_, Blake2_128Concat, (T::ClassId, T::TokenId), BorrowInfo, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100)]
		pub fn deposit(origin: OriginFor<T>, _token: (T::ClassId, T::TokenId), daily_price: u128, duration: u128, collateral: u128) -> DispatchResult{
			let _owner = ensure_signed(origin)?;
			let total_amount = daily_price * duration + collateral;
			let _new_nft = NftInfo {
				daily_price,
				duration,
				collateral,
				total_amount,
			};
			// //All_nfts上加token
			// let new_nft_count = Self::nft_count().checked_add(1).ok_or(<Error<T>>::NftCntOverflow)?; 
			// <Nfts_count<T>>::put(new_nft_count);
			// //在Nft_owner上加(owner, token)
			// <Nft_owner<T>>::try_mutate(&owner, |nft_vec| {nft_vec.try_push(token)}).map_err(|_| <Error<T>>::ExceedMaxNftOwned)?;
			// //在Nfts上加(token,Nft_info)
			// //在Nft_basic_status加(owner, Nft_info)
			// //不用转ownership先
			// <Borrow_or_not<T>>::mutate_exists(nft_id), |bool| *bool = false);
			// Self::deposit_event(Event::Deposited(owner, Gameland_account, token));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn withdraw_nft(origin: OriginFor<T>, _token: (T::ClassId, T::TokenId)) -> DispatchResult{
			let _withdrawer = ensure_signed(origin)?;
			// let owner = <Nft_owner<T>>::get(&token);
			// ensure!(owner == withdrawer, "You are not the owner");
			// ensure!(<Borrow_or_not<T>>::get(&token) == false, "The nft is borrowed");	
			// let new_nft_count = Self::nft_count().checked_sub(1).ok_or(<Error<T>>::NftCntOverflow)?; 
			// <Nfts_count<T>>::put(new_nft_count);
			// <Nft_owner<T>>::remove(token);
			// //Nfts上remove(token,Nft_info)
			// //All_nfts上remove token
			// <Nft_basic_status<T>>::remove(token));
			// <Borrow_or_not<T>>::remove(token);
			// Self::deposit_event(Event::Withdrew(Gameland_account, owner, token));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn borrow(origin: OriginFor<T>, _token: (T::ClassId, T::TokenId), _amount: u128) -> DispatchResult {
			let _borrower = ensure_signed(origin)?;
			// ensure!(<Nft<T>>::get(&token).total_amount <= amount, "not enough amount");//如果发多了呢？
			// ensure!(<Borrow_or_not<T>>::get(&token) == false, "The nft is borrowed");
			// //::transfer(owner, borrower, token)? 这步己经做好是本人owner校验 
			// //ownership不要改
			// let price = <Nft<T>>::get(&token).daily_price * <Nft<T>>::get(&token).duration;
			// let amount_to_owner = price - ((price * 3) / 100;)
			// let amount_to_gameland = <Nft<T>>::get(&token).total_amount - amount_to_owner;
			// //T::Currency::transfer()
			// // amount_to_owner
			// //amount_to_gameland
			// let due_date = <Nft<T>>::get(&token).duration + <frame_system::Pallet<T>>::block_number();
			// <Borrow_info<T>>::insert(borrower, due_date);
			// <Borrow_or_not<T>>::mutate_exists(token), |bool| *bool = true);
			// //Borrow_status 加token, borrower,duetime
			// Self::deposit_event(Event::Rented(token, borrower, owner));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn _return(origin: OriginFor<T>,  _token: (T::ClassId, T::TokenId)) -> DispatchResult {
			let _borrower = ensure_signed(origin)?;
			// ensure!(<Borrow_or_not<T>>::get(&token) == true, "the nft has not been borrowed");
			// //::transfer(owner, borrower, token)? 这步己经做好是本人owner校验 
			// //ownership不要改
			// //T::Currency::transfer() 还collateral给borrower
			// <Nft_basic_status<T>>::remove(token);
			// <Borrow_or_not<T>>::remove(token);
			// //All_nfts上remove token
			// Self::deposit_event(Event::Returned(token, owner, borrower));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn confiscate(origin: OriginFor<T>, _token: (T::ClassId, T::TokenId)) -> DispatchResult {
			// ensure!(<Borrow_or_not<T>>::get(&token) == true, "the nft has not been borrowed");
			let _caller = ensure_signed(origin)?;
			// ensure!(<Nft_owner<T>>::get(token) == caller, "Only owner can confiscate");			
			// ensure!(<Borrow_info<T>>::get(nft_id).due_date == <frame_system::Pallet<T>>::block_number(), "Not yet");
			// //T::Currency::transfer() 还collateral转给owner
			// //改owner成borrower
			// <Nft_basic_status<T>>::remove(nft_id);
			// <Borrow_or_not<T>>::remove(nft_id);
			// //All_nfts上remove token
			// //Nfts_count要减一
			// //Borrow_status也要改
			// Self::deposit_event(Event::Confiscated(token, caller));
			Ok(())
		}
	}


	impl<T: Config> Pallet<T> {
		//Vec<(T::ClassId, T::TokenId)>
		pub fn get_all_nfts() {}
		//Borrow_info
		pub fn get_borrow_info(token: (T::ClassId, T::TokenId)){}
		pub fn check_the_borrow_status(token: (T::ClassId, T::TokenId)) -> bool {
			return true;
		}
		//Query the owner of a NFT
		pub fn query_the_nft_owner(token: (T::ClassId, T::TokenId)) {}
	}
}