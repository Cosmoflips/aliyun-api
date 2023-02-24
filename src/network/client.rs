use std::fmt::Display;

use super::request::OSSRequest;

pub struct Client<'a, T>
where
    T: Display + ?Sized,
{
    key_id: &'a T,
    key_secret: &'a T,
    region: &'a T,
}

impl<'a, 'b, T: Display + ?Sized> Client<'a, T> {
    pub fn new(key_id: &'a T, key_secret: &'a T, region: &'a T) -> Self {
        Self {
            key_id,
            key_secret,
            region,
        }
    }

    pub fn request_oss<P: Display + ?Sized>(
        &'a self,
        verb: &'b P,
        bucket: &'b P,
        api: &'b P,
    ) -> OSSRequest<'a, 'b, T, P> {
        OSSRequest {
            key_id: self.key_id,
            key_secret: self.key_secret,
            region: self.region,
            verb,
            bucket,
            api,
            query: Vec::new(),
            body: String::new(),
        }
    }
}
