use frame_support::weights::Weight;

pub trait WeightInfo {
    fn mint() -> Weight;
    fn burn() -> Weight;
    fn transfer_asset() -> Weight;
    fn transfer_token() -> Weight;
}