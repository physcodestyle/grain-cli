use std::fmt;

pub struct Grain<T> {
    name: String,
    origins: Vec<(T, T)>,
    params: Option<Vec<Vec<f64>>>,
    units: Vec<f64>,
}

impl<T> fmt::Display for Grain<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut collections = String::new();
        for o in self.origins.iter() {
            let formatted = format!("{}\t|\t{}", &o.0, &o.1);
            collections.push_str(&formatted);
        }
        write!(f, "{}\n{}", self.name, collections)
    }
}

impl<T> Grain<T> {
    pub fn new(
        name: String,
        origins: Vec<(T, T)>,
        params: Option<Vec<Vec<f64>>>,
        units: Vec<f64>) -> Self
    {
        Self {
            name: name,
            origins: origins,
            params: params,
            units: units,
        }
    }
}

pub trait Calc<T> {
    fn distribute(&self, coords: T) -> Vec<f64>;
    fn migrate(&self, delta: &f64);
}
