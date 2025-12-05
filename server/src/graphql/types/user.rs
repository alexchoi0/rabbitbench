use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::ProjectType;
use crate::db::{models, queries};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct UserType {
    pub id: ID,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[ComplexObject]
impl UserType {
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<ProjectType>> {
        let pool = ctx.data::<PgPool>()?;
        let user_id = Uuid::parse_str(&self.id)?;
        let projects = queries::get_user_projects(pool, user_id).await?;
        Ok(projects.into_iter().map(ProjectType::from).collect())
    }

    async fn api_tokens(&self, ctx: &Context<'_>) -> Result<Vec<ApiTokenType>> {
        let pool = ctx.data::<PgPool>()?;
        let user_id = Uuid::parse_str(&self.id)?;
        let tokens = queries::get_user_api_tokens(pool, user_id).await?;
        Ok(tokens.into_iter().map(ApiTokenType::from).collect())
    }
}

impl From<models::User> for UserType {
    fn from(u: models::User) -> Self {
        Self {
            id: ID(u.id.to_string()),
            email: u.email,
            name: u.name,
            avatar_url: u.avatar_url,
            created_at: u.created_at,
        }
    }
}

#[derive(SimpleObject)]
pub struct ApiTokenType {
    pub id: ID,
    pub name: String,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<models::ApiToken> for ApiTokenType {
    fn from(t: models::ApiToken) -> Self {
        Self {
            id: ID(t.id.to_string()),
            name: t.name,
            last_used_at: t.last_used_at,
            created_at: t.created_at,
        }
    }
}

#[derive(SimpleObject)]
pub struct ApiTokenResult {
    pub token: ApiTokenType,
    pub secret: String,
}
