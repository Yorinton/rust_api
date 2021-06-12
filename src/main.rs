use lambda::{handler_fn, Context};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq; 

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = lambda::run(handler_fn(handler)).await?;
    Ok(response)
}

async fn handler(event: Value, _: Context) -> Result<Response, Error> {
    let response = Response::new(200, event);
    Ok(response)
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Response {
    // APIGatewayに返すLambdaのレスポンス形式が決まっているため、
    // statusCodeはキャメルケースにする必要がある
    statusCode: u32,
    body: String,
}

#[allow(non_snake_case)]
impl Response {
    fn new(status_code: u32, body: Value) -> Self {
        Response {
            statusCode: status_code,
            // bodyにはjsonではなくStringを指定する必要がある
            body: body.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn handler_handles() {
        let event = json!({
            "answer": 42
        });
        assert_eq!(
            handler(event.clone(), Context::default())
                .await
                .expect("expected Ok(_) value"),
            Response::new(200, event)
        )
    }
}
