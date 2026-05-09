use crate::routes::fraud_score::FraudScoreRequest;
use chrono::{Datelike, Timelike};
pub struct Vectorize;

pub mod limits {
    pub const MAX_AMOUNT: u32 = 10_000;
    pub const MAX_INSTALLMENTS: u8 = 12;
    pub const AMOUNT_VS_AVG_RATIO: u8 = 10;
    pub const MAX_MINUTES: u16 = 1440;
    pub const MAX_KM: u16 = 1000;
    pub const MAX_TX_COUNT_24H: u8 = 20;
    pub const MAX_MERCHANT_AVG_AMOUNT: u16 = 10_000;
}

impl Vectorize {
    pub fn new(payload: FraudScoreRequest) -> [f32; 14] {
        let amount = (payload.transaction.amount / limits::MAX_AMOUNT as f32).clamp(0.0, 1.0);

        let installments = (payload.transaction.installments as f32
            / limits::MAX_INSTALLMENTS as f32)
            .clamp(0.0, 1.0);

        let amount_vs_avg = ((payload.transaction.amount / payload.customer.avg_amount) as f32
            / limits::AMOUNT_VS_AVG_RATIO as f32)
            .clamp(0.0, 1.0);
        let hour_of_day = payload.transaction.requested_at.hour() as f32 / 23.0;

        let days_of_week = payload.transaction.requested_at.num_days_from_ce() as f32 / 6.0;

        let minutes_since_last_tx = match payload.last_transaction {
            Some(ref lt) => {
                (lt.timestamp.minute() as f32 / limits::MAX_MINUTES as f32).clamp(0.0, 1.0)
            }
            None => -1 as f32,
        };

        let km_from_last_tx = match payload.last_transaction {
            Some(ref lt) => (lt.km_from_current as f32 / limits::MAX_KM as f32).clamp(0.0, 1.0),
            None => -1 as f32,
        };

        let km_from_home =
            (payload.terminal.km_from_home as f32 / limits::MAX_KM as f32).clamp(0.0, 1.0);

        let tx_count_24h = (payload.customer.tx_count_24h as f32 / limits::MAX_TX_COUNT_24H as f32)
            .clamp(0.0, 1.0);

        let is_online = if payload.terminal.is_online {
            1.0_f32
        } else {
            0.0
        };

        let card_present = if payload.terminal.card_present {
            1.0_f32
        } else {
            0.0
        };

        let unknown_merchant = if payload
            .customer
            .known_merchants
            .contains(&payload.merchant.id)
        {
            0.0
        } else {
            1.0_f32
        };

        let mcc_risk = match payload.merchant.mcc.as_str() {
            "5411" => 0.15,
            "5812" => 0.30,
            "5912" => 0.20,
            "5944" => 0.45,
            "7801" => 0.80,
            "7802" => 0.75,
            "7995" => 0.85,
            "4511" => 0.35,
            "5311" => 0.25,
            "5999" => 0.50,
            _ => 0.50,
        };

        let merchant_avg_amount = (payload.merchant.avg_amount as f32
            / limits::MAX_MERCHANT_AVG_AMOUNT as f32)
            .clamp(0.0, 1.0);

        [
            amount,
            installments,
            amount_vs_avg,
            hour_of_day,
            days_of_week,
            minutes_since_last_tx,
            km_from_last_tx,
            km_from_home,
            tx_count_24h,
            is_online,
            card_present,
            unknown_merchant,
            mcc_risk,
            merchant_avg_amount,
        ]
    }
}
