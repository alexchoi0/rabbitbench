use sqlx::PgPool;
use uuid::Uuid;

use crate::db::models::{Benchmark, Branch, Measure, Testbed};

pub async fn get_benchmark_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Benchmark>, sqlx::Error> {
    sqlx::query_as::<_, Benchmark>("SELECT * FROM benchmarks WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn get_branch_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Branch>, sqlx::Error> {
    sqlx::query_as::<_, Branch>("SELECT * FROM branches WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn get_testbed_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Testbed>, sqlx::Error> {
    sqlx::query_as::<_, Testbed>("SELECT * FROM testbeds WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn get_measure_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Measure>, sqlx::Error> {
    sqlx::query_as::<_, Measure>("SELECT * FROM measures WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}
