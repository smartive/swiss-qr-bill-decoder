use crate::models::address::Address;
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct QRData {
    iban: String,
    recipient_address: Address,
    sender_address: Option<Address>,
    amount: Option<String>,
    currency: String,
    reference_type: String,
    reference: Option<String>,
    message: Option<String>,
}

impl QRData {
    pub(crate) fn new(
        iban: String,
        recipient_address: Address,
        sender_address: Option<Address>,
        amount: Option<String>,
        currency: String,
        reference_type: String,
        reference: Option<String>,
        message: Option<String>,
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
        }
    }
}
