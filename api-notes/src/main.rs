use std::net::SocketAddr;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Note {
    id: i64,
    titre: String,
    contenu: String,
}

#[derive(Debug, Deserialize)]
struct NouvelleNote {
    titre: String,
    contenu: String,
}

#[derive(Debug, thiserror::Error)]
enum ApiError {
    #[error("ressource introuvable")]
    Introuvable,
    #[error("requête invalide : {0}")]
    Invalide(String),
    #[error(transparent)]
    Base(#[from] sqlx::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (statut, message) = match &self {
            ApiError::Introuvable => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::Invalide(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            ApiError::Base(e) => {
                tracing::error!(erreur = %e, "erreur de base de données");
                // ne jamais exposer le détail interne au client
                (StatusCode::INTERNAL_SERVER_ERROR, String::from("erreur interne"))
            }
        };
        (statut, Json(serde_json::json!({ "erreur": message }))).into_response()
    }
}

async fn lister(State(etat): State<AppState>) -> Result<Json<Vec<Note>>, ApiError> {
    let notes = sqlx::query_as::<_, Note>("SELECT id, titre, contenu FROM notes ORDER BY id")
        .fetch_all(&etat.db)
        .await?;
    Ok(Json(notes))
}

async fn creer(
    State(etat): State<AppState>,
    Json(entree): Json<NouvelleNote>,
) -> Result<(StatusCode, Json<Note>), ApiError> {
    if entree.titre.trim().is_empty() {
        return Err(ApiError::Invalide(String::from("le titre est vide")));
    }
    let note = sqlx::query_as::<_, Note>(
        "INSERT INTO notes (titre, contenu) VALUES (?, ?) RETURNING id, titre, contenu",
    )
    .bind(&entree.titre)
    .bind(&entree.contenu)
    .fetch_one(&etat.db)
    .await?;
    Ok((StatusCode::CREATED, Json(note)))
}

async fn lire(
    State(etat): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Note>, ApiError> {
    sqlx::query_as::<_, Note>("SELECT id, titre, contenu FROM notes WHERE id = ?")
        .bind(id)
        .fetch_optional(&etat.db)
        .await?
        .map(Json)
        .ok_or(ApiError::Introuvable)
}

fn application(etat: AppState) -> Router {
    Router::new()
        .route("/notes", get(lister).post(creer))
        .route("/notes/{id}", get(lire))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(etat)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| String::from("sqlite:notes.db?mode=rwc"));
    let db = SqlitePool::connect(&url).await?;
    sqlx::migrate!().run(&db).await?; // exécute les fichiers du dossier migrations/

    let adresse = SocketAddr::from(([127, 0, 0, 1], 3000));
    let ecouteur = tokio::net::TcpListener::bind(adresse).await?;
    tracing::info!(%adresse, "serveur démarré");

    axum::serve(ecouteur, application(AppState { db }))
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt; // fournit oneshot

    async fn appli_de_test() -> Router {
        let db = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1) // chaque connexion aurait sinon sa propre base en mémoire
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!().run(&db).await.unwrap();
        application(AppState { db })
    }

    #[tokio::test]
    async fn creer_puis_lire_une_note() {
        let appli = appli_de_test().await;

        let requete = Request::builder()
            .method("POST")
            .uri("/notes")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"titre":"essai","contenu":"bonjour"}"#))
            .unwrap();
        let reponse = appli.clone().oneshot(requete).await.unwrap();
        assert_eq!(reponse.status(), StatusCode::CREATED);

        let requete = Request::builder().uri("/notes/1").body(Body::empty()).unwrap();
        let reponse = appli.oneshot(requete).await.unwrap();
        assert_eq!(reponse.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn note_absente_renvoie_404() {
        let appli = appli_de_test().await;
        let requete = Request::builder().uri("/notes/999").body(Body::empty()).unwrap();
        let reponse = appli.oneshot(requete).await.unwrap();
        assert_eq!(reponse.status(), StatusCode::NOT_FOUND);
    }
}
