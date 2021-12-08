use self::range_limit::RangeLimit;

pub mod multpart;
pub mod cors;
pub mod measureable;
pub mod range_limit;
pub mod config;
pub mod crypto_string;


pub struct Assert<const COND: bool>;

trait IsTrue {}

impl IsTrue for Assert<true> {}

pub type RangeLimitString<const L: usize, const H: usize> = RangeLimit<String, L, H>;
pub type MaxLimitString<const H: usize> = RangeLimit<String, 0, H>;

pub type RangeLimitVec<T, const L: usize, const H: usize> = RangeLimit<Vec<T>, L, H>;
