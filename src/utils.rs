use self::range_limit::RangeLimit;

pub mod limit_string;
pub mod multpart;

pub mod measureable;
pub mod range_limit;

pub struct Assert<const COND: bool>;

trait IsTrue {}

impl IsTrue for Assert<true> {}

pub type RangeLimitString<const L: usize, const H: usize> = RangeLimit<String, L, H>;
pub type MaxLimitString<const H: usize> = RangeLimit<String, 0, H>;

pub type RangeLimitVec<T,const L: usize, const H: usize>=RangeLimit<Vec<T>,L,H>;
