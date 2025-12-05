use async_graphql::{SimpleObject, ID};
use chrono::{DateTime, Utc};

use crate::db::models;

#[derive(SimpleObject, Clone)]
pub struct BranchType {
    pub id: ID,
    pub name: String,
}

impl From<models::Branch> for BranchType {
    fn from(b: models::Branch) -> Self {
        Self {
            id: ID(b.id.to_string()),
            name: b.name,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct TestbedType {
    pub id: ID,
    pub name: String,
}

impl From<models::Testbed> for TestbedType {
    fn from(t: models::Testbed) -> Self {
        Self {
            id: ID(t.id.to_string()),
            name: t.name,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct MeasureType {
    pub id: ID,
    pub name: String,
    pub units: Option<String>,
}

impl From<models::Measure> for MeasureType {
    fn from(m: models::Measure) -> Self {
        Self {
            id: ID(m.id.to_string()),
            name: m.name,
            units: m.units,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct BenchmarkType {
    pub id: ID,
    pub name: String,
}

impl From<models::Benchmark> for BenchmarkType {
    fn from(b: models::Benchmark) -> Self {
        Self {
            id: ID(b.id.to_string()),
            name: b.name,
        }
    }
}

#[derive(SimpleObject)]
pub struct ThresholdType {
    pub id: ID,
    pub branch_id: Option<ID>,
    pub testbed_id: Option<ID>,
    pub measure_id: ID,
    pub upper_boundary: Option<f64>,
    pub lower_boundary: Option<f64>,
    pub min_sample_size: i32,
    pub created_at: DateTime<Utc>,
}

impl From<models::Threshold> for ThresholdType {
    fn from(t: models::Threshold) -> Self {
        Self {
            id: ID(t.id.to_string()),
            branch_id: t.branch_id.map(|id| ID(id.to_string())),
            testbed_id: t.testbed_id.map(|id| ID(id.to_string())),
            measure_id: ID(t.measure_id.to_string()),
            upper_boundary: t.upper_boundary,
            lower_boundary: t.lower_boundary,
            min_sample_size: t.min_sample_size,
            created_at: t.created_at,
        }
    }
}
