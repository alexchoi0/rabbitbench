use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::{Branch, Measure, Project, Testbed, Benchmark};

pub async fn create_project(
    pool: &PgPool,
    user_id: Uuid,
    slug: &str,
    name: &str,
    description: Option<&str>,
    public: bool,
) -> Result<Project, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        r#"
        INSERT INTO projects (user_id, slug, name, description, public)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(slug)
    .bind(name)
    .bind(description)
    .bind(public)
    .fetch_one(pool)
    .await
}

pub async fn get_project_by_slug(
    pool: &PgPool,
    user_id: Uuid,
    slug: &str,
) -> Result<Option<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        "SELECT * FROM projects WHERE user_id = $1 AND slug = $2",
    )
    .bind(user_id)
    .bind(slug)
    .fetch_optional(pool)
    .await
}

pub async fn get_project_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn get_user_projects(pool: &PgPool, user_id: Uuid) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        "SELECT * FROM projects WHERE user_id = $1 ORDER BY updated_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn update_project(
    pool: &PgPool,
    id: Uuid,
    name: Option<&str>,
    description: Option<&str>,
    public: Option<bool>,
) -> Result<Project, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        r#"
        UPDATE projects SET
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            public = COALESCE($4, public),
            updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(description)
    .bind(public)
    .fetch_one(pool)
    .await
}

pub async fn delete_project(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn get_or_create_branch(
    pool: &PgPool,
    project_id: Uuid,
    name: &str,
) -> Result<Branch, sqlx::Error> {
    sqlx::query_as::<_, Branch>(
        r#"
        INSERT INTO branches (project_id, name)
        VALUES ($1, $2)
        ON CONFLICT (project_id, name) DO UPDATE SET name = EXCLUDED.name
        RETURNING *
        "#,
    )
    .bind(project_id)
    .bind(name)
    .fetch_one(pool)
    .await
}

pub async fn get_project_branches(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<Branch>, sqlx::Error> {
    sqlx::query_as::<_, Branch>(
        "SELECT * FROM branches WHERE project_id = $1 ORDER BY name",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
}

pub async fn get_or_create_testbed(
    pool: &PgPool,
    project_id: Uuid,
    name: &str,
) -> Result<Testbed, sqlx::Error> {
    sqlx::query_as::<_, Testbed>(
        r#"
        INSERT INTO testbeds (project_id, name)
        VALUES ($1, $2)
        ON CONFLICT (project_id, name) DO UPDATE SET name = EXCLUDED.name
        RETURNING *
        "#,
    )
    .bind(project_id)
    .bind(name)
    .fetch_one(pool)
    .await
}

pub async fn get_project_testbeds(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<Testbed>, sqlx::Error> {
    sqlx::query_as::<_, Testbed>(
        "SELECT * FROM testbeds WHERE project_id = $1 ORDER BY name",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
}

pub async fn get_or_create_measure(
    pool: &PgPool,
    project_id: Uuid,
    name: &str,
    units: Option<&str>,
) -> Result<Measure, sqlx::Error> {
    sqlx::query_as::<_, Measure>(
        r#"
        INSERT INTO measures (project_id, name, units)
        VALUES ($1, $2, $3)
        ON CONFLICT (project_id, name) DO UPDATE SET
            units = COALESCE(EXCLUDED.units, measures.units)
        RETURNING *
        "#,
    )
    .bind(project_id)
    .bind(name)
    .bind(units)
    .fetch_one(pool)
    .await
}

pub async fn get_project_measures(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<Measure>, sqlx::Error> {
    sqlx::query_as::<_, Measure>(
        "SELECT * FROM measures WHERE project_id = $1 ORDER BY name",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
}

pub async fn get_or_create_benchmark(
    pool: &PgPool,
    project_id: Uuid,
    name: &str,
) -> Result<Benchmark, sqlx::Error> {
    sqlx::query_as::<_, Benchmark>(
        r#"
        INSERT INTO benchmarks (project_id, name)
        VALUES ($1, $2)
        ON CONFLICT (project_id, name) DO UPDATE SET name = EXCLUDED.name
        RETURNING *
        "#,
    )
    .bind(project_id)
    .bind(name)
    .fetch_one(pool)
    .await
}

pub async fn get_project_benchmarks(
    pool: &PgPool,
    project_id: Uuid,
) -> Result<Vec<Benchmark>, sqlx::Error> {
    sqlx::query_as::<_, Benchmark>(
        "SELECT * FROM benchmarks WHERE project_id = $1 ORDER BY name",
    )
    .bind(project_id)
    .fetch_all(pool)
    .await
}
