use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::NamedTempFile;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn binary() -> Command {
    Command::cargo_bin("splitwise").expect("binary should build")
}

#[tokio::test]
async fn groups_list_reads_successfully() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/get_groups"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "groups": [
                {"id": 1, "name": "Trip"}
            ]
        })))
        .mount(&server)
        .await;

    binary()
        .env_clear()
        .args([
            "--json",
            "--base-url",
            &server.uri(),
            "--token",
            "test-token",
            "groups",
            "list",
        ])
        .assert()
        .success()
        .stdout("{\"groups\":[{\"id\":1,\"name\":\"Trip\"}]}\n");
}

#[tokio::test]
async fn comments_create_supports_typed_flags() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/create_comment"))
        .and(header("authorization", "Bearer test-token"))
        .and(body_json(serde_json::json!({
            "expense_id": 42,
            "content": "Hello there"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "comment": {
                "id": 9,
                "content": "Hello there"
            }
        })))
        .mount(&server)
        .await;

    binary()
        .env_clear()
        .args([
            "--json",
            "--base-url",
            &server.uri(),
            "--token",
            "test-token",
            "comments",
            "create",
            "--expense-id",
            "42",
            "--content",
            "Hello there",
        ])
        .assert()
        .success()
        .stdout("{\"comment\":{\"content\":\"Hello there\",\"id\":9}}\n");
}

#[tokio::test]
async fn groups_create_supports_body_file() {
    let server = MockServer::start().await;
    let body_file = NamedTempFile::new().expect("temp file should exist");
    std::fs::write(
        body_file.path(),
        r#"{"name":"Road Trip","group_type":"trip","users__0__email":"ada@example.com"}"#,
    )
    .expect("body file should be written");

    Mock::given(method("POST"))
        .and(path("/create_group"))
        .and(header("authorization", "Bearer test-token"))
        .and(body_json(serde_json::json!({
            "name": "Road Trip",
            "group_type": "trip",
            "users__0__email": "ada@example.com"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "group": {
                "id": 88,
                "name": "Road Trip"
            }
        })))
        .mount(&server)
        .await;

    binary()
        .env_clear()
        .args([
            "--json",
            "--base-url",
            &server.uri(),
            "--token",
            "test-token",
            "groups",
            "create",
            "--body",
            &format!("@{}", body_file.path().display()),
        ])
        .assert()
        .success()
        .stdout("{\"group\":{\"id\":88,\"name\":\"Road Trip\"}}\n");
}

#[tokio::test]
async fn semantic_failures_exit_with_code_two() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/delete_friend/99"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": false,
            "errors": {
                "base": ["There was an issue deleting that friendship"]
            }
        })))
        .mount(&server)
        .await;

    binary()
        .env_clear()
        .args([
            "--base-url",
            &server.uri(),
            "--token",
            "test-token",
            "friends",
            "delete",
            "99",
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("semantic failure"));
}
