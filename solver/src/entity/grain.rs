use std::fmt;

pub struct Grain<T> {
    name: String,
    origins: Vec<(T, T)>,
    params: Vec<Vec<f64>>,
    units: Vec<f64>,
}

impl<T> fmt::Display for Grain<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut collections = String::new();
        for o in self.origins.iter() {
            let formatted = format!("\n{}\t|\t{}", &o.0, &o.1);
            collections.push_str(&formatted);
        }
        write!(f, "{}{}", self.name, collections)
    }
}

impl<T> Grain<T> {
    pub fn new(
        name: String,
        origins: Vec<(T, T)>,
        params: Vec<Vec<f64>>,
        units: Vec<f64>) -> Self
    {
        Self {
            name: name,
            origins: origins,
            params: params,
            units: units,
        }
    }

    pub fn get_units(&self) -> &Vec<f64> {
        &self.units
    }

    pub fn change_origin(&mut self, index: usize, coords: T, velocity: T) {
        if index < self.origins.len() {
            self.origins[index].0 = coords;
            self.origins[index].1 = velocity;
        } else {
            println!("[Grains]: Index {} in origins is not available", index);
        }
    }

    pub fn change_params(&mut self, index: usize, params: Vec<f64>) {
        if index < self.params.len() {
            self.params[index] = params;
        } else {
            println!("[Grains]: Index {} in params is not available", index);
        }
    }
}

pub trait Calc<T> {
    fn distribute(&self, coords: T) -> Vec<f64>;
    fn migrate(&mut self, delta: &f64);
}
