//! A simple black-scholes example to be proven inside the zkVM.
#![no_main]
use std::f64::consts::{PI, SQRT_2};
use special::Error;

sp1_zkvm::entrypoint!(main);

// Standard normal distribution PDF. used for greeks calculations (to be implemented)
fn pdf_std_norm(x: f64) -> f64 {
    (-x.powi(2) / 2.0).exp() / (PI.sqrt() * SQRT_2)
}

fn cdf_std_norm(x: f64) -> f64 {
    (x / SQRT_2).error() * 0.5 + 0.5
}

// Black-Scholes call option price.
// using s is spot price, t is time to maturity, r is risk-free interest rate, v is volatility (sigma)
// and spot * strike_factor is the strike price (for ITM calls strike factor less than 1, for OTM calls strike factor greater than 1)
fn call_option_price(s: f64, strike_factor: f64, r: f64, t: f64, v: f64) -> f64 {
    let d1 = (-strike_factor.ln() + (r + v.powi(2) / 2.0) * t) / (v * t.sqrt());
    let d2 = d1 - v * t.sqrt();
    s *(cdf_std_norm(d1) - strike_factor * (-r * t).exp() * cdf_std_norm(d2))
}
pub fn main() {
    let s: f64 = sp1_zkvm::io::read::<f64>();
    let r: f64 = sp1_zkvm::io::read::<f64>();
    let v: f64 = sp1_zkvm::io::read::<f64>();
    let strike_factors: [f64; 3] = sp1_zkvm::io::read::<[f64;3]>();
    let tenors: [f64; 4] = sp1_zkvm::io::read::<[f64; 4]>();
    let mut call_prices: [[f64; 3]; 4] = [[0.0; 3]; 4];
    for i in 0..4 {
        for j in 0..3 {
            let t: f64 = tenors[i];
            let s: f64 = 100.0;
            let factor: f64 = strike_factors[j] as f64;
            call_prices[i][j] = call_option_price(s, factor, r, t, v);
        }
    }
    
    sp1_zkvm::io::write(&call_prices);
}
