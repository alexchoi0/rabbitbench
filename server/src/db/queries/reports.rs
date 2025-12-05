use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::{Alert, Metric, Report, Threshold};

pub async fn create_report(
    pool: &PgPool,
    project_id: Uuid,
    branch_id: Uuid,
    testbed_id: Uuid,
    git_hash: Option<&str>,
) -> Result<Report, sqlx::Error> {
    sqlx::query_as::<_, Report>(
        r#"
        INSERT INTO reports (project_id, branch_id, testbed_id, git_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(project_id)
    .bind(branch_id)
    .bind(testbed_id)
    .bind(git_hash)
    .fetch_one(pool)
    .await
}

pub async fn get_report_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Report>, sqlx::Error> {
    sqlx::query_as::<_, Report>("SELECT * FROM reports WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn get_project_reports(
    pool: &PgPool,
    project_id: Uuid,
    limit: i64,
) -> Result<Vec<Report>, sqlx::Error> {
    sqlx::query_as::<_, Report>(
        "SELECT * FROM reports WHERE project_id = $1 ORDER BY created_at DESC LIMIT $2",
    )
    .bind(project_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn create_metric(
    pool: &PgPool,
    report_id: Uuid,
    benchmark_id: Uuid,
    measure_id: Uuid,
    value: f64,
    lower_value: Option<f64>,
    upper_value: Option<f64>,
) -> Result<Metric, sqlx::Error> {
    sqlx::query_as::<_, Metric>(
        r#"
        INSERT INTO metrics (report_id, benchmark_id, measure_id, value, lower_value, upper_value)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(report_id)
    .bind(benchmark_id)
    .bind(measure_id)
    .bind(value)
    .bind(lower_value)
    .bind(upper_value)
    .fetch_one(pool)
    .await
}

pub async fn get_report_metrics(
    pool: &PgPool,
    report_id: Uuid,
) -> Result<Vec<Metric>, sqlx::Error> {
    sqlx::query_as::<_, Metric>("SELECT * FROM metrics WHERE report_id = $1")
        .bind(report_id)
        .fetch_all(pool)
        .await
}

pub async fn get_metric_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Metric>, sqlx::Error> {
    sqlx::query_as::<_, Metric>("SELECT * FROM metrics WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

#[derive(Debug, sqlx::FromRow)]
pub struct PerfDataPoint {
    pub benchmark_id: Uuid,
    pub branch_id: Uuid,
    pub testbed_id: Uuid,
    pub measure_id: Uuid,
    pub value: f64,
    pub lower_value: Option<f64>,
    pub upper_value: Option<f64>,
    pub git_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub async fn get_perf_data(
    pool: &PgPool,
    project_id: Uuid,
    benchmark_ids: &[Uuid],
    branch_ids: &[Uuid],
    testbed_ids: &[Uuid],
    measure_ids: &[Uuid],
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> Result<Vec<PerfDataPoint>, sqlx::Error> {
    sqlx::query_as::<_, PerfDataPoint>(
        r#"
        SELECT
            m.benchmark_id,
            r.branch_id,
            r.testbed_id,
            m.measure_id,
            m.value,
            m.lower_value,
            m.upper_value,
            r.git_hash,
            r.created_at
        FROM metrics m
        JOIN reports r ON m.report_id = r.id
        WHERE r.project_id = $1
            AND m.benchmark_id = ANY($2)
            AND r.branch_id = ANY($3)
            AND r.testbed_id = ANY($4)
            AND m.measure_id = ANY($5)
            AND ($6::timestamptz IS NULL OR r.created_at >= $6)
            AND ($7::timestamptz IS NULL OR r.created_at <= $7)
        ORDER BY r.created_at ASC
        "#,
    )
    .bind(project_id)
    .bind(benchmark_ids)
    .bind(branch_ids)
    .bind(testbed_ids)
    .bind(measure_ids)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

pub async fn get_baseline_metrics(
    pool: &PgPool,
    project_id: Uuid,
    benchmark_id: Uuid,
    branch_id: Uuid,
    testbed_id: Uuid,
    measure_id: Uuid,
    sample_size: i32,
) -> Result<Vec<f64>, sqlx::Error> {
    let rows = sqlx::query_scalar::<_, f64>(
        r#"
        SELECT m.value
        FROM metrics m
        JOIN reports r ON m.report_id = r.id
        WHERE r.project_id = $1
            AND m.benchmark_id = $2
            AND r.branch_id = $3
            AND r.testbed_id = $4
            AND m.measure_id = $5
        ORDER BY r.created_at DESC
        LIMIT $6
        "#,
    )
    .bind(project_id)
    .bind(benchmark_id)
    .bind(branch_id)
    .bind(testbed_id)
    .bind(measure_id)
    .bind(sample_size as i64)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_threshold(
    pool: &PgPool,
    project_id: Uuid,
    branch_id: Option<Uuid>,
    testbed_id: Option<Uuid>,
    measure_id: Uuid,
    upper_boundary: Option<f64>,
    lower_boundary: Option<f64>,
    min_sample_size: i32,
) -> Result<Threshold, sqlx::Error> {
    sqlx::query_as::<_, Threshold>(
        r#"
        INSERT INTO thresholds (project_id, branch_id, testbed_id, measure_id, upper_boundary, lower_boundary, min_sample_size)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
    )
    .bind(project_id)
    .bind(branch_id)
    .bind(testbed_id)
    .bind(measure_id)
    .bind(upper_boundary)
    .bind(lower_boundary)
    .bind(min_sample_size)
    .fetch_one(pool)
    .await
}

pub async fn get_project_thresholds(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<Threshold>, sqlx::Error> {
    sqlx::query_as::<_, Threshold>(
        "SELECT * FROM thresholds WHERE project_id = $1 ORDER BY created_at DESC",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
}

pub async fn get_applicable_thresholds(
    pool: &PgPool,
    project_id: Uuid,
    branch_id: Uuid,
    testbed_id: Uuid,
    measure_id: Uuid,
) -> Result<Vec<Threshold>, sqlx::Error> {
    sqlx::query_as::<_, Threshold>(
        r#"
        SELECT * FROM thresholds
        WHERE project_id = $1
            AND (branch_id IS NULL OR branch_id = $2)
            AND (testbed_id IS NULL OR testbed_id = $3)
            AND measure_id = $4
        "#,
    )
    .bind(project_id)
    .bind(branch_id)
    .bind(testbed_id)
    .bind(measure_id)
    .fetch_all(pool)
    .await
}

pub async fn update_threshold(
    pool: &PgPool,
    id: Uuid,
    upper_boundary: Option<f64>,
    lower_boundary: Option<f64>,
    min_sample_size: Option<i32>,
) -> Result<Threshold, sqlx::Error> {
    sqlx::query_as::<_, Threshold>(
        r#"
        UPDATE thresholds SET
            upper_boundary = COALESCE($2, upper_boundary),
            lower_boundary = COALESCE($3, lower_boundary),
            min_sample_size = COALESCE($4, min_sample_size),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(upper_boundary)
    .bind(lower_boundary)
    .bind(min_sample_size)
    .fetch_one(pool)
    .await
}

pub async fn delete_threshold(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM thresholds WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_threshold_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Threshold>, sqlx::Error> {
    sqlx::query_as::<_, Threshold>("SELECT * FROM thresholds WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create_alert(
    pool: &PgPool,
    threshold_id: Uuid,
    metric_id: Uuid,
    baseline_value: f64,
    percent_change: f64,
) -> Result<Alert, sqlx::Error> {
    sqlx::query_as::<_, Alert>(
        r#"
        INSERT INTO alerts (threshold_id, metric_id, baseline_value, percent_change)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(threshold_id)
    .bind(metric_id)
    .bind(baseline_value)
    .bind(percent_change)
    .fetch_one(pool)
    .await
}

pub async fn get_project_alerts(
    pool: &PgPool,
    project_id: Uuid,
    status: Option<&str>,
) -> Result<Vec<Alert>, sqlx::Error> {
    if let Some(status) = status {
        sqlx::query_as::<_, Alert>(
            r#"
            SELECT a.* FROM alerts a
            JOIN thresholds t ON a.threshold_id = t.id
            WHERE t.project_id = $1 AND a.status = $2
            ORDER BY a.created_at DESC
            "#,
        )
        .bind(project_id)
        .bind(status)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, Alert>(
            r#"
            SELECT a.* FROM alerts a
            JOIN thresholds t ON a.threshold_id = t.id
            WHERE t.project_id = $1
            ORDER BY a.created_at DESC
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
    }
}

pub async fn get_report_alerts(pool: &PgPool, report_id: Uuid) -> Result<Vec<Alert>, sqlx::Error> {
    sqlx::query_as::<_, Alert>(
        r#"
        SELECT a.* FROM alerts a
        JOIN metrics m ON a.metric_id = m.id
        WHERE m.report_id = $1
        ORDER BY a.created_at DESC
        "#,
    )
    .bind(report_id)
    .fetch_all(pool)
    .await
}

pub async fn dismiss_alert(pool: &PgPool, id: Uuid) -> Result<Alert, sqlx::Error> {
    sqlx::query_as::<_, Alert>("UPDATE alerts SET status = 'dismissed' WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn get_alert_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Alert>, sqlx::Error> {
    sqlx::query_as::<_, Alert>("SELECT * FROM alerts WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}
