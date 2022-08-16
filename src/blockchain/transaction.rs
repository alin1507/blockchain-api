use std::fmt;


#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

impl Person {
    pub fn new(first_name: String, last_name: String) -> Self {
        Person {
            first_name,
            last_name,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.first_name, self.last_name)
    }
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from_adress: String,
    pub to_adress: String,
    pub amount_transfered: u32,
}

impl Transaction {
    pub fn new(from_adress: String, to_adress: String, amount_transfered: u32, ) -> Self {
        Transaction {
            amount_transfered,
            from_adress,
            to_adress,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.amount_transfered, self.from_adress, self.to_adress
        )
    }
}
