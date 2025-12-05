use async_graphql::{ComplexObject, Context, Enum, Result, SimpleObject, ID};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{models, queries};
use super::{BenchmarkType, BranchType, MeasureType, TestbedType, ThresholdType};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct MetricType {
    pub id: ID,
    #[graphql(skip)]
    pub benchmark_id: Uuid,
    #[graphql(skip)]
    pub measure_id: Uuid,
    pub value: f64,
    pub lower_value: Option<f64>,
    pub upper_value: Option<f64>,
}

#[ComplexObject]
impl MetricType {
    async fn benchmark(&self, ctx: &Context<'_>) -> Result<Option<BenchmarkType>> {
        let pool = ctx.data::<PgPool>()?;
        let benchmark = queries::get_benchmark_by_id(pool, self.benchmark_id).await?;
        Ok(benchmark.map(BenchmarkType::from))
    }

    async fn measure(&self, ctx: &Context<'_>) -> Result<Option<MeasureType>> {
        let pool = ctx.data::<PgPool>()?;
        let measure = queries::get_measure_by_id(pool, self.measure_id).await?;
        Ok(measure.map(MeasureType::from))
    }
}

impl From<models::Metric> for MetricType {
    fn from(m: models::Metric) -> Self {
        Self {
            id: ID(m.id.to_string()),
            benchmark_id: m.benchmark_id,
            measure_id: m.measure_id,
            value: m.value,
            lower_value: m.lower_value,
            upper_value: m.upper_value,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AlertStatus {
    Active,
    Dismissed,
    Resolved,
}

impl From<&str> for AlertStatus {
    fn from(s: &str) -> Self {
        match s {
            "dismissed" => AlertStatus::Dismissed,
            "resolved" => AlertStatus::Resolved,
            _ => AlertStatus::Active,
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct AlertType {
    pub id: ID,
    #[graphql(skip)]
    pub threshold_id: Uuid,
    #[graphql(skip)]
    pub metric_id: Uuid,
    pub baseline_value: f64,
    pub percent_change: f64,
    pub status: AlertStatus,
    pub created_at: DateTime<Utc>,
}

#[ComplexObject]
impl AlertType {
    async fn threshold(&self, ctx: &Context<'_>) -> Result<Option<ThresholdType>> {
        let pool = ctx.data::<PgPool>()?;
        let threshold = queries::get_threshold_by_id(pool, self.threshold_id).await?;
        Ok(threshold.map(ThresholdType::from))
    }

    async fn metric(&self, ctx: &Context<'_>) -> Result<Option<MetricType>> {
        let pool = ctx.data::<PgPool>()?;
        let metric = queries::get_metric_by_id(pool, self.metric_id).await?;
        Ok(metric.map(MetricType::from))
    }
}

impl From<models::Alert> for AlertType {
    fn from(a: models::Alert) -> Self {
        Self {
            id: ID(a.id.to_string()),
            threshold_id: a.threshold_id,
            metric_id: a.metric_id,
            baseline_value: a.baseline_value,
            percent_change: a.percent_change,
            status: AlertStatus::from(a.status.as_str()),
            created_at: a.created_at,
        }
    }
}

#[derive(SimpleObject)]
pub struct PerfResult {
    pub series: Vec<PerfSeries>,
}

#[derive(SimpleObject)]
pub struct PerfSeries {
    pub benchmark: BenchmarkType,
    pub branch: BranchType,
    pub testbed: TestbedType,
    pub measure: MeasureType,
    pub data: Vec<PerfDataPoint>,
}

#[derive(SimpleObject)]
pub struct PerfDataPoint {
    pub x: DateTime<Utc>,
    pub y: f64,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub git_hash: Option<String>,
}
