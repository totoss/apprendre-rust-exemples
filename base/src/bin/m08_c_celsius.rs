use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Celsius(i32);

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} °C", self.0)
    }
}

impl From<i32> for Celsius {
    fn from(degres: i32) -> Self {
        Celsius(degres)
    }
}

impl FromStr for Celsius {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let degres = s.trim().trim_end_matches("°C").trim().parse()?;
        Ok(Celsius(degres))
    }
}

fn main() {
    let a: Celsius = 21.into();                 // From donne Into gratuitement
    let b: Celsius = "18 °C".parse().unwrap();  // parse() utilise FromStr
    println!("{a} {b} {}", a > b);
    println!("{}", Celsius::default());
}
