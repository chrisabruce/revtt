use cookie::{Cookie, CookieJar};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, RedirectPolicy, Response};

pub use reqwest::Error;

const BASE_URL: &str = "https://revolutiontt.me";
const HEADER_SET_COOKIE: &str = "Set-Cookie";
const HEADER_COOKIE: &str = "Cookie";

#[derive(Debug)]
pub struct Connection {
    client: Client,
    jar: CookieJar,
    username: String,
    password: String,
}

impl Connection {
    pub fn new(username: &str, password: &str) -> Self {
        Connection {
            client: reqwest::Client::builder()
                //.redirect(RedirectPolicy::none())
                .build()
                .unwrap(),
            jar: CookieJar::new(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    /// Returns `Error` if login fails, otherwise `Ok()`
    pub fn login(&mut self) -> Result<(), Error> {
        // Need to hit page to get session cookie
        let res = self.client.get(&format!("{}/login.php", BASE_URL)).send()?;
        self.store_cookies(&res);

        let params = [
            ("username", self.username.clone()),
            ("password", self.password.clone()),
            ("submit", "login".to_string()),
        ];

        println!("***\ncookies: {:#?}", self.cookie_headers());

        let mut res = self
            .client
            .post(&format!("{}/takelogin.php", BASE_URL))
            .form(&params)
            .headers(self.cookie_headers())
            .send()?;
        println!("{:#?}", res.headers());
        println!("{:#?}", res.text());
        Ok(())
    }

    fn store_cookies(&mut self, res: &Response) {
        res.headers()
            .get_all(HEADER_SET_COOKIE)
            .into_iter()
            .map(|header_value| header_value.to_str())
            .filter_map(Result::ok)
            .map(|s| s.to_string())
            .for_each(|cookie_string| {
                if let Ok(cookie) = Cookie::parse(cookie_string) {
                    self.jar.add_original(cookie);
                }
            });
    }

    fn cookie_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for cookie in self.jar.iter() {
            headers.append(
                HEADER_COOKIE,
                HeaderValue::from_str(&format!("{}={}", cookie.name(), cookie.value())).unwrap(),
            );
        }
        headers
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
