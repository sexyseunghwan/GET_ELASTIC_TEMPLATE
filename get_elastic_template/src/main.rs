mod common;
mod controller;
mod model;
mod repository;
mod service;
mod utils_modules;

use crate::utils_modules::logger_utils::*;
use crate::controller::main_controller::*;


#[tokio::main]
async fn main() {

    set_global_logger();

    main_controller().await;
}