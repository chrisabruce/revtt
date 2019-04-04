use revtt::Connection;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let r = Connection::new(&env::var("USER").unwrap(), &env::var("PASS").unwrap());
    r.login();
}
