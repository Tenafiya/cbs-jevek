use actix_http::Request;
use actix_web::{App, http::StatusCode, test, web};
use cbs_jevek::AppState;
use cbs_jevek::app::health::controllers::{engine_check, run_migrations};
use sea_orm::ConnectionTrait;
use serde_json::json;

mod common;
use common::{body_json, build_state, setup_test_database};

async fn test_app() -> impl actix_web::dev::Service<
    Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    let app =
        App::new().service(web::scope("").route("/v1/health/engine", web::get().to(engine_check)));
    test::init_service(app).await
}

async fn table_exists(state: &AppState, name: &str) -> bool {
    let sql = format!(
        "SELECT COUNT(*) FROM information_schema.tables \
         WHERE table_schema = 'public' AND table_name = '{name}'"
    );
    let row = state
        .pgdb
        .query_one(sea_orm::Statement::from_string(
            state.pgdb.get_database_backend(),
            sql,
        ))
        .await
        .unwrap()
        .unwrap();

    let count: i64 = row.try_get_by::<i64, _>("count").unwrap();
    count > 0
}

#[tokio::test]
async fn engine_health_returns_success() {
    let app = test_app().await;

    let req = test::TestRequest::get()
        .uri("/v1/health/engine")
        .to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), StatusCode::OK);
    let response_json = body_json(res).await;

    assert_eq!(response_json["code"], "OPERATION_SUCCESS");
    assert_eq!(response_json["message"], "Health Check Success");
    assert_eq!(response_json["success"], true);
    assert_eq!(response_json["data"], json!(null));
}

#[tokio::test]
async fn migrate_up_and_down_via_endpoint() {
    let Some(db_url) = setup_test_database().await else {
        eprintln!("Skipping test: PostgreSQL container could not be started");
        return;
    };

    let state = build_state(&db_url).await;

    let app = test::init_service(
        App::new().app_data(web::Data::new(state.clone())).service(
            web::resource("/v1/health/{direction}/migration/{steps}")
                .route(web::get().to(run_migrations)),
        ),
    )
    .await;

    let migrate = |direction: &str, steps: u32| {
        let uri = format!("/v1/health/{direction}/migration/{steps}");
        test::TestRequest::get().uri(&uri).to_request()
    };

    let req = migrate("UP", 2);
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let response_json = body_json(resp).await;
    assert_eq!(
        response_json["code"],
        cbs_jevek::utils::errors::ApiCode::OperationSuccess as i32
    );
    assert_eq!(response_json["message"], "Migration Successful");
    assert_eq!(response_json["success"], true);
    assert_eq!(response_json["data"], json!(null));
    assert!(table_exists(&state, "countries").await);

    let req = migrate("DOWN", 1);
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    assert!(!table_exists(&state, "countries").await);
}
