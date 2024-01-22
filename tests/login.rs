mod helpers;
use helpers::do_login;

#[tokio::test]
async fn test_login() {
    let cookie = do_login().await;
    assert!(cookie.len() > 0);
}
