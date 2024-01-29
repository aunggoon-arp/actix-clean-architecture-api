pub fn has_data<T>(data: Option<T>) -> bool {
    match data {
        Some(_dt) => true,
        None => false,
    }
}