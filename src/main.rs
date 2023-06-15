use levenberg_marquardt::{LeastSquaresProblem, LevenbergMarquardt};
use nalgebra::{VectorN, Vector2, Matrix2, Owned, U2};

#[derive(Debug)]
struct ExampleProblem {
    // holds current value of the n parameters
    p: Vector2<f64>,
}

// We implement a trait for every problem we want to solve
impl LeastSquaresProblem<f64, U2, U2> for ExampleProblem {
    type ParameterStorage = Owned<f64, U2>;
    type ResidualStorage = Owned<f64, U2>;
    type JacobianStorage = Owned<f64, U2, U2>;
     
    fn set_params(&mut self, p: &VectorN<f64, U2>) {
        self.p.copy_from(p);
        // do common calculations for residuals and the Jacobian here
    }
     
    fn params(&self) -> VectorN<f64, U2> { self.p }
     
    fn residuals(&self) -> Option<Vector2<f64>> {
        let [x, y] = [self.p.x, self.p.y];
        // vector containing residuals

        Some(Vector2::new(x*x + y - 11., x + y*y - 7.))
    }
     
    fn jacobian(&self) -> Option<Matrix2<f64>> {
        let [x, y] = [self.p.x, self.p.y];

                 
        // first row of Jacobian, derivatives of first residual
        let d1_x = 2. * x; // 
        let d1_y = 1.;     // 

        // second row of Jacobian, derivatives of second residual
        let d2_x = 1.;     // 
        let d2_y = 2. * y; // 

        Some(Matrix2::new(
            d1_x, d1_y,
            d2_x, d2_y,
        ))
    }
}

fn main() {
    let problem = ExampleProblem {
        p: Vector2::new(1., 1.),
    };
    
    let (result, report) = LevenbergMarquardt::new().minimize(problem);
    assert!(report.termination.was_successful());
    assert!(report.objective_function.abs() < 1e-10);

    println!("{:?}", result);

}


