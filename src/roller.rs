use rand::thread_rng;
use rand::{self, Rng};

#[derive(Debug, Clone)]
pub struct RollResult {
    pub dices: u32,
    pub faces: u32,
    pub modifier: i32,
    pub results: Vec<i32>,
    pub total: i32,
}

impl Default for RollResult {
    fn default() -> Self {
        RollResult {
            dices: 1,
            faces: 6,
            modifier: 0,
            results: vec![],
            total: 0,
        }
    }
}

impl ToString for RollResult {
    fn to_string(&self) -> String {
        let mut retval: String = "".into();
        for (idx, result) in self.results.iter().enumerate() {
            if idx > 0 {
                retval = format!("{}, {}", &retval, result);
            } else {
                retval = format!("{}", result);
            }
        }
        if self.results.len() > 1 {
            retval = format!("{} (Total: {})", &retval, self.total);
        }
        retval
    }
}

impl RollResult {
    pub fn add_result(&mut self, result: i32) -> () {
        self.results.push(result);
        self.total += result;
    }
}

pub struct Roller;
impl Roller {
    pub fn roll_mod(dices: u32, faces: u32, modifier: i32) -> RollResult {
        let mut results: RollResult = RollResult {
            dices,
            faces,
            modifier,
            ..Default::default()
        };

        for _ in 0..dices {
            let mut rng = thread_rng();
            let roll: i32 = rng.gen_range(1..=faces) as i32;
            let with_mod: i32 = roll + modifier;
            results.add_result(with_mod);
        }

        results
    }
}
