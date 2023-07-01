#[cfg(debug_assertions)]
fn get_version() -> String {
    return "Debug".to_owned();
}

#[cfg(not(debug_assertions))]
fn get_version() -> String {
    return "Release".to_owned();
}

pub fn get_name() -> String {
    return "John Client | ".to_owned() + &get_version();
}