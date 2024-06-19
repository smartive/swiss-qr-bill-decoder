use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Address {
    address_type: String,
    name: String,
    address_line_1: String,
    address_line_2: String,
    country: String,
}

impl Address {
    pub(crate) fn new(
        address_type: String,
        name: String,
        address_line_1: String,
        address_line_2: String,
        country: String,
    ) -> Self {
        Self {
            address_type,
            name,
            address_line_1,
            address_line_2,
            country,
        }
    }
}
