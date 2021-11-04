use crate as pallet_audit;
// use sp_core::H256;
// use frame_support::sp_runtime::{
// 	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
// };
// use frame_system as system;
// use frame_support::parameter_types;
use frame_support::sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use sp_core::H256;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Audit: pallet_audit::{Module, Call, Storage, Event<T>},
	}
);

impl frame_system::Config for TestRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	// type OnSetCode = ();
}

impl pallet_audit::Config for TestRuntime {
	type Event = Event;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> frame_support::sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap().into()
}