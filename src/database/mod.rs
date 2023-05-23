use chrono::{
    DateTime,
    Utc,
};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Expense {
    timestamp: DateTime<Utc>,
    category: String,
    description: String,
    value: u64,
}

pub(crate) fn get_expense_list() -> Result<Vec<Expense>, ()> {
    Ok(vec![])
}

pub(crate) fn get_expense(_id: &str) -> Result<Expense, ()> {
    Ok(Expense {
        timestamp: Utc::now(),
        category: "Montly bill".to_string(),
        description: "Mortgage".to_string(),
        value: 80000,
    })
}
