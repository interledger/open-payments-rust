pub fn pointer_to_url(pointer: &str) -> String {
    if pointer.starts_with('$') {
        format!("https://{}", &pointer[1..])
    } else {
        pointer.to_string()
    }
}
