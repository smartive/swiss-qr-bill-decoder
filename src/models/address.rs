#[derive(Debug)]
pub struct Address {
    address_type: String,
    name: String,
    address_line_1: String,
    address_line_2: String,
    country: String,
}

impl Address {
    pub(crate) fn new(address_type: Option<&str>,
                      name: Option<&str>,
                      street_or_address_line_1: Option<&str>,
                      building_number_or_address_line_2: Option<&str>,
                      postal_code: Option<&str>,
                      town: Option<&str>,
                      country: Option<&str>) -> Self {
        let address_type = address_type.expect("Missing address type").to_string();
        assert!(address_type.eq("K") || address_type.eq("S"), "Only address types K and S are supported");

        let name = name.expect("Missing name").to_string();
        assert!(!name.is_empty(), "Recipient name is empty");

        let street_or_address_line_1 = street_or_address_line_1.expect("Missing street or address line 1");
        let building_number_or_address_line_2 = building_number_or_address_line_2.expect("Missing building number or address line 2");
        let postal_code = postal_code.expect("Missing postal code");
        let town = town.expect("Missing town");
        let country = country.expect("Missing country").to_string();

        let address_line_1 = if address_type.eq("K") {
            format!("{street_or_address_line_1}")
        } else {
            format!("{street_or_address_line_1} {building_number_or_address_line_2}")
        };

        let address_line_2 = if address_type.eq("K") {
            format!("{building_number_or_address_line_2}")
        } else {
            format!("{postal_code} {town}")
        };

        Self {
            address_type,
            name,
            address_line_1,
            address_line_2,
            country,
        }
    }
}