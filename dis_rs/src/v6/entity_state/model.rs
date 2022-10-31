#[derive(Debug, PartialEq, Default)]
pub struct EntityCapabilities {
    pub ammunition_supply : bool,
    pub fuel_supply : bool,
    pub recovery : bool,
    pub repair : bool,
}

impl EntityCapabilities {
    pub fn ammunition_supply(mut self) -> Self {
        self.ammunition_supply = true;
        self
    }

    pub fn fuel_supply(mut self) -> Self {
        self.fuel_supply = true;
        self
    }

    pub fn recovery(mut self) -> Self {
        self.recovery = true;
        self
    }

    pub fn repair(mut self) -> Self {
        self.repair = true;
        self
    }
}
