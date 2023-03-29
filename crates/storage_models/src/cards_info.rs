use diesel::{Identifiable, Queryable};

use crate::schema::cards_info;

#[derive(Clone, Debug, Queryable, Identifiable, serde::Deserialize, serde::Serialize)]
#[diesel(table_name = cards_info, primary_key(card_iin))]
pub struct CardInfo {
    pub card_iin: String,
    pub card_issuer: Option<String>,
    pub card_network: Option<String>,
    pub card_type: Option<String>,
    pub card_subtype: Option<String>,
    pub card_issuing_country: Option<String>,
    pub bank_code_id: Option<String>,
    pub bank_code: Option<String>,
    pub country_code: Option<String>,
}
