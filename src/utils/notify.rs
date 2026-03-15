use notify_rust::Notification;

use crate::config::constants::FRBW_ICON_NAME;
use crate::config::default::UserConfig;

// takes a summary and a body and sends a notification via notify-send
pub fn send_notification(cfg: &UserConfig, summary: &str, body: String) -> Result<(), Box<dyn std::error::Error>> {
    // return early if notifications are disabled
    if !cfg.notifications {
        return Ok(());
    }

    // Send notification
    Notification::new()
        .summary(summary)
        .body(body.as_str())
        .icon(FRBW_ICON_NAME)
        .show()?;

    Ok(())
}

#[test]
fn test_send_notification() {}
