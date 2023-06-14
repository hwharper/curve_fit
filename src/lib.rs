use range_checker::CheckVerbose;

#[derive(Debug, Clone, Copy, range_checker::CheckVerbose)]
pub struct Config {
    #[filter(|p0: &f64| p0.is_normal())]
    pub p0: f64,
    pub check_finite: bool,
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
        }
    }
}

pub struct Curve<const N: usize> {}

impl<const N: usize> Curve<N> {
    pub fn eval(&self) -> f64 {
        todo!()
    }
}

pub fn fit<const N: usize>(
    func: impl Fn(f64, [f64; N]) -> f64,
    x_data: &[f64],
    y_data: &[f64],
    cfg: Config,
) -> Result<Curve<N>, Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn target_func(x: f64, p: [f64; 4]) -> f64 {
        p[0] * x + p[1]
    }

    #[test]
    fn it_works() {
        let f = fit(
            |x: f64, p: [f64; 2]| p[0] * x + p[1],
            // target_func,
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
}
