const MERCURY_ORBITAL_PERIOD: f64 = 0.2408467;
const VENUS_ORBITAL_PERIOD: f64 = 0.61519726;
const EARTH_ORBITAL_PERIOD_SECONDS: f64 = 31557600.0;
const MARS_ORBITAL_PERIOD: f64 = 1.8808158;
const JUPITER_ORBITAL_PERIOD: f64 = 11.862615;
const SATURN_ORBITAL_PERIOD: f64 = 29.447498;
const URANUS_ORBITAL_PERIOD: f64 = 84.016846;
const NEPTUNE_ORBITAL_PERIOD: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / (MERCURY_ORBITAL_PERIOD * EARTH_ORBITAL_PERIOD_SECONDS)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / (VENUS_ORBITAL_PERIOD * EARTH_ORBITAL_PERIOD_SECONDS)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / EARTH_ORBITAL_PERIOD_SECONDS
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / (MARS_ORBITAL_PERIOD * EARTH_ORBITAL_PERIOD_SECONDS)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / (JUPITER_ORBITAL_PERIOD * EARTH_ORBITAL_PERIOD_SECONDS)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / (SATURN_ORBITAL_PERIOD * EARTH_ORBITAL_PERIOD_SECONDS)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / (URANUS_ORBITAL_PERIOD * EARTH_ORBITAL_PERIOD_SECONDS)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / (NEPTUNE_ORBITAL_PERIOD * EARTH_ORBITAL_PERIOD_SECONDS)
    }
}
