//! Integration tests for database models.

use basic_site::models::{session::Session, user::User};
use basic_site::password::generate_hash;
use basic_site::util::current_time_micros;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use uuid::Uuid;

/// Creates an in-memory SQLite database with migrations applied.
async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

fn create_test_user(username: &str, password: &str) -> User {
    User {
        id: Uuid::new_v4(),
        username: username.to_owned(),
        password_hash: generate_hash(password),
        email: None,
        created_at: current_time_micros(),
    }
}

// ============================================================================
// User model tests
// ============================================================================

#[tokio::test]
async fn user_insert_and_get_by_id() {
    let db = setup_test_db().await;
    let user = create_test_user("testuser", "password123");

    User::insert(&db, &user).await.expect("insert failed");

    let fetched = User::get_by_id(&db, user.id).await.expect("get failed");
    assert_eq!(fetched.username, "testuser");
    assert_eq!(fetched.id, user.id);
}

#[tokio::test]
async fn user_get_by_username() {
    let db = setup_test_db().await;
    let user = create_test_user("findme", "password123");

    User::insert(&db, &user).await.expect("insert failed");

    let fetched = User::get_by_username(&db, "findme")
        .await
        .expect("get failed");
    assert_eq!(fetched.id, user.id);
}

#[tokio::test]
async fn user_get_by_username_not_found() {
    let db = setup_test_db().await;

    let result = User::get_by_username(&db, "nonexistent").await;
    assert!(matches!(result, Err(sqlx::Error::RowNotFound)));
}

#[tokio::test]
async fn user_check_login_success() {
    let db = setup_test_db().await;
    let user = create_test_user("loginuser", "correctpassword");

    User::insert(&db, &user).await.expect("insert failed");

    let result = User::check_login(&db, "loginuser", "correctpassword").await;
    assert!(result.is_some());
    assert_eq!(result.unwrap().username, "loginuser");
}

#[tokio::test]
async fn user_check_login_wrong_password() {
    let db = setup_test_db().await;
    let user = create_test_user("loginuser2", "correctpassword");

    User::insert(&db, &user).await.expect("insert failed");

    let result = User::check_login(&db, "loginuser2", "wrongpassword").await;
    assert!(result.is_none());
}

#[tokio::test]
async fn user_check_login_nonexistent_user() {
    let db = setup_test_db().await;

    let result = User::check_login(&db, "ghost", "anypassword").await;
    assert!(result.is_none());
}

#[tokio::test]
async fn user_update_email() {
    let db = setup_test_db().await;
    let user = create_test_user("emailuser", "password123");

    User::insert(&db, &user).await.expect("insert failed");

    User::update_email(&db, user.id, Some("test@example.com"))
        .await
        .expect("update failed");

    let fetched = User::get_by_id(&db, user.id).await.expect("get failed");
    assert_eq!(fetched.email, Some("test@example.com".to_owned()));
}

#[tokio::test]
async fn user_clear_email() {
    let db = setup_test_db().await;
    let mut user = create_test_user("clearmail", "password123");
    user.email = Some("old@example.com".to_owned());

    User::insert(&db, &user).await.expect("insert failed");
    User::update_email(&db, user.id, None)
        .await
        .expect("update failed");

    let fetched = User::get_by_id(&db, user.id).await.expect("get failed");
    assert_eq!(fetched.email, None);
}

// ============================================================================
// Session model tests
// ============================================================================

fn create_test_session(user_id: Uuid) -> Session {
    let now = current_time_micros();
    Session {
        id: Uuid::new_v4(),
        user_id,
        ip_address: "127.0.0.1:12345".to_owned(),
        user_agent: "TestAgent/1.0".to_owned(),
        created_at: now,
        expires_at: now + 604_800_000_000, // 1 week in microseconds
    }
}

#[tokio::test]
async fn session_insert_and_get_by_id() {
    let db = setup_test_db().await;
    let user = create_test_user("sessionuser", "password123");
    User::insert(&db, &user).await.expect("user insert failed");

    let session = create_test_session(user.id);
    Session::insert(&db, &session)
        .await
        .expect("session insert failed");

    let fetched = Session::get_by_id(&db, session.id)
        .await
        .expect("get failed");
    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.user_id, user.id);
    assert_eq!(fetched.ip_address, "127.0.0.1:12345");
}

#[tokio::test]
async fn session_get_by_id_not_found() {
    let db = setup_test_db().await;

    let result = Session::get_by_id(&db, Uuid::new_v4())
        .await
        .expect("query failed");
    assert!(result.is_none());
}

#[tokio::test]
async fn session_delete_by_id() {
    let db = setup_test_db().await;
    let user = create_test_user("deleteuser", "password123");
    User::insert(&db, &user).await.expect("user insert failed");

    let session = create_test_session(user.id);
    Session::insert(&db, &session)
        .await
        .expect("session insert failed");

    let deleted = Session::delete_by_id(&db, session.id)
        .await
        .expect("delete failed");
    assert_eq!(deleted, 1);

    let fetched = Session::get_by_id(&db, session.id)
        .await
        .expect("get failed");
    assert!(fetched.is_none());
}

#[tokio::test]
async fn session_get_by_user_id() {
    let db = setup_test_db().await;
    let user = create_test_user("multisession", "password123");
    User::insert(&db, &user).await.expect("user insert failed");

    // Create multiple sessions
    for _ in 0..3 {
        let session = create_test_session(user.id);
        Session::insert(&db, &session)
            .await
            .expect("session insert failed");
    }

    let sessions = Session::get_by_user_id(&db, user.id)
        .await
        .expect("get failed");
    assert_eq!(sessions.len(), 3);
}

#[tokio::test]
async fn session_get_by_user_id_excludes_expired() {
    let db = setup_test_db().await;
    let user = create_test_user("expireuser", "password123");
    User::insert(&db, &user).await.expect("user insert failed");

    // Create an expired session
    let now = current_time_micros();
    let expired_session = Session {
        id: Uuid::new_v4(),
        user_id: user.id,
        ip_address: "127.0.0.1:12345".to_owned(),
        user_agent: "TestAgent/1.0".to_owned(),
        created_at: now - 1_000_000_000, // 1000 seconds ago
        expires_at: now - 1,             // expired
    };
    Session::insert(&db, &expired_session)
        .await
        .expect("session insert failed");

    // Create a valid session
    let valid_session = create_test_session(user.id);
    Session::insert(&db, &valid_session)
        .await
        .expect("session insert failed");

    let sessions = Session::get_by_user_id(&db, user.id)
        .await
        .expect("get failed");
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].id, valid_session.id);
}

#[tokio::test]
async fn session_cascade_delete_on_user_delete() {
    let db = setup_test_db().await;
    let user = create_test_user("cascadeuser", "password123");
    User::insert(&db, &user).await.expect("user insert failed");

    let session = create_test_session(user.id);
    Session::insert(&db, &session)
        .await
        .expect("session insert failed");

    // Delete the user directly
    sqlx::query!("DELETE FROM user WHERE id = ?", user.id)
        .execute(&db)
        .await
        .expect("user delete failed");

    // Session should be cascade deleted
    let fetched = Session::get_by_id(&db, session.id)
        .await
        .expect("get failed");
    assert!(fetched.is_none());
}
