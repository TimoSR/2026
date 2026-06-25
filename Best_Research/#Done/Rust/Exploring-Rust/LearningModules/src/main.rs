mod riftgate_network;
mod utils;

use riftgate_network::server;

//use riftgate_network::{client::data_layer::send_data, server::ports::listen};
//use utils::{helper_function, some_public_function};

fn main() {
    println!("Starting main application");

    utils::some_public_function();
    utils::helper_function();
 
    server::ports::listen();
    riftgate_network::connect();
    riftgate_network::client::data_layer::send_data();

    println!("Finished main application");
}
