#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct WeatherCity {
    id: u64,
    city: String,
    country: String,
}

const CITY_DATA: Symbol = symbol_short!("CITY_DATA");

#[contract]
pub struct WeatherContract;

#[contractimpl]
impl WeatherContract {

    pub fn get_cities(env: Env) -> Vec<WeatherCity> {
        env.storage()
            .instance()
            .get(&CITY_DATA)
            .unwrap_or(Vec::new(&env))
    }

    pub fn add_city(
        env: Env,
        city: String,
        country: String
    ) -> String {

        let mut cities: Vec<WeatherCity> =
            env.storage()
            .instance()
            .get(&CITY_DATA)
            .unwrap_or(Vec::new(&env));

        let new_city = WeatherCity {
            id: env.prng().gen::<u64>(),
            city,
            country,
        };

        cities.push_back(new_city);

        env.storage()
            .instance()
            .set(&CITY_DATA, &cities);

        String::from_str(
            &env,
            "City added"
        )
    }

    pub fn delete_city(
        env: Env,
        id: u64
    ) -> String {

        let mut cities: Vec<WeatherCity> =
            env.storage()
            .instance()
            .get(&CITY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..cities.len() {
            if cities.get(i).unwrap().id == id {
                cities.remove(i);

                env.storage()
                    .instance()
                    .set(&CITY_DATA, &cities);

                return String::from_str(
                    &env,
                    "Deleted"
                );
            }
        }

        String::from_str(
            &env,
            "City not found"
        )
    }
}

mod test;