use crate::core::{CreateUser, Profile, User, UserExtension};
use crate::data::Database;
use crate::AppState;
use axum::{
    debug_handler, extract::State, http::StatusCode, response::IntoResponse, Extension, Json,
};

#[debug_handler]
pub async fn profile(Extension(user_ext): Extension<UserExtension>) -> impl IntoResponse {
    if let Some(u) = user_ext.user {
        Json(Profile::from(u)).into_response()
    } else {
        Json("None").into_response()
    }
}

pub async fn create_user<D: Database>(
    State(state): State<AppState<D>>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    match user.r#type.as_str() {
        "superadmin" => {
            println!("creating user.");
            match User::create(state.db, &payload).await {
                Ok(_) => {
                    println!("user created successfully");
                    return (StatusCode::OK, "user created".into());
                }
                Err(_) => {
                    ("user creation failed");
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Error".into());
                }
            }
        }
        _ => (StatusCode::UNAUTHORIZED, "user creation not permitted"),
    }
}
