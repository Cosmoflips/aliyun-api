use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::offset::Utc;
use hmac::{Hmac, Mac};
use reqwest::{ClientBuilder, Method};
use sha1::Sha1;
use std::fmt::Display;

type Result<T> = reqwest::Result<T>;
trait Auth {
    fn sign(&self, time: &str) -> String;
}

pub trait Request {
    fn query(self, k: String, v: String) -> Self;
    fn querys(self, querys: &mut Vec<(String, String)>) -> Self;
    fn body(self, text: String) -> Self;
}
pub struct OSSRequest<'a, 'b, T, P>
where
    T: Display + ?Sized,
    P: Display + ?Sized,
{
    pub key_id: &'a T,
    pub key_secret: &'a T,
    pub region: &'a T,
    pub verb: &'b P,
    pub bucket: &'b P,
    pub api: &'b P,
    pub query: Vec<(String, String)>,
    pub body: String,
}

impl<'a, 'b, T: Display + ?Sized, P: Display + ?Sized> Auth for OSSRequest<'a, 'b, T, P> {
    fn sign(&self, time: &str) -> String {
        let mut mac = Hmac::<Sha1>::new_from_slice(self.key_secret.to_string().as_bytes())
            .expect("init key error");
        mac.update(format!("{}\n\n\n{}\n{}/{}", self.verb, time, self.bucket, self.api).as_bytes());
        STANDARD.encode(mac.finalize().into_bytes())
    }
}

impl<'a, 'b, T: Display + ?Sized, P: Display + ?Sized> Request for OSSRequest<'a, 'b, T, P> {
    fn query(mut self, k: String, v: String) -> Self {
        self.query.push((k, v));
        Self { ..self }
    }
    fn querys(mut self, querys: &mut Vec<(String, String)>) -> Self {
        self.query.append(querys);
        Self { ..self }
    }
    fn body(mut self, text: String) -> Self {
        self.body = text;
        Self { ..self }
    }
}

impl<'a, 'b, T: Display + ?Sized, P: Display + ?Sized> OSSRequest<'a, 'b, T, P> {
    pub async fn send(&mut self) -> Result<String> {
        let time = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let auth = format!("OSS {}:{}", self.key_id, self.sign(&time));
        self.query.sort_unstable();
        if let Ok(method) = Method::from_bytes(self.verb.to_string().as_bytes()) {
            let resp = ClientBuilder::new()
                .build()?
                .request(
                    method,
                    format!("https://{}.{}/{}", self.bucket, self.region, self.api)
                        .replace("+", "%2B"),
                )
                .header("Authorization", auth)
                .header("Date", time)
                .body(self.body.clone())
                .query(&self.query[..])
                .send()
                .await?;
            return Ok(resp.text().await?);
        }
        Ok("invalid method".to_string())
    }
}
