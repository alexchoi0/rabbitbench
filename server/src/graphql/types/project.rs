use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::{models, queries};
use super::{BranchType, TestbedType, MeasureType, BenchmarkType, ReportType, ThresholdType};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ProjectType {
    pub id: ID,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
    pub created_at: DateTime<Utc>,
}

#[ComplexObject]
impl ProjectType {
    async fn branches(&self, ctx: &Context<'_>) -> Result<Vec<BranchType>> {
        let pool = ctx.data::<PgPool>()?;
        let project_id = Uuid::parse_str(&self.id)?;
        let branches = queries::get_project_branches(pool, project_id).await?;
        Ok(branches.into_iter().map(BranchType::from).collect())
    }

    async fn testbeds(&self, ctx: &Context<'_>) -> Result<Vec<TestbedType>> {
        let pool = ctx.data::<PgPool>()?;
        let project_id = Uuid::parse_str(&self.id)?;
        let testbeds = queries::get_project_testbeds(pool, project_id).await?;
        Ok(testbeds.into_iter().map(TestbedType::from).collect())
    }

    async fn measures(&self, ctx: &Context<'_>) -> Result<Vec<MeasureType>> {
        let pool = ctx.data::<PgPool>()?;
        let project_id = Uuid::parse_str(&self.id)?;
        let measures = queries::get_project_measures(pool, project_id).await?;
        Ok(measures.into_iter().map(MeasureType::from).collect())
    }

    async fn benchmarks(&self, ctx: &Context<'_>) -> Result<Vec<BenchmarkType>> {
        let pool = ctx.data::<PgPool>()?;
        let project_id = Uuid::parse_str(&self.id)?;
        let benchmarks = queries::get_project_benchmarks(pool, project_id).await?;
        Ok(benchmarks.into_iter().map(BenchmarkType::from).collect())
    }

    async fn thresholds(&self, ctx: &Context<'_>) -> Result<Vec<ThresholdType>> {
        let pool = ctx.data::<PgPool>()?;
        let project_id = Uuid::parse_str(&self.id)?;
        let thresholds = queries::get_project_thresholds(pool, project_id).await?;
        Ok(thresholds.into_iter().map(ThresholdType::from).collect())
    }

    async fn recent_reports(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] limit: i32,
    ) -> Result<Vec<ReportType>> {
        let pool = ctx.data::<PgPool>()?;
        let project_id = Uuid::parse_str(&self.id)?;
        let reports = queries::get_project_reports(pool, project_id, limit as i64).await?;
        Ok(reports.into_iter().map(ReportType::from).collect())
    }
}

impl From<models::Project> for ProjectType {
    fn from(p: models::Project) -> Self {
        Self {
            id: ID(p.id.to_string()),
            slug: p.slug,
            name: p.name,
            description: p.description,
            public: p.public,
            created_at: p.created_at,
        }
    }
}
