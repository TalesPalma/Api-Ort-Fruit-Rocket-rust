#[cfg(test)]
mod tests {
    use reqwest::{Client, StatusCode};

    #[tokio::test]
    async fn test_home_request() {
        let client = Client::new();
        let resp = client
            .get("http://127.0.0.1:8000/")
            .send()
            .await
            .expect("Erro in request home");

        assert_eq!(resp.status(), StatusCode::OK)
    }

    #[tokio::test]
    async fn test_not_authorized_request() {
        let client = Client::new();
        let resp = client
            .get("http://127.0.0.1:8000/unauthorized")
            .send()
            .await
            .expect("Erro in request not_authorized");

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_login_request() {
        let client = Client::new();
        let login_data = serde_json::json!({
            "email": "test@example.com",
            "password": "password123"
        });

        let resp = client
            .post("http://127.0.0.1:8000/login")
            .json(&login_data)
            .send()
            .await
            .expect("Erro in request login");

        assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_recurso_request() {
        let client = Client::new();
        let resp = client
            .get("http://127.0.0.1:8000/recurso")
            .send()
            .await
            .expect("Erro in request get recurso");

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_recurso_request() {
        let client = Client::new();
        let recurso_data = serde_json::json!({
            "name": "Recurso Test",
            "description": "Descrição do recurso de teste"
        });

        let resp = client
            .post("http://127.0.0.1:8000/recurso")
            .json(&recurso_data)
            .send()
            .await
            .expect("Erro in request create recurso");

        assert!(resp.status() == StatusCode::CREATED || resp.status() == StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_modify_recurso_request() {
        let client = Client::new();
        let recurso_data = serde_json::json!({
            "name": "Recurso Modificado",
            "description": "Descrição do recurso modificado"
        });

        let resp = client
            .put("http://127.0.0.1:8000/recurso/1")
            .json(&recurso_data)
            .send()
            .await
            .expect("Erro in request modify recurso");

        assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_recurso_request() {
        let client = Client::new();
        let resp = client
            .delete("http://127.0.0.1:8000/recurso/1")
            .send()
            .await
            .expect("Erro in request delete recurso");

        assert!(
            resp.status() == StatusCode::NO_CONTENT || resp.status() == StatusCode::BAD_REQUEST
        );
    }
}
