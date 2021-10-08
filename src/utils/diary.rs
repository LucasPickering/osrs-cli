use serde::{Deserialize, Serialize};

/// The different tiers of achievement diaries
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum AchievementDiaryLevel {
    Easy,
    Medium,
    Hard,
    Elite,
}
