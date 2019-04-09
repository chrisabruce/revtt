use revtt::Connection;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let mut r = Connection::new(&env::var("USER").unwrap(), &env::var("PASS").unwrap());
    let _ = r.login();
}
