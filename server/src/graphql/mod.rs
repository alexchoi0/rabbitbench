pub mod mutation;
pub mod query;
pub mod schema;
pub mod types;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
};
use sqlx::PgPool;

use crate::auth::{extract_bearer_token, hash_api_token, validate_jwt};
use crate::db::queries;

pub use schema::AppSchema;

use mutation::MutationRoot;
use query::QueryRoot;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub schema: AppSchema,
    pub pool: PgPool,
    pub auth_secret: String,
    pub dev_mode: bool,
}

pub fn create_schema(pool: PgPool, auth_secret: String, dev_mode: bool) -> AppState {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .data(auth_secret.clone())
        .finish();

    AppState {
        schema,
        pool,
        auth_secret,
        dev_mode,
    }
}

pub async fn graphql_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = req.into_inner();

    if state.dev_mode {
        request = request.data(AuthContext {
            user_id: "dev-user-id".to_string(),
            email: "dev@localhost".to_string(),
            name: Some("Dev User".to_string()),
            avatar_url: None,
        });
        return state.schema.execute(request).await.into();
    }

    if let Some(auth_header) = headers.get("authorization").and_then(|v| v.to_str().ok()) {
        if let Some(token) = extract_bearer_token(auth_header) {
            if let Ok(claims) = validate_jwt(token, &state.auth_secret) {
                request = request.data(AuthContext {
                    user_id: claims.sub,
                    email: claims.email,
                    name: claims.name,
                    avatar_url: claims.picture,
                });
            } else {
                let token_hash = hash_api_token(token);
                if let Ok(Some(api_token)) =
                    queries::get_api_token_by_hash(&state.pool, &token_hash).await
                {
                    if let Ok(Some(user)) =
                        queries::get_user_by_id(&state.pool, api_token.user_id).await
                    {
                        let _ =
                            queries::update_api_token_last_used(&state.pool, api_token.id).await;

                        request = request.data(AuthContext {
                            user_id: user.id.to_string(),
                            email: user.email,
                            name: user.name,
                            avatar_url: user.avatar_url,
                        });
                    }
                }
            }
        }
    }

    state.schema.execute(request).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
    ))
}
