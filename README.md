# Introduction
This repository offers an ergonomic Rust wrapper designed to seamlessly fetch JSON responses from The Ergast API, a comprehensive source for Formula 1 data.

# Basic Usage
```[rust]
// fetch results endpoint
let path = api::Path{year:2023, round:Some(1)};
let params = api::URLParams::default();
ergast::Ergast::results(path, params).await.unwrap();
```