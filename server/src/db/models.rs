use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ApiToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub token_hash: String,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub user_id: Uuid,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Branch {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Testbed {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Measure {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub units: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Benchmark {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Report {
    pub id: Uuid,
    pub project_id: Uuid,
    pub branch_id: Uuid,
    pub testbed_id: Uuid,
    pub git_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Metric {
    pub id: Uuid,
    pub report_id: Uuid,
    pub benchmark_id: Uuid,
    pub measure_id: Uuid,
    pub value: f64,
    pub lower_value: Option<f64>,
    pub upper_value: Option<f64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Threshold {
    pub id: Uuid,
    pub project_id: Uuid,
    pub branch_id: Option<Uuid>,
    pub testbed_id: Option<Uuid>,
    pub measure_id: Uuid,
    pub upper_boundary: Option<f64>,
    pub lower_boundary: Option<f64>,
    pub min_sample_size: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Alert {
    pub id: Uuid,
    pub threshold_id: Uuid,
    pub metric_id: Uuid,
    pub baseline_value: f64,
    pub percent_change: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
