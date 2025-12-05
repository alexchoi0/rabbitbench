use async_graphql::{Context, Object, Result, ID};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::db::queries;
use crate::graphql::types::*;
use crate::graphql::AuthContext;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let auth = ctx.data_opt::<AuthContext>();

        match auth {
            Some(auth) => {
                let pool = ctx.data::<PgPool>()?;

                let user = queries::upsert_user(
                    pool,
                    &auth.email,
                    auth.name.as_deref(),
                    auth.avatar_url.as_deref(),
                )
                .await?;

                Ok(Some(UserType::from(user)))
            }
            None => Ok(None),
        }
    }

    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<ProjectType>> {
        let auth = ctx
            .data_opt::<AuthContext>()
            .ok_or_else(|| async_graphql::Error::new("Not authenticated"))?;

        let pool = ctx.data::<PgPool>()?;

        let user = queries::upsert_user(
            pool,
            &auth.email,
            auth.name.as_deref(),
            auth.avatar_url.as_deref(),
        )
        .await?;

        let projects = queries::get_user_projects(pool, user.id).await?;
        Ok(projects.into_iter().map(ProjectType::from).collect())
    }

    async fn project(&self, ctx: &Context<'_>, slug: String) -> Result<Option<ProjectType>> {
        let auth = ctx
            .data_opt::<AuthContext>()
            .ok_or_else(|| async_graphql::Error::new("Not authenticated"))?;

        let pool = ctx.data::<PgPool>()?;

        let user = queries::upsert_user(
            pool,
            &auth.email,
            auth.name.as_deref(),
            auth.avatar_url.as_deref(),
        )
        .await?;

        let project = queries::get_project_by_slug(pool, user.id, &slug).await?;
        Ok(project.map(ProjectType::from))
    }

    async fn perf(
        &self,
        ctx: &Context<'_>,
        project_slug: String,
        benchmarks: Vec<ID>,
        branches: Vec<ID>,
        measures: Vec<ID>,
        testbeds: Vec<ID>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<PerfResult> {
        let auth = ctx
            .data_opt::<AuthContext>()
            .ok_or_else(|| async_graphql::Error::new("Not authenticated"))?;

        let pool = ctx.data::<PgPool>()?;

        let user = queries::upsert_user(
            pool,
            &auth.email,
            auth.name.as_deref(),
            auth.avatar_url.as_deref(),
        )
        .await?;

        let project = queries::get_project_by_slug(pool, user.id, &project_slug)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Project not found"))?;

        let benchmark_ids: Vec<Uuid> = benchmarks
            .iter()
            .filter_map(|id| Uuid::parse_str(id.as_str()).ok())
            .collect();
        let branch_ids: Vec<Uuid> = branches
            .iter()
            .filter_map(|id| Uuid::parse_str(id.as_str()).ok())
            .collect();
        let measure_ids: Vec<Uuid> = measures
            .iter()
            .filter_map(|id| Uuid::parse_str(id.as_str()).ok())
            .collect();
        let testbed_ids: Vec<Uuid> = testbeds
            .iter()
            .filter_map(|id| Uuid::parse_str(id.as_str()).ok())
            .collect();

        let data_points = queries::get_perf_data(
            pool,
            project.id,
            &benchmark_ids,
            &branch_ids,
            &testbed_ids,
            &measure_ids,
            start_date,
            end_date,
        )
        .await?;

        let mut series_map: HashMap<(Uuid, Uuid, Uuid, Uuid), Vec<PerfDataPoint>> = HashMap::new();

        for dp in data_points {
            let key = (dp.benchmark_id, dp.branch_id, dp.testbed_id, dp.measure_id);
            series_map.entry(key).or_default().push(PerfDataPoint {
                x: dp.created_at,
                y: dp.value,
                lower: dp.lower_value,
                upper: dp.upper_value,
                git_hash: dp.git_hash,
            });
        }

        let mut series = Vec::new();

        for ((benchmark_id, branch_id, testbed_id, measure_id), data) in series_map {
            let benchmark = queries::get_benchmark_by_id(pool, benchmark_id).await?;
            let branch = queries::get_branch_by_id(pool, branch_id).await?;
            let testbed = queries::get_testbed_by_id(pool, testbed_id).await?;
            let measure = queries::get_measure_by_id(pool, measure_id).await?;

            if let (Some(benchmark), Some(branch), Some(testbed), Some(measure)) =
                (benchmark, branch, testbed, measure)
            {
                series.push(PerfSeries {
                    benchmark: BenchmarkType::from(benchmark),
                    branch: BranchType::from(branch),
                    testbed: TestbedType::from(testbed),
                    measure: MeasureType::from(measure),
                    data,
                });
            }
        }

        Ok(PerfResult { series })
    }

    async fn alerts(
        &self,
        ctx: &Context<'_>,
        project_slug: String,
        status: Option<AlertStatus>,
    ) -> Result<Vec<AlertType>> {
        let auth = ctx
            .data_opt::<AuthContext>()
            .ok_or_else(|| async_graphql::Error::new("Not authenticated"))?;

        let pool = ctx.data::<PgPool>()?;

        let user = queries::upsert_user(
            pool,
            &auth.email,
            auth.name.as_deref(),
            auth.avatar_url.as_deref(),
        )
        .await?;

        let project = queries::get_project_by_slug(pool, user.id, &project_slug)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Project not found"))?;

        let status_str = status.map(|s| match s {
            AlertStatus::Active => "active",
            AlertStatus::Dismissed => "dismissed",
            AlertStatus::Resolved => "resolved",
        });

        let alerts = queries::get_project_alerts(pool, project.id, status_str).await?;
        Ok(alerts.into_iter().map(AlertType::from).collect())
    }
}
