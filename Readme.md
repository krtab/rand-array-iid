# Rand-array-iid

[![Build Status](https://img.shields.io/travis/krtab/rand-array-iid/main?style=flat-square)](https://travis-ci.org/krtab/rand-array-iid)
[![Crates.io](https://img.shields.io/crates/v/rand-array-iid?style=flat-square)](https://crates.io/crates/rand-array-iid)
[![Docs](https://img.shields.io/badge/docs-doc.rs-blue?style=flat-square)](https://docs.rs/rand-array-iid/0.1.0/rand_array_iid/)

A rust crate to create arrays whose elements are independently identically distributed.

## Install

```
[dependencies]
rand-array-iid = "0.1.0"
```

## Examples

### An array of normally distributed scalars
```rust
use rand_array_iid::IIDDistr;
use rand_distr::Distribution;
use rand_distr::StandardNormal;
let distr = IIDDistr::new(StandardNormal);
let mut rng = rand::thread_rng();
// Each of x element is distributed according to StandardNormal.
let x : [f64; 10] = distr.sample(&mut rng);
```

### An array of 3D vectors sampled from the unit sphere
```rust
use rand_array_iid::IIDDistr;
use rand_distr::Distribution;
use rand_distr::UnitSphere;
let distr = IIDDistr::new(UnitSphere);
let mut rng = rand::thread_rng();
// Each of x element is sampled uniformly from the surface of the 3D unit sphere.
let x : [[f64; 3]; 10] = distr.sample(&mut rng);
```

## Why only arrays?

Collections such as [`Vec`] that implement [`std::iter::FromIterator`] bear
no information on their size in their type, hence the idstribution would have
to be restricted to a given size. They can also be sampled as follow:

```rust
use rand_distr::Distribution;
use rand::Rng;
fn sample_iid<D,R, Col>(dist: D, rng: &mut R, n: usize) -> Col
where
    R: Rng + ?Sized,
    Col: std::iter::IntoIterator,
    Col: std::iter::FromIterator<<Col as std::iter::IntoIterator>::Item>,
    D: Distribution<<Col as std::iter::IntoIterator>::Item>,
{
    dist.sample_iter(rng).take(n).collect()
}
```