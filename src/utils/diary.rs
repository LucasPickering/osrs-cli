use serde::{Deserialize, Serialize};
use strum::{EnumIter, ToString};

/// The different tiers of achievement diaries
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    EnumIter,
    ToString,
    Serialize,
    Deserialize,
)]
pub enum AchievementDiaryLevel {
    Easy,
    Medium,
    Hard,
    Elite,
}
