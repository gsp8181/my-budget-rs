pub fn get_attributes(attributes: &str) -> Vec<&str> {
    attributes.split(',').collect::<Vec<_>>()
}
