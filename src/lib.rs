/// Represents a Fibonacci Retracement calculation for financial analysis.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use fibonacci_retracement::FibonacciRetracement;
/// let fib_retracement = FibonacciRetracement {
///     fibonacci_one: 100.0,
///     fibonacci_zero: 0.0,
///     fibonacci_retracement_level: 0.618,
/// };
///
/// assert_eq!(fib_retracement.calculate_retracement(), 61.8);
/// ```
pub struct FibonacciRetracement {
    pub fibonacci_one: f64,
    pub fibonacci_zero: f64,
    pub fibonacci_retracement_level: f64,
}

impl FibonacciRetracement {
    /// Calculates the Fibonacci retracement level.
    ///
    /// This method uses the Fibonacci levels to calculate the retracement level
    /// based on the provided `fibonacci_one`, `fibonacci_zero`, and `fibonacci_retracement_level`.
    ///
    /// # Returns
    ///
    /// The calculated retracement value as a `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fibonacci_retracement::FibonacciRetracement;
    /// let fib_retracement = FibonacciRetracement {
    ///     fibonacci_one: 200.0,
    ///     fibonacci_zero: 100.0,
    ///     fibonacci_retracement_level: 0.5,
    /// };
    ///
    /// assert_eq!(fib_retracement.calculate_retracement(), 150.0);
    /// ```
    pub fn calculate_retracement(&self) -> f64 {
        self.fibonacci_zero + (self.fibonacci_one - self.fibonacci_zero) * self.fibonacci_retracement_level
    }
}