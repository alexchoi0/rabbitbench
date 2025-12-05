use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{models, queries};
use super::{BranchType, TestbedType, MetricType, AlertType};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ReportType {
    pub id: ID,
    #[graphql(skip)]
    pub branch_id: Uuid,
    #[graphql(skip)]
    pub testbed_id: Uuid,
    pub git_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[ComplexObject]
impl ReportType {
    async fn branch(&self, ctx: &Context<'_>) -> Result<Option<BranchType>> {
        let pool = ctx.data::<PgPool>()?;
        let branch = queries::get_branch_by_id(pool, self.branch_id).await?;
        Ok(branch.map(BranchType::from))
    }

    async fn testbed(&self, ctx: &Context<'_>) -> Result<Option<TestbedType>> {
        let pool = ctx.data::<PgPool>()?;
        let testbed = queries::get_testbed_by_id(pool, self.testbed_id).await?;
        Ok(testbed.map(TestbedType::from))
    }

    async fn metrics(&self, ctx: &Context<'_>) -> Result<Vec<MetricType>> {
        let pool = ctx.data::<PgPool>()?;
        let report_id = Uuid::parse_str(&self.id)?;
        let metrics = queries::get_report_metrics(pool, report_id).await?;
        Ok(metrics.into_iter().map(MetricType::from).collect())
    }

    async fn alerts(&self, ctx: &Context<'_>) -> Result<Vec<AlertType>> {
        let pool = ctx.data::<PgPool>()?;
        let report_id = Uuid::parse_str(&self.id)?;
        let alerts = queries::get_report_alerts(pool, report_id).await?;
        Ok(alerts.into_iter().map(AlertType::from).collect())
    }
}

impl From<models::Report> for ReportType {
    fn from(r: models::Report) -> Self {
        Self {
            id: ID(r.id.to_string()),
            branch_id: r.branch_id,
            testbed_id: r.testbed_id,
            git_hash: r.git_hash,
            created_at: r.created_at,
        }
    }
}
