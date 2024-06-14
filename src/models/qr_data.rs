use crate::models::address::Address;

#[derive(Debug)]
pub struct QRData {
    iban: String,
    recipient_address: Address,
    sender_address: Option<Address>,
    amount: String,
    currency: String,
    reference_type: String,
    reference: String,
    message: String,
}

impl QRData {
    pub(crate) fn new(iban: String,
                      recipient_address: Address,
                      sender_address: Option<Address>,
                      amount: String,
                      currency: String,
                      reference_type: String,
                      reference: String,
                      message: String) -> Self {
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