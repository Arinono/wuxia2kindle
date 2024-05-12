use askama::Template;
use axum::extract::{Path, State};
use sqlx::PgPool;

use crate::server::{auth::AuthKind, Error};

#[derive(Template)]
#[template(path = "chapter.html")]
pub struct ChapterTemplate {
    pub name: String,
    pub content: String,
}

struct ChapterQuery {
    pub name: String,
    pub content: String,
}

pub async fn chapter(
    auth: AuthKind,
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<ChapterTemplate, Error> {
    if let Err(error) = auth.human() {
        return Err(error);
    }

    let chapter = sqlx::query_as!(
        ChapterQuery,
        "SELECT name, content FROM chapters WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await;

    match chapter {
        Err(e) => {
            eprintln!("Error getting chapter: {}", e);
            Err(Error::AppError(anyhow::Error::from(e)))
        }
        Ok(o_chapter) => match o_chapter {
            None => Err(Error::NotFound("chapter not found".to_owned())),
            Some(chapter) => Ok(ChapterTemplate {
                name: chapter.name,
                content: chapter.content,
            }),
        },
    }
}
