use crate::{
    db::DbPool,
    models::post::{Post, PostRequest},
    response::server_response::ServerResponse,
};
use axum::{Extension, Json, extract::State, response::IntoResponse};
use rand::Rng;
use regex::Regex;

use rand::distr::Alphanumeric;
use rand::rngs::ThreadRng;

use uuid::Uuid;

const MAX_TITLE_LENGTH: usize = 70;
const MIN_TITLE_LENGTH: usize = 10;
const MAX_CONTENT_LENGTH: usize = 2500;
const MIN_CONTENT_LENGTH: usize = 1500;

pub async fn blog_post(
    State(pool): State<DbPool>,
    Extension(user_id): Extension<String>,
    Json(payload): Json<PostRequest>,
) -> Result<impl IntoResponse, ServerResponse<()>> {
    // Try to parse the user ID
    let author_id = Uuid::parse_str(&user_id)
        .map_err(|_| ServerResponse::ServerError("Invalid UUID".to_string()))?;

    // Extract data from the payload
    let post_title = payload.title;
    let post_body = payload.body;
    let slug = gen_slug(&post_title);

    match validate_blog(&post_title, &post_body) {
        Ok(_) => {
            let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (author_id, title, body, slug) VALUES  ($1, $2, $3, $4) RETURNING *",
    )
    .bind(author_id)
    .bind(post_title)
    .bind(post_body)
    .bind(slug)
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string());
            match post {
                Ok(post) => Ok(ServerResponse::SuccessMessage(
                    "Posted Successfully".to_string(),
                    Some(post),
                )),
                Err(err) => {
                    eprint!("Error message {}", err);
                    Err(ServerResponse::ServerError(
                        "Something went wrong.".to_string(),
                    ))
                }
            }
        }
        Err(e) => Err(ServerResponse::ServerError(format!(
            "Something went wrong: {}",
            e
        ))),
    }
}

pub fn gen_slug(title: &str) -> String {
    let title = title.to_lowercase();
    let title = title.replace(' ', "-");
    let re = Regex::new(r"[^a-z0-9-]").unwrap();
    let title = re.replace_all(&title, "");
    let title = title.trim_matches('-').to_string();
    let rng: ThreadRng = rand::rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let slug = format!("{}-{}", title, random_string);

    slug
}

fn validate_blog(title: &str, content: &str) -> Result<(), String> {
    if title.len() < MIN_TITLE_LENGTH || title.len() > MAX_TITLE_LENGTH {
        return Err(format!(
            "Title must be between {} and {} characters. Current length: {}",
            MIN_TITLE_LENGTH,
            MAX_TITLE_LENGTH,
            title.len()
        ));
    }

    if content.len() < MIN_CONTENT_LENGTH || content.len() > MAX_CONTENT_LENGTH {
        return Err(format!(
            "Content must be between {} and {} characters. Current length: {}",
            MIN_CONTENT_LENGTH,
            MAX_CONTENT_LENGTH,
            content.len()
        ));
    }

    Ok(())
}
