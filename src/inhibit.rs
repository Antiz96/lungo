//! Acquire inhibit file descriptor lock through logind

use std::os::fd::OwnedFd;
use zbus::blocking::Connection;
use zbus::zvariant::OwnedFd as ZbusFd;

pub fn acquire_inhibit() -> Result<OwnedFd, Box<dyn std::error::Error>> {
    let conn = Connection::system()?;

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
        )?
        .body()
        .deserialize()?;

    Ok(reply.into())
}
