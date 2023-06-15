use ndarray::{Array, ArrayBase, Dim, OwnedRepr};
use range_checker::CheckVerbose;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Normal;
use rand;

#[derive(Debug, Clone, Copy, range_checker::CheckVerbose)]
pub struct Config {
    #[filter(|p0: &f64| p0.is_normal())]
    pub p0: f64,
    pub check_finite: bool,
    pub method: method
}

pub enum method {
    lm,      // Levenberg-Marquardt algorithm
    dogbox,  // dogleg algorithm
    trf      // Trust Region Reflective algorithm
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unmatched data length. x_data: {x_data_len} != y_data: {y_data_len}")]
    UnmatchedLength {
        x_data_len: usize,
        y_data_len: usize,
    },
    #[error("config {0}")]
    ConfigCheckFailed(range_checker::Error),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            p0: 1.0,
            check_finite: true,
            method: method::lm
        }
    }
}

pub struct Curve<const N: usize> {}

impl<const N: usize> Curve<N> {
    pub fn eval(&self) -> f64 {
        todo!()
    }
}

pub trait CurveFit<const N: usize> {
    fn fit(&self, x_data: &[f64], y_data: &[f64], cfg: Config) -> Result<Curve<N>, Error>;
}

impl<T, const N: usize> CurveFit<N> for T
where
    T: Fn(f64, [f64; N]) -> f64,
{
    fn fit(&self, x_data: &[f64], y_data: &[f64], cfg: Config) -> Result<Curve<N>, Error> {
        // data length check
        if x_data.len() != y_data.len() {
            return Err(Error::UnmatchedLength {
                x_data_len: x_data.len(),
                y_data_len: y_data.len(),
            });
        }

        // config check
        if let Err(e) = cfg.check() {
            if let Some(e) = e.into_iter().next() {
                return Err(Error::ConfigCheckFailed(e));
            }
        }

        Ok(Curve {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target_func(x: f64, p: [f64; 2]) -> f64 {
        p[0] * x + p[1]
    }

    #[test]
    fn it_works() {
        let lamda_func = |x: f64, p: [f64; 2]| p[0] * x + p[1];
        lamda_func
            .fit(
                &[1.0, 2.0, 3.0],
                &[1.0, 2.0, 3.0],
                Config {
                    check_finite: false,
                    ..Default::default()
                },
            )
            .unwrap();

        let f = target_func
            .fit(
                &[1.0, 2.0, 3.0],
                &[1.0, 2.0, 3.0],
                Config {
                    check_finite: false,
                    ..Default::default()
                },
            )
            .unwrap();

        f.eval();
    }

    #[test]
    fn simple_test() {
        let xdata = Array::linspace(0., 4., 50);
        let y = xdata.map(|&x| target_func(x, [2.5, 1.3]));
        let normal = Normal::new(0.0, 1.0).unwrap();
        let y_noise = Array::random_using(xdata.shape(), normal, &mut rand::thread_rng());
        let y_data = y + y_noise; 
        println!("y data:\n{:?}", y_data);
    

        // dbg!(xdata, ydata);
    }
}
