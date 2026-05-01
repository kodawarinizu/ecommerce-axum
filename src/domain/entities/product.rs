use uuid::Uuid;
use rust_decimal::Decimal;

pub enum Category {
    Electronics,
    Clothing,
    Food,
    Books,
    Sports,
    Home,
    Beauty,
    Toys,
}

pub struct Product {
    id: Uuid,
    name: String,
    desc: String,
    category: Category,
    stock: u32,
    value: Decimal,
    active: bool
}

impl Product {
    pub fn new(
        name: String,
        desc: String,
        category: Category,
        stock: u32,
        value: Decimal
    ) -> Self {
        Self { id: Uuid::new_v4(), name, desc, category, stock, value, active: true }
    }

    pub fn disable(&mut self) {
        self.active = false;
    }
}
