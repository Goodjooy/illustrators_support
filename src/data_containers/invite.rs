use serde::{Deserialize, Serialize};

use crate::utils::{RangeLimitString, RangeLimitVec};

#[derive(Serialize, Deserialize)]
pub struct InviteCodeNew {
    codes: RangeLimitVec<RangeLimitString<8, 32>, 1, 3>,
}
