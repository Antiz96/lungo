//! Acquire inhibit file descriptor lock through logind
//!
//! The implementation is adapted from the "caffeine-applet" project
//! See https://github.com/Antiz96/lungo/blob/main/THIRD-PARTY-NOTICES.md for details

use anyhow::Context;
use std::os::fd::OwnedFd;
use zbus::blocking::Connection;
use zbus::zvariant::OwnedFd as ZbusFd;

pub fn acquire_inhibit() -> anyhow::Result<OwnedFd> {
    let conn = Connection::system().context("Failed to connect to the system D-Bus")?;

    let reply: ZbusFd = conn
        .call_method(
            Some("org.freedesktop.login1"),
            "/org/freedesktop/login1",
            Some("org.freedesktop.login1.Manager"),
            "Inhibit",
            &(
                "idle:sleep:handle-lid-switch",
                "Lungo",
                "Keeping the system awake",
                "block",
            ),
        )
        .context("Failed to call logind Inhibit()")?
        .body()
        .deserialize()
        .context("Failed to deserialize logind reply")?;

    Ok(reply.into())
}
