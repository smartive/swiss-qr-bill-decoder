use crate::models::address::Address;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct QRData {
    iban: String,
    recipient_address: Address,
    sender_address: Option<Address>,
    amount: Option<String>,
    currency: String,
    reference_type: String,
    reference: Option<String>,
    message: Option<String>,
    billing_information: Option<String>,
}

impl QRData {

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        iban: String,
        recipient_address: Address,
        sender_address: Option<Address>,
        amount: Option<String>,
        currency: String,
        reference_type: String,
        reference: Option<String>,
        message: Option<String>,
        billing_information: Option<String>,
    ) -> Self {
        Self {
            iban,
            recipient_address,
            sender_address,
            amount,
            currency,
            reference_type,
            reference,
            message,
            billing_information,
        }
    }
}
