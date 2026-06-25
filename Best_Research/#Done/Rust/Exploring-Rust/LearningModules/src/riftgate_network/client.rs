use crate::riftgate_network::connect;

pub mod data_layer {
    pub fn send_data() {
        println!("Sending data... (network::client module)");
        // Can call items from the parent module if needed
        // super::private_network_util(); // Would need 'pub(super)' or similar visibility
        super::connect(); // Can call public items from parent
    }
}
