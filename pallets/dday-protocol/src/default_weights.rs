use frame_support::weights::Weight;

pub trait WeightInfo {
    fn dday_transmission() -> Weight;
    fn cancel() -> Weight;
}

impl crate::WeightInfo for () {
    fn dday_transmission() -> Weight {
        0 // TODO
    }
    fn cancel() -> Weight {
        0 // TODO
    }
}
