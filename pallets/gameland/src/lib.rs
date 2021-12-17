#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
use frame_system::pallet_prelude::*;
use codec::{Decode, Encode};
use sp_runtime::{
	RuntimeDebug,
};
use scale_info::TypeInfo;

//1. nft单个/多个， 2.fungible用currencies, 3.batch转用的utility
// #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
// pub enum TokenType {
// 	Nft,
// 	Fungible,
// }

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct NftInfo{ //要用tokenid作type
	pub daily_price : u128,
	pub duration: u128,
	pub collateral: u128,
	pub total_amount: u128,
}
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct BorrowInfo<T: Config> {
	pub borrower : T::AccountId,
	pub due_date: u128, 
}

pub use module::*;
#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        //type Gameland_account: Get<AccountId>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	//checked 查是否己有这功能？是有，但在平台上做transfer之后要转回去所以要做记录
	#[pallet::storage]
	#[pallet::getter(fn nft_owner)]
	pub(super) type Nft_owner<T> = StorageValue<T::AccountId, (ClassIdOf<T>, TokenIdOf<T>)>;//owner->token

    //checked
	#[pallet::storage]
	#[pallet::getter(fn nfts)]
	pub type Nfts<T> = StorageValue<(ClassIdOf<T>, TokenIdOf<T>), Nft_info>; //token->nft_info
	//checked
	#[pallet::storage]
	#[pallet::getter(fn nft_basic_status)]
	pub(super) type Nft_basic_status<T> = StorageValue<AccountId, Nft_info>; //owner->nft_info
	//checked
	#[pallet::storage]
	#[pallet::getter(fn nfts_count)]
	pub type Nfts_count<T> = StorageValue<_, u64>; //u64 要不要加个token->u64?
	//checked
	#[pallet::storage]
	#[pallet::getter(fn borrow_or_not)]
	pub(super) type Borrow_or_not<T> = StorageValue<(ClassIdOf<T>, TokenIdOf<T>), bool>; //token -> bool
	//checked
	#[pallet::storage]
	#[pallet::getter(fn all_nfts)]
	pub type All_nfts<T> = StorageValue<_, (ClassIdOf<T>, TokenIdOf<T>)>; //tokens
	//checked
	#[pallet::storage]
	#[pallet::getter(fn borrow_status)]
	pub(super) type Borrow_status<T> = StorageValue(ClassIdOf<T>, TokenIdOf<T>), Borrow_info>;//token -> borrow_info

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Deposited(AccountId, AccountId, (ClassIdOf<T>, TokenIdOf<T>)),//owner, Gameland_account, nft_id
		Withdrew(AccountId, AccountId, (ClassIdOf<T>, TokenIdOf<T>)),//Gameland_account, owner, nft_id
		Rented((ClassIdOf<T>, TokenIdOf<T>), AccountId, AccountId),//nft_id, borrower, owner
		Returned((ClassIdOf<T>, TokenIdOf<T>), AccountId, AccountId),//nft_id, owner, borrower
		Confiscated((ClassIdOf<T>, TokenIdOf<T>), AccountId),//nft_id, owner
	}

    // parameter_types! {
	// 	pub const Gameland_account: AccountId = xxxxxx;
	// }
	#[pallet::constant]
	type Gameland_account: Get<AccountId>;

	#[pallet::error]
	pub enum Error<T> {
		// NoneValue,
		// StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//单个
		#[pallet::weight(100)]
		pub fn deposit(origin: OriginFor<T>, token: (ClassIdOf<T>, TokenIdOf<T>), daily_price: Balance, duration: Balance, collateral: Balance) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			let total_amount = daily_price * duration + collateral;
			let new_nft = Nft_info<T: Config> {
				daily_price,
				duration,
				collateral,
				total_amount,
			};
			//All_nfts上加token
			let new_nft_count = Self::nft_count().checked_add(1).ok_or(<Error<T>>::NftCntOverflow)?; 
			<Nfts_count<T>>::put(new_nft_count);
			//在Nft_owner上加(owner, token)
			<Nft_owner<T>>::try_mutate(&owner, |nft_vec| {nft_vec.try_push(token)}).map_err(|_| <Error<T>>::ExceedMaxNftOwned)?;
			//在Nfts上加(token,Nft_info)
			//在Nft_basic_status加(owner, Nft_info)
			//不用转ownership先
			<Borrow_or_not<T>>::mutate_exists(nft_id), |bool| *bool = false);
			Self::deposit_event(Event::Deposited(owner, Gameland_account, token));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn withdraw_nft(origin: OriginFor<T>, token: (ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult {
			let withdrawer = ensure_signed(origin)?;
			let owner = <Nft_owner<T>>::get(&token);
			ensure!(owner == withdrawer, "You are not the owner");
			ensure!(<Borrow_or_not<T>>::get(&token) == false, "The nft is borrowed");	
			let new_nft_count = Self::nft_count().checked_sub(1).ok_or(<Error<T>>::NftCntOverflow)?; 
			<Nfts_count<T>>::put(new_nft_count);
			<Nft_owner<T>>::remove(token);
			//Nfts上remove(token,Nft_info)
			//All_nfts上remove token
			<Nft_basic_status<T>>::remove(token));
			<Borrow_or_not<T>>::remove(token);
			Self::deposit_event(Event::Withdrew(Gameland_account, owner, token));
			Ok(())
		}
		//如何确保转账的原子性？
		#[pallet::weight(100)]
		pub fn borrow((origin: OriginFor<T>, token: (ClassIdOf<T>, TokenIdOf<T>), amount: u32) -> DispatchResult {
			let borrower = ensure_signed(origin)?;
			ensure!(<Nft<T>>::get(&token).total_amount <= amount, "not enough amount");//如果发多了呢？
			ensure!(<Borrow_or_not<T>>::get(&token) == false, "The nft is borrowed");
			//::transfer(owner, borrower, token)? 这步己经做好是本人owner校验 
			//ownership不要改
			let price = <Nft<T>>::get(&token).daily_price * <Nft<T>>::get(&token).duration;
			let amount_to_owner = price - ((price * 3) / 100;)
			let amount_to_gameland = <Nft<T>>::get(&token).total_amount - amount_to_owner;
			//T::Currency::transfer()
			// amount_to_owner
			//amount_to_gameland
			let due_date = <Nft<T>>::get(&token).duration + <frame_system::Pallet<T>>::block_number();
			<Borrow_info<T>>::insert(borrower, due_date);
			<Borrow_or_not<T>>::mutate_exists(token), |bool| *bool = true);
			//Borrow_status 加token, borrower,duetime
			Self::deposit_event(Event::Rented(token, borrower, owner));
			Ok(())
		}
		#[pallet::weight(100)]
		//还完后就从nftlist 那除去，如果要再出租就deposit一次
		pub fn _return(origin: OriginFor<T>, token: (ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult {
			let borrower = ensure_signed(origin)?;
			ensure!(<Borrow_or_not<T>>::get(&token) == true, "the nft has not been borrowed");
			//::transfer(owner, borrower, token)? 这步己经做好是本人owner校验 
			//ownership不要改
			//T::Currency::transfer() 还collateral给borrower
			<Nft_basic_status<T>>::remove(token);
			<Borrow_or_not<T>>::remove(token);
			//All_nfts上remove token
			Self::deposit_event(Event::Returned(token, owner, borrower));//owner哪里来？
			Ok(())
		}
		#[pallet::weight(100)]
		pub fn confiscate(origin: OriginFor<T>, token: (ClassIdOf<T>, TokenIdOf<T>)) -> DispatchResult {
			ensure!(<Borrow_or_not<T>>::get(&token) == true, "the nft has not been borrowed");
			let caller = ensure_signed(origin)?;
			ensure!(<Nft_owner<T>>::get(token) == caller, "Only owner can confiscate");			
			ensure!(<Borrow_info<T>>::get(nft_id).due_date == <frame_system::Pallet<T>>::block_number(), "Not yet");
			//T::Currency::transfer() 还collateral转给owner
			//改owner成borrower
			<Nft_basic_status<T>>::remove(nft_id);
			<Borrow_or_not<T>>::remove(nft_id);
			//All_nfts上remove token
			//Nfts_count要减一
			//Borrow_status也要改
			Self::deposit_event(Event::Confiscated(token, caller));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn get_all_nfts() -> Vec<(ClassIdOf<T>, TokenIdOf<T>)> {
			for i in All_nfts.len {
				//println!
			}
		}
		#[pallet::weight(100)]
		//单个token的info
		pub fn get_nft_info(token: (ClassIdOf<T>, TokenIdOf<T>)) -> Nft_info {
			//Nfts
		}
		#[pallet::weight(100)]
		////Get the borrower, due_date
		pub fn get_borrow_info(token: (ClassIdOf<T>, TokenIdOf<T>)) -> Borrow_info {
			
		}
		#[pallet::weight(100)]
		pub fn check_the_borrow_status(token: (ClassIdOf<T>, TokenIdOf<T>)) -> bool {
			
		}
		#[pallet::weight(100)]
		//Query the owner of a NFT
		pub fn query_the_nft_owner(token: (ClassIdOf<T>, TokenIdOf<T>)) -> AccountId {
			
		}
	}
}