use rocket::{Build, Rocket};
use crate::domain::customer::{CustomerService};
use crate::gateway::customer::{create_customer, get_customer};
use std::sync::Arc;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi_get_routes, rapidoc::*, swagger_ui::*};
use crate::gateway::customer::okapi_add_operation_for_create_customer_;
use crate::gateway::customer::okapi_add_operation_for_get_customer_;


pub fn server(
    customer_service: Arc<CustomerService>
) -> Rocket<Build> {
    rocket::build()
        .manage(customer_service)
        .mount("/api", openapi_get_routes![create_customer, get_customer])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../api/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../api/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}