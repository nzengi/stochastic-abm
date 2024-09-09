use rand::Rng;

/// The Arithmetic Brownian Motion (ABM) model simulates the price movement
/// of an asset over time using the following formula:
///
/// dS = mu * dt + sigma * d_w
///
/// Where:
/// - `mu` is the drift (expected return)
/// - `sigma` is the volatility (standard deviation of returns)
/// - `d_w` is a Wiener process increment (Brownian motion)
pub struct ArithmeticBrownianMotion {
    pub mu: f64,
    pub sigma: f64,
    pub n_paths: usize,
    pub n_steps: usize,
    pub t_end: f64,
    pub s_0: f64,
}

impl ArithmeticBrownianMotion {
    /// Creates a new instance of the Arithmetic Brownian Motion model.
    ///
    /// # Arguments
    ///
    /// * `mu` - The drift (mean) of the asset's returns.
    /// * `sigma` - The volatility (standard deviation) of the asset's returns.
    /// * `n_paths` - Number of simulated paths.
    /// * `n_steps` - Number of steps in each path.
    /// * `t_end` - Total time of simulation.
    /// * `s_0` - Initial value of the asset (price at t=0).
    ///
    /// # Returns
    ///
    /// A new instance of `ArithmeticBrownianMotion`.
    pub fn new(mu: f64, sigma: f64, n_paths: usize, n_steps: usize, t_end: f64, s_0: f64) -> Self {
        Self {
            mu,
            sigma,
            n_paths,
            n_steps,
            t_end,
            s_0,
        }
    }

    /// Simulates the asset price paths using the Euler-Maruyama method.
    ///
    /// # Returns
    ///
    /// A 2D vector where each inner vector represents a simulated path of asset prices.
    ///
    /// Each path has `n_steps + 1` values, including the initial value `s_0`.
    pub fn simulate(&self) -> Vec<Vec<f64>> {
        let dt = self.t_end / self.n_steps as f64; // Time step size
        let mut rng = rand::thread_rng(); // Random number generator
        let mut paths = vec![vec![self.s_0; self.n_steps + 1]; self.n_paths]; // Initialize paths

        // Simulate each path
        for i in 0..self.n_paths {
            for j in 1..=self.n_steps {
                let d_w = rng.gen::<f64>() * dt.sqrt(); // Corrected to snake_case
                paths[i][j] = paths[i][j - 1] + self.mu * dt + self.sigma * d_w; // Euler-Maruyama update
            }
        }

        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abm_simulation() {
        let abm = ArithmeticBrownianMotion::new(0.05, 0.4, 50, 200, 1.0, 200.0);
        let paths = abm.simulate();
        assert_eq!(paths.len(), 50);
        assert_eq!(paths[0].len(), 201); // n_steps + 1
    }
}
