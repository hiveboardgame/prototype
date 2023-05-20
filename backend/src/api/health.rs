use actix_web::{get, HttpResponse};

#[get("/health_check")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use actix_web::test::{self, TestRequest};

    #[actix_rt::test]
    async fn health_check() {
        let app = test::init_service(crate::new_test_app().await).await;
        let resp = TestRequest::get()
            .uri("/api/health_check")
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
    }
}
