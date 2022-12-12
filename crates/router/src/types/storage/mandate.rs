use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use time::PrimitiveDateTime;

use crate::schema::mandate;
// use serde::{Deserialize, Serialize};
use crate::{
    pii::{self, Secret},
    types::storage::enums as storage_enums,
};

#[derive(Clone, Debug, Identifiable, Queryable)]
#[diesel(table_name = mandate)]
pub struct Mandate {
    pub id: i32,
    pub mandate_id: String,
    pub customer_id: String,
    pub merchant_id: String,
    pub payment_method_id: String,
    pub mandate_status: storage_enums::MandateStatus,
    pub mandate_type: storage_enums::MandateType,
    pub customer_accepted_at: Option<PrimitiveDateTime>,
    pub customer_ip_address: Option<Secret<String, pii::IpAddress>>,
    pub customer_user_agent: Option<String>,
    pub network_transaction_id: Option<String>,
    pub previous_transaction_id: Option<String>,
    pub created_at: PrimitiveDateTime,
    pub mandate_amount: Option<i32>,
    pub mandate_currency: Option<storage_enums::Currency>,
    pub amount_captured: Option<i32>,
    pub connector: String,
    pub connector_mandate_id: Option<String>,
}

#[derive(
    router_derive::Setter, Clone, Debug, Default, Insertable, router_derive::DebugAsDisplay,
)]
#[diesel(table_name = mandate)]
pub struct MandateNew {
    pub mandate_id: String,
    pub customer_id: String,
    pub merchant_id: String,
    pub payment_method_id: String,
    pub mandate_status: storage_enums::MandateStatus,
    pub mandate_type: storage_enums::MandateType,
    pub customer_accepted_at: Option<PrimitiveDateTime>,
    pub customer_ip_address: Option<Secret<String, pii::IpAddress>>,
    pub customer_user_agent: Option<String>,
    pub network_transaction_id: Option<String>,
    pub previous_transaction_id: Option<String>,
    pub created_at: Option<PrimitiveDateTime>,
    pub mandate_amount: Option<i32>,
    pub mandate_currency: Option<storage_enums::Currency>,
    pub amount_captured: Option<i32>,
    pub connector: String,
    pub connector_mandate_id: Option<String>,
}

#[derive(Debug)]
pub enum MandateUpdate {
    StatusUpdate {
        mandate_status: storage_enums::MandateStatus,
    },
    CaptureAmountUpdate {
        amount_captured: Option<i32>,
    },
    ConnectorReferenceUpdate {
        connector_mandate_id: Option<String>,
    },
}

#[derive(Clone, Eq, PartialEq, Copy, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MandateAmountData {
    pub amount: i32,
    pub currency: storage_enums::Currency,
}

#[derive(Clone, Debug, Default, AsChangeset, router_derive::DebugAsDisplay)]
#[diesel(table_name = mandate)]
pub(super) struct MandateUpdateInternal {
    mandate_status: Option<storage_enums::MandateStatus>,
    amount_captured: Option<i32>,
    connector_mandate_id: Option<String>,
}

impl From<MandateUpdate> for MandateUpdateInternal {
    fn from(mandate_update: MandateUpdate) -> Self {
        match mandate_update {
            MandateUpdate::StatusUpdate { mandate_status } => Self {
                mandate_status: Some(mandate_status),
                connector_mandate_id: None,
                amount_captured: None,
            },
            MandateUpdate::CaptureAmountUpdate { amount_captured } => Self {
                mandate_status: None,
                amount_captured,
                connector_mandate_id: None,
            },
            MandateUpdate::ConnectorReferenceUpdate {
                connector_mandate_id,
            } => Self {
                connector_mandate_id,
                ..Default::default()
            },
        }
    }
}
