use async_graphql::{Context, InputObject, Object, Result, ID};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::{generate_api_token, hash_api_token};
use crate::db::queries;
use crate::graphql::AuthContext;
use crate::graphql::types::*;
use crate::services::check_threshold;

pub struct MutationRoot;

#[derive(InputObject)]
pub struct CreateProjectInput {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    #[graphql(default = false)]
    pub public: bool,
}

#[derive(InputObject)]
pub struct UpdateProjectInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub public: Option<bool>,
}

#[derive(InputObject)]
pub struct MetricInput {
    pub benchmark: String,
    pub measure: String,
    pub value: f64,
    pub lower_value: Option<f64>,
    pub upper_value: Option<f64>,
}

#[derive(InputObject)]
pub struct CreateReportInput {
    pub project_slug: String,
    pub branch: String,
    pub testbed: String,
    pub git_hash: Option<String>,
    pub metrics: Vec<MetricInput>,
}

#[derive(InputObject)]
pub struct CreateThresholdInput {
    pub project_slug: String,
    pub branch_id: Option<ID>,
    pub testbed_id: Option<ID>,
    pub measure_id: ID,
    pub upper_boundary: Option<f64>,
    pub lower_boundary: Option<f64>,
    #[graphql(default = 2)]
    pub min_sample_size: i32,
}

#[derive(InputObject)]
pub struct UpdateThresholdInput {
    pub upper_boundary: Option<f64>,
    pub lower_boundary: Option<f64>,
    pub min_sample_size: Option<i32>,
}

#[Object]
impl MutationRoot {
    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> Result<ProjectType> {
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

        let project = queries::create_project(
            pool,
            user.id,
            &input.slug,
            &input.name,
            input.description.as_deref(),
            input.public,
        )
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e {
                if db_err.constraint() == Some("projects_user_id_slug_key") {
                    return async_graphql::Error::new(format!(
                        "A project with slug '{}' already exists",
                        input.slug
                    ));
                }
            }
            async_graphql::Error::new(e.to_string())
        })?;

        queries::get_or_create_measure(pool, project.id, "latency", Some("ns")).await?;

        Ok(ProjectType::from(project))
    }

    async fn update_project(
        &self,
        ctx: &Context<'_>,
        slug: String,
        input: UpdateProjectInput,
    ) -> Result<ProjectType> {
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

        let project = queries::get_project_by_slug(pool, user.id, &slug)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Project not found"))?;

        let updated = queries::update_project(
            pool,
            project.id,
            input.name.as_deref(),
            input.description.as_deref(),
            input.public,
        )
        .await?;

        Ok(ProjectType::from(updated))
    }

    async fn delete_project(&self, ctx: &Context<'_>, slug: String) -> Result<bool> {
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

        let project = queries::get_project_by_slug(pool, user.id, &slug)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Project not found"))?;

        Ok(queries::delete_project(pool, project.id).await?)
    }

    async fn create_report(&self, ctx: &Context<'_>, input: CreateReportInput) -> Result<ReportType> {
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

        let project = queries::get_project_by_slug(pool, user.id, &input.project_slug)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Project not found"))?;

        let branch = queries::get_or_create_branch(pool, project.id, &input.branch).await?;
        let testbed = queries::get_or_create_testbed(pool, project.id, &input.testbed).await?;

        let report = queries::create_report(
            pool,
            project.id,
            branch.id,
            testbed.id,
            input.git_hash.as_deref(),
        )
        .await?;

        for metric_input in input.metrics {
            let benchmark =
                queries::get_or_create_benchmark(pool, project.id, &metric_input.benchmark).await?;
            let measure =
                queries::get_or_create_measure(pool, project.id, &metric_input.measure, None)
                    .await?;

            let metric = queries::create_metric(
                pool,
                report.id,
                benchmark.id,
                measure.id,
                metric_input.value,
                metric_input.lower_value,
                metric_input.upper_value,
            )
            .await?;

            let thresholds = queries::get_applicable_thresholds(
                pool,
                project.id,
                branch.id,
                testbed.id,
                measure.id,
            )
            .await?;

            for threshold in thresholds {
                let baseline_values = queries::get_baseline_metrics(
                    pool,
                    project.id,
                    benchmark.id,
                    branch.id,
                    testbed.id,
                    measure.id,
                    threshold.min_sample_size,
                )
                .await?;

                if let Some(violation) = check_threshold(&threshold, metric.value, &baseline_values)
                {
                    queries::create_alert(
                        pool,
                        threshold.id,
                        metric.id,
                        violation.baseline_value,
                        violation.percent_change,
                    )
                    .await?;
                }
            }
        }

        Ok(ReportType::from(report))
    }

    async fn create_threshold(
        &self,
        ctx: &Context<'_>,
        input: CreateThresholdInput,
    ) -> Result<ThresholdType> {
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

        let project = queries::get_project_by_slug(pool, user.id, &input.project_slug)
            .await?
            .ok_or_else(|| async_graphql::Error::new("Project not found"))?;

        let branch_id = input
            .branch_id
            .as_ref()
            .and_then(|id| Uuid::parse_str(id.as_str()).ok());
        let testbed_id = input
            .testbed_id
            .as_ref()
            .and_then(|id| Uuid::parse_str(id.as_str()).ok());
        let measure_id = Uuid::parse_str(input.measure_id.as_str())?;

        let threshold = queries::create_threshold(
            pool,
            project.id,
            branch_id,
            testbed_id,
            measure_id,
            input.upper_boundary,
            input.lower_boundary,
            input.min_sample_size,
        )
        .await?;

        Ok(ThresholdType::from(threshold))
    }

    async fn update_threshold(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: UpdateThresholdInput,
    ) -> Result<ThresholdType> {
        let _auth = ctx
            .data_opt::<AuthContext>()
            .ok_or_else(|| async_graphql::Error::new("Not authenticated"))?;

        let pool = ctx.data::<PgPool>()?;
        let threshold_id = Uuid::parse_str(id.as_str())?;

        let threshold = queries::update_threshold(
            pool,
            threshold_id,
            input.upper_boundary,
            input.lower_boundary,
            input.min_sample_size,
        )
        .await?;

        Ok(ThresholdType::from(threshold))
    }

    async fn delete_threshold(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let _auth = ctx
            .data_opt::<AuthContext>()
            .ok_or_else(|| async_graphql::Error::new("Not authenticated"))?;

        let pool = ctx.data::<PgPool>()?;
        let threshold_id = Uuid::parse_str(id.as_str())?;

        Ok(queries::delete_threshold(pool, threshold_id).await?)
    }

    async fn dismiss_alert(&self, ctx: &Context<'_>, id: ID) -> Result<AlertType> {
        let _auth = ctx
            .data_opt::<AuthContext>()
            .ok_or_else(|| async_graphql::Error::new("Not authenticated"))?;

        let pool = ctx.data::<PgPool>()?;
        let alert_id = Uuid::parse_str(id.as_str())?;

        let alert = queries::dismiss_alert(pool, alert_id).await?;
        Ok(AlertType::from(alert))
    }

    async fn create_api_token(&self, ctx: &Context<'_>, name: String) -> Result<ApiTokenResult> {
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

        let secret = generate_api_token();
        let token_hash = hash_api_token(&secret);

        let token = queries::create_api_token(pool, user.id, &name, &token_hash).await?;

        Ok(ApiTokenResult {
            token: ApiTokenType::from(token),
            secret,
        })
    }

    async fn revoke_api_token(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
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

        let token_id = Uuid::parse_str(id.as_str())?;
        Ok(queries::delete_api_token(pool, token_id, user.id).await?)
    }
}
