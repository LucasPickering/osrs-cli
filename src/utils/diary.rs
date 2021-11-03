use derive_more::Display;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// The different tiers of achievement diaries
#[derive(
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    PartialOrd,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum AchievementDiaryLevel {
    Easy,
    Medium,
    Hard,
    Elite,
}
