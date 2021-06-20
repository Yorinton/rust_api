use lambda::{handler_fn, Context};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
mod docx;
use docx::{Doc};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = lambda::run(handler_fn(handler)).await?;
    Ok(response)
}

async fn handler(event: Value, _: Context) -> Result<Response, Error> {
    let doc: Doc = serde_json::from_value(event).unwrap();
    match docx::create_docx(doc) {
        Ok(()) => println!("成功!"),
        Err(e) => println!("エラー発生：{}", e)
    };
    let response = Response::new(200, Value::String("aa".to_string()));
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
            "title": "タイトル",
            "header": "表題文",
            "sections": [
              {
                "title": "項目１のタイトル",
                "paragraphs": [
                  {
                    "body": "段落１\n段落１",
                    "sentences": []
                  }
                ],
                "line_no": 1
              },
              {
                "title": "項目２のタイトル",
                "paragraphs": [
                  {
                    "body": "",
                    "sentences": [
                      {
                        "body": "文章１\n文章１"
                      }
                    ]
                  }
                ]
              },
              {
                "title": "項目３のタイトル",
                "paragraphs": [
                  {
                    "body": "段落２段落２",
                    "sentences": [
                      {
                        "body": "文章１\n文章１"
                      },
                      {
                        "body": "文章２文章２"
                      },
                      {
                        "body": "文章３文章３"
                      }
                    ]
                  }
                ]
              },
              {
                "title": "項目４のタイトル",
                "paragraphs": [
                  {
                    "body": "段落３段落３",
                    "sentences": [
                      {
                        "body": "文章１\n文章１"
                      }
                    ]
                  },
                  {
                    "body": "段落４段落４",
                    "sentences": [
                      {
                        "body": "文章１\n文章１"
                      }
                    ]
                  },
                  {
                    "body": "段落５段落５",
                    "sentences": [
                      {
                        "body": "文章１文章１"
                      },
                      {
                        "body": "文章２文章２"
                      }
                    ]
                  }
                ]
              }
            ]
          });
        assert_eq!(
            handler(event.clone(), Context::default())
                .await
                .expect("expected Ok(_) value"),
            Response::new(200, Value::String("aa".to_string()))
        )
    }
}
