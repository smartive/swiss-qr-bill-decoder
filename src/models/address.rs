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
        address_type: Option<&str>,
        name: Option<&str>,
        street_or_address_line_1: Option<&str>,
        building_number_or_address_line_2: Option<&str>,
        postal_code: Option<&str>,
        town: Option<&str>,
        country: Option<&str>,
    ) -> Result<Self, String> {
        let address_type = match address_type {
            None => return Err("Missing address type".to_string()),
            Some(address_type) if address_type.is_empty() => {
                return Err("Address type is empty".to_string())
            }
            Some(address_type) if !address_type.eq("K") && !address_type.eq("S") => {
                return Err("Only address types K and S are supported".to_string())
            }
            Some(address_type) => address_type.to_string(),
        };

        let name = match name {
            None => return Err("Missing name".to_string()),
            Some(name) if name.is_empty() => return Err("Recipient name is empty".to_string()),
            Some(name) => name.to_string(),
        };

        let street_or_address_line_1 = match street_or_address_line_1 {
            None => return Err("Missing street or address line 1".to_string()),
            Some(street_or_address_line_1) => street_or_address_line_1.to_string(),
        };

        let building_number_or_address_line_2 = match building_number_or_address_line_2 {
            None => return Err("Missing building number or address line 2".to_string()),
            Some(building_number_or_address_line_2) => {
                building_number_or_address_line_2.to_string()
            }
        };

        let postal_code = match postal_code {
            None => return Err("Missing postal code".to_string()),
            Some(postal_code) => postal_code.to_string(),
        };

        let town = match town {
            None => return Err("Missing town".to_string()),
            Some(town) => town.to_string(),
        };

        let country = match country {
            None => return Err("Missing country".to_string()),
            Some(country) => country.to_string(),
        };

        let address_line_1 = if address_type.eq("K") {
            street_or_address_line_1.to_string()
        } else {
            format!("{street_or_address_line_1} {building_number_or_address_line_2}")
        };

        let address_line_2 = if address_type.eq("K") {
            building_number_or_address_line_2.to_string()
        } else {
            format!("{postal_code} {town}")
        };

        Ok(Self {
            address_type,
            name,
            address_line_1,
            address_line_2,
            country,
        })
    }
}
