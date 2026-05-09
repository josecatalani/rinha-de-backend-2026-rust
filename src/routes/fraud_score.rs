use axum::{http::StatusCode, Json};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use crate::vectorizer::Vectorize;

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub amount: f32,
    pub installments: u8,
    pub requested_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct Customer {
    pub avg_amount: f32,
    pub tx_count_24h: u8,
    pub known_merchants: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Merchant {
    pub id: String,
    pub mcc: String,
    pub avg_amount: f32,
}

#[derive(Deserialize, Debug)]
pub struct Terminal {
    pub is_online: bool,
    pub card_present: bool,
    pub km_from_home: f32,
}

#[derive(Deserialize, Debug)]
pub struct LastTransaction {
    pub timestamp: DateTime<Utc>,
    pub km_from_current: f32,
}

#[derive(Deserialize, Debug)]
pub struct FraudScoreRequest {
  pub id: String,
  pub transaction: Transaction,
  pub customer: Customer,
  pub merchant: Merchant,
  pub terminal: Terminal,
  pub last_transaction: Option<LastTransaction>
}

#[derive(Serialize)]
pub struct FraudScoreResponse {
    pub approved: bool,
    pub score: f32,
}

pub async fn post(
    Json(body): Json<FraudScoreRequest>,
) -> (StatusCode, Json<FraudScoreResponse>) {

    let vectors = Vectorize::new(body);
    println!("{:#?}", vectors);
    // let score = calculate_score(&body);
    // let recommendation = match score {
    //     s if s >= 0.7 => "deny",
    //     s if s >= 0.4 => "review",
    //     _ => "approve",
    // };

    (
        StatusCode::OK,
        Json(FraudScoreResponse {
            approved: true,
            score: 1.0
        }),
    )
}

// fn calculate_score(req: &FraudScoreRequest) -> f32 {
//     if req.amount > 10_000.0 {
//         0.8
//     } else {
//         0.2
//     }
// }
