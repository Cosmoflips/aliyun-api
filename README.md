# Aliyun API  
非官方的Rust API SDK

## 现已支持的
- [ ] OSS
- [ ] 物联网

## 示例
```rust
use aliyun_api::{client::Client, request::Request};

#[tokio::main]
async fn main() {
    let ID = "*******************";
    let SECRET = "************************";
    let REGION = "oss-cn-hangzhou.aliyuncs.com";
    let client = Client::new(ID, SECRET,REGION);
    let mut req = client
        .request_oss("GET", "/BucketName","OBjectName")
        .querys(&mut vec![
            ("list-type".to_string(), "2".to_string()),
        ])
        .body(String::new());
    if let Ok(text) = req.send().await {
        println!("{text}");
    }
}
```