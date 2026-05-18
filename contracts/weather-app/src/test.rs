#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_add_city() {
        let env = Env::default();
        let contract_id = env.register_contract(None, WeatherContract);

        let client = WeatherContractClient::new(&env, &contract_id);

        client.add_city(
            &String::from_str(&env, "Kuala Lumpur"),
            &String::from_str(&env, "Malaysia"),
        );

        let cities = client.get_cities();

        assert_eq!(cities.len(), 1);
    }
}
