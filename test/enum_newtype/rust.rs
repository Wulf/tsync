/// test/rust.rs
use tsync::tsync;

#[derive(Serialize, Deserialize)]
#[tsync]
#[serde(tag = "type")]
enum Message<Params, Value> {
    Request(Request<Params>),
    Response(Response<Value>),
}

#[derive(Serialize, Deserialize)]
#[tsync]
struct Request<Params> {
    id: String,
    method_type: String,
    params: Params,
}

#[derive(Serialize, Deserialize)]
#[tsync]
struct Response<Value> {
    id: String,
    result: Value,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
#[tsync]
pub enum CaptureConfigurationStruct {
    Video { pub height: u32, pub width: u32 },
    Redirect,
}

/// cases below were provided by joaoantoniocardoso on github in the discussion for issue #58
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
#[tsync]
pub enum CaptureConfigurationNewtype {
    Video(VideoCaptureConfiguration),
    Redirect(RedirectCaptureConfiguration),
}

#[derive(Deserialize, Serialize)]
#[tsync]
pub struct VideoCaptureConfiguration {
    pub height: u32,
    pub width: u32,
}

#[derive(Deserialize, Serialize)]
#[tsync]
pub struct RedirectCaptureConfiguration {}
