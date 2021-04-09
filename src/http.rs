use std::collections::HashMap;
use url::Url;

struct Request;

impl Request {

    pub fn url(openapi: &str, param: HashMap<String, String>) -> String {
        return Url::parse_with_params(format!("{}{}", "http://api.data.go.kr/openapi/", openapi).as_str(), param).unwrap().to_owned().to_string();
    }

    pub async fn async_get(openapi: &str, param: HashMap<String, String>) -> String {
        let url = Request::url(openapi, param);
        let req = reqwest::get(url).await.unwrap();

        return req.text().await.unwrap();
    }

    pub fn sync_get(openapi: &str, param: HashMap<String, String>) -> String {
        let url = Request::url(openapi, param);

        let req = reqwest::blocking::get(url).unwrap();

        return req.text().unwrap();
    }
}
