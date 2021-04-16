#[cfg(test)]
mod json {
    use serde::{Deserialize, Serialize};

    #[test]
    fn json() {
        #[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug)]
        struct Address {
            street: String,
            city: String,
            state: String,
            zip: u128
        }

        let address = Address {
            street: "21 Stone Rd".to_owned(),
            city: "Boulder".to_owned(),
            state: "CO".to_owned(),
            zip: 80504.to_owned()
        };

        let json = serde_json::to_string( &address ).unwrap();
        let addressx: Address = serde_json::from_str( &json ).unwrap();
        assert_eq!( address, addressx );
    }
}