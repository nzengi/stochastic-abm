# Arithmetic Brownian Motion (ABM) in Rust

This Rust crate provides an implementation of the **Arithmetic Brownian Motion (ABM)** model, a simple stochastic process used to simulate the price movement of assets over time.

## Overview

The Arithmetic Brownian Motion (ABM) is defined by the stochastic differential equation:

dS = μ * dt + σ * dW

```swift
Where:
- `S` is the asset price,
- `μ` is the drift (mean or trend),
- `σ` is the volatility (standard deviation of returns),
- `dW` is the Wiener process increment (Brownian motion),
- `dt` is the time increment.

ABM is commonly used in finance for modeling asset prices, though it has limitations such as not enforcing positivity of the asset price.
```


