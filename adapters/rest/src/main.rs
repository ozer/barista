use anyhow::Result;
use serde::{Deserialize, Serialize};
use tide::{Error, Request, Response, Server, StatusCode};

use crate::order_coffee_request::OrderCoffeeRequest;
use barista_application::application::port::incoming::order_coffee_use_case::{
    OrderCoffeeCommand, OrderCoffeeUseCase,
};
use barista_application::application::service::order_coffee_service::OrderCoffeeService;
use barista_application::domain::coffee::CoffeeType;
use barista_application::domain::coffee_shop_exception::CoffeeShopException;
use barista_application::domain::order::Order;
use dotenv::dotenv;
use persistence::customer_persistence_adapter::CustomerPersistenceAdapter;
use persistence::order_persistence_adapter::OrderPersistenceAdapter;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tide::http::{Body, Mime};
use tide::utils::After;

mod order_coffee_request;

#[derive(Serialize, Deserialize)]
pub struct OrderCoffeeResponse {
    order: Order,
}

#[derive(Serialize, Deserialize)]
pub struct GetOrderResponse {
    order: Option<Order>,
}

macro_rules! barista_response {
    ($t: ty) => {
        impl From<$t> for tide::Response {
            fn from(o: $t) -> Self {
                let order_json = serde_json::to_string(&o).unwrap();
                let mut res = tide::Response::new(StatusCode::Ok);
                let mime: Mime = Mime::from("application/json");
                res.set_body(Body::from(order_json));
                res.set_content_type(mime);
                res
            }
        }
    };
}

barista_response!(GetOrderResponse);
barista_response!(OrderCoffeeResponse);

#[derive(Clone)]
pub struct AppState {
    order_coffee_use_case: Arc<dyn OrderCoffeeUseCase + Send + Sync>,
}

impl AppState {
    pub fn new(order_coffee_use_case: Arc<dyn OrderCoffeeUseCase + Send + Sync>) -> Self {
        Self {
            order_coffee_use_case,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OCRequest {
    #[serde(rename = "customerName")]
    pub customer_name: Option<String>,
    #[serde(rename = "coffeeType")]
    pub coffee_type: Option<String>,
}

async fn handle_order_coffee(mut req: Request<AppState>) -> tide::Result<OrderCoffeeResponse> {
    let request: OrderCoffeeRequest = req
        .body_json()
        .await
        .map_err(|err| Error::from_str(StatusCode::BadRequest, String::from(err.to_string())))?;

    let coffee_type: CoffeeType =
        request
            .coffee_type
            .parse()
            .map_err(|err: CoffeeShopException| {
                Error::from_str(StatusCode::BadRequest, String::from(err.to_string()))
            })?;

    let command = OrderCoffeeCommand::new(request.customer_name, coffee_type);

    let order = req
        .state()
        .order_coffee_use_case
        .order_coffee(&command)
        .await
        .map_err(|err| Error::from_str(StatusCode::BadRequest, err.to_string()))?;

    Ok(OrderCoffeeResponse { order })
}

async fn get_order_by_id(req: Request<AppState>) -> tide::Result<GetOrderResponse> {
    let order_id = req
        .param("orderId")
        .map(|val| -> Result<i32, tide::Error> {
            match val.parse::<i32>() {
                Ok(oki) => Ok(oki),
                Err(_) => Err(Error::from_str(
                    StatusCode::BadRequest,
                    String::from("orderId can be only an integer."),
                )),
            }
        })?
        .map_err(|err| Error::from_str(StatusCode::BadRequest, String::from(err.to_string())))
        .map(|value| value)?;

    let order = req
        .state()
        .order_coffee_use_case
        .get_order_by_id(order_id)
        .await?;

    Ok(GetOrderResponse { order })
}

#[async_std::main]
async fn main() -> Result<()> {
    // Dot Env
    dotenv().ok();

    // DB Pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://ozer:123456@localhost:5409/coffeeshop")
        .await?;

    // Adapters
    let order_persistence_adapter = OrderPersistenceAdapter::new(pool.clone());
    let customer_persistence_adapter = CustomerPersistenceAdapter::new(pool.clone());

    // Use Case
    let order_coffee_use_case = Arc::new(OrderCoffeeService::new(
        Box::new(order_persistence_adapter.clone()),
        Box::new(customer_persistence_adapter),
        Box::new(order_persistence_adapter),
    ));

    // App State
    let app_state = AppState::new(order_coffee_use_case);

    // Tide Init
    tide::log::start();

    let mut app = Server::with_state(app_state);

    app.with(After(|mut res: Response| async {
        if let Some(err) = res.downcast_error::<CoffeeShopException>() {
            let msg = format!("ERROR {:?}", err);
            res.set_status(StatusCode::BadRequest);
            res.set_content_type(Mime::from("application/json"));
            res.set_body(msg);
        }

        Ok(res)
    }));

    // Route
    app.at("/order").post(handle_order_coffee);

    app.at("/order/:orderId").get(get_order_by_id);

    // App Listen
    app.listen("127.0.0.1:8080").await?;

    // Lets go
    Ok(())
}
