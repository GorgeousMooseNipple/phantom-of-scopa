#[derive(Debug, Clone, Copy)]
pub enum Suite {
    Clubs,
    Coins,
    Cups,
    Swords,
}

#[derive(Debug, Clone, Copy)]
pub enum CardValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Fante,
    Cavallo,
    Re,
}

impl CardValue {
    pub fn value(&self) -> u8 {
        use CardValue::*;
        match self {
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Fante => 8,
            Cavallo => 9,
            Re => 10,
        }
    }

    pub fn prime(&self) -> u8 {
        use CardValue::*;
        match self {
            Seven => 21,
            Six => 18,
            One => 16,
            Five => 15,
            Four => 14,
            Three => 13,
            Two => 12,
            _ => 10,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suite: Suite,
    pub value: CardValue,
}

impl Card {
    pub fn new(suite: Suite, value: CardValue) -> Self {
        Self { suite, value }
    }

    pub fn value(&self) -> u8 {
        self.value.value()
    }

    pub fn prime(&self) -> u8 {
        self.value.prime()
    }
}
