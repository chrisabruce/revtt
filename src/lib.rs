use reqwest::{header, Client};

pub use reqwest::Error;

const BASE_URL: &str = "https://revolutiontt.me";

#[derive(Debug)]
pub struct Connection {
    client: Client,
    username: String,
    password: String,
}

impl Connection {
    pub fn new(username: &str, password: &str) -> Self {
        //Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_6; en-en) AppleWebKit/533.19.4 (KHTML, like Gecko) Version/5.0.3 Safari/533.19.4
        let mut headers = header::HeaderMap::new();
        headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10_6_6; en-en) AppleWebKit/533.19.4 (KHTML, like Gecko) Version/5.0.3 Safari/533.19.4"));
        Connection {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .default_headers(headers)
                .build()
                .unwrap(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    /// Returns `Error` if login fails, otherwise `Ok()`
    pub fn login(&mut self) -> Result<(), Error> {
        // Need to hit page to get session cookie
        let mut res = self.client.get(&format!("{}/login.php", BASE_URL)).send()?;
        println!("{:#?}", res.text());
        let params = [
            ("username", self.username.clone()),
            ("password", self.password.clone()),
            ("submit", "login".to_string()),
        ];

        let mut res = self
            .client
            .post(&format!("{}/takelogin.php", BASE_URL))
            .form(&params)
            .send()?;

        println!("{:#?}", res.headers());
        println!("{:#?}", res.text());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let r = Connection::new(&"test", &"pass");
        assert_eq!(r.username, "test");
        assert_eq!(r.password, "pass");
    }
}
