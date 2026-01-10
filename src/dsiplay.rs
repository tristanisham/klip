use std::env;
pub enum DisplayServer {
    X11,
    Wayland,
    Unknown,
}

pub fn get_server() -> DisplayServer {
    if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
        match session_type.as_str() {
            "wayland" => return DisplayServer::Wayland,
            "x11" => return DisplayServer::X11,
            _ => {}
        };
    }

    if env::var("WAYLAND_DISPLAY").is_ok() {
        return DisplayServer::Wayland;
    }

    if env::var("DISPLAY").is_ok() {
        return DisplayServer::X11;
    }

    DisplayServer::Unknown
}
