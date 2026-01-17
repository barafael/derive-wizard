//! Min/max bounds example types
//!
//! Demonstrates:
//! - #[min(n)] attribute for minimum numeric value
//! - #[max(n)] attribute for maximum numeric value

use elicitor::Survey;

#[derive(Survey, Debug)]
pub struct GameSettings {
    #[ask("Set the difficulty level (1-10):")]
    #[min(1)]
    #[max(10)]
    pub difficulty: u32,

    #[ask("Set volume (0-100):")]
    #[min(0)]
    #[max(100)]
    pub volume: u32,

    #[ask("Set brightness (-50 to 50):")]
    #[min(-50)]
    #[max(50)]
    pub brightness: i32,

    #[ask("Set mouse sensitivity (1 to 10):")]
    #[min(1)]
    #[max(10)]
    pub sensitivity: u32,
}
