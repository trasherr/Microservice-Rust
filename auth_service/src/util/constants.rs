use std::env;

use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref PORT: String = set_port();
    pub static ref ADDRESS: String = set_address();
    pub static ref CONSUL: String = set_consul();
}

fn set_port() -> String{
    dotenv().ok();
    env::var("PORT").unwrap()
}

fn set_address() -> String{
    dotenv().ok();
    env::var("ADDRESS").unwrap()
}

fn set_consul() -> String{
    dotenv().ok();
    env::var("CONSUL").unwrap()
}
