use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::domain::coffee_shop_exception::CoffeeShopException;

#[derive(Debug, Copy, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Coffee {
    pub coffee_type: CoffeeType,
}

#[derive(Debug, Copy, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum CoffeeType {
    Cappuccino,
    Americano,
    Espresso,
    Macchiato,
    Mocha,
    Latte,
}

impl std::fmt::Display for CoffeeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for CoffeeType {
    type Err = CoffeeShopException;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Cappucino" => Ok(CoffeeType::Cappuccino),
            "Americano" => Ok(CoffeeType::Americano),
            "Espress" => Ok(CoffeeType::Espresso),
            "Macchiato" => Ok(CoffeeType::Macchiato),
            "Mocha" => Ok(CoffeeType::Mocha),
            "Latte" => Ok(CoffeeType::Latte),
            _ => Err(CoffeeShopException::InvalidCoffeeType(String::from(s))),
        }
    }
}

impl Coffee {
    pub fn new(coffee_type: CoffeeType) -> Coffee {
        let coffee = Coffee { coffee_type };
        coffee.prepare();
        coffee
    }

    pub fn get_price(&self) -> i32 {
        match self.coffee_type {
            CoffeeType::Cappuccino => 5,
            CoffeeType::Macchiato => 6,
            CoffeeType::Mocha => 7,
            CoffeeType::Americano => 8,
            CoffeeType::Espresso => 9,
            CoffeeType::Latte => 10,
        }
    }

    fn prepare(&self) {
        println!("Preparing your coffeee...")
    }
}

#[cfg(test)]
mod tests {
    use super::Coffee;
    use super::CoffeeType;
    use crate::domain::coffee_shop_exception::CoffeeShopException;

    #[test]
    fn test_price() {
        let coffee = Coffee::new(CoffeeType::Latte);
        assert_eq!(coffee.get_price(), 10);
    }

    #[test]
    fn throws_error_false_parse_to_coffee_type() {
        let coffee_type: Result<CoffeeType, CoffeeShopException> = String::from("Test").parse();
        let error = CoffeeShopException::InvalidCoffeeType(String::from("Test"));

        assert!(coffee_type.is_err());
        assert_eq!(error, coffee_type.unwrap_err());
    }
}
