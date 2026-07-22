//! Systray applet implementation
//! Built with ksni:
//! https://github.com/iovxw/ksni
//! https://crates.io/crates/ksni

use ksni::TrayMethods;
use ksni::menu::*;
use log::{debug, info};
use std::future;
use std::os::fd::OwnedFd;
use std::process;
use tokio::task;

use crate::inhibit;

pub struct LungoTray {
    inhibit_fd: Option<OwnedFd>,
}

// Systray Applet implementation
impl ksni::Tray for LungoTray {
    // Set id
    fn id(&self) -> String {
        "Lungo".into()
    }

    // Set icon
    fn icon_name(&self) -> String {
        let icon = if self.inhibit_fd.is_some() {
            "lungo-on"
        } else {
            "lungo-off"
        };

        debug!("Icon set or updated: {icon}");
        icon.into()
    }

    // Set title
    fn title(&self) -> String {
        "Lungo".into()
    }

    // Set tooltip
    fn tool_tip(&self) -> ksni::ToolTip {
        ksni::ToolTip {
            title: "Lungo".into(),
            ..Default::default()
        }
    }

    // Toggle checkable menu entry on click (triggers inhibitor lock and changes icon accordingly)
    fn activate(&mut self, _x: i32, _y: i32) {
        self.toggle();
    }

    // Build menu
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        vec![
            // Checkable menu entry, reporting the current state and allowing to toggle the creation
            // or release of the inhibit file descriptor lock
            CheckmarkItem {
                label: "Enable".into(),
                checked: self.inhibit_fd.is_some(),
                activate: Box::new(|this: &mut Self| this.toggle()),
                ..Default::default()
            }
            .into(),
            // Exit button
            StandardItem {
                label: "Exit".into(),
                activate: Box::new(|_| {
                    info!("Exited on user request");
                    process::exit(0);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

// Helper to toggle the creation and releasing of the inhibit file descriptor lock
impl LungoTray {
    fn toggle(&mut self) {
        if self.inhibit_fd.is_some() {
            self.inhibit_fd = None;
            info!("Inhibitor disabled");
        } else {
            match task::block_in_place(inhibit::acquire_inhibit) {
                Ok(fd) => {
                    self.inhibit_fd = Some(fd);
                    info!("Inhibitor enabled");
                }
                Err(error) => {
                    log::error!("Unable to acquire inhibitor: {error}");
                }
            }
        }
    }
}

// Starting the Systray Applet
pub async fn run() {
    let tray = LungoTray { inhibit_fd: None };

    tray.spawn()
        .await
        .expect("Unable to start the systray applet");

    info!("Systray applet started");

    // Run forever
    future::pending().await
}
