//! tests/health_check.rs

//`tokio::test` is equivalent to `tokio::main`
//can inspect the code using
//`cargo expand --test health_check` (<- name of the test file)

#[tokio::test]
async fn health_check_works() {
    //Arrange
    spawn_app();
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

//Launch our application in the background ~somehow~
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
