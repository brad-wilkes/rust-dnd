use rand::Rng;

#[derive(Clone, Copy)]
pub enum DieType {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl DieType {
    pub fn sides(&self) -> u32 {
        match self {
            DieType::D4 => 4,
            DieType::D6 => 6,
            DieType::D8 => 8,
            DieType::D10 => 10,
            DieType::D12 => 12,
            DieType::D20 => 20,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DieType::D4 => "d4",
            DieType::D6 => "d6",
            DieType::D8 => "d8",
            DieType::D10 => "d10",
            DieType::D12 => "d12",
            DieType::D20 => "d20",
        }
    }

    pub fn all() -> &'static [DieType] {
        use DieType::*;
        &[D4, D6, D8, D10, D12, D20]
    }
}

pub fn roll_die(die: DieType) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=die.sides())
}