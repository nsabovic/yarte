# Disclaimer
All struct and function in this crate -- crate those based on the real crates -- are entirely fictional. 
All celebrity code are impersonated.
The following program contains coarse language and due to its content it should not be used by anyone.

# Yarte [![Latest version](https://img.shields.io/crates/v/yarte.svg)](https://crates.io/crates/yarte) [![Build Status](https://travis-ci.org/botika/yarte.svg?branch=master)](https://travis-ci.org/botika/yarte) [![Financial Contributors on Open Collective](https://opencollective.com/yarte/all/badge.svg?label=financial+contributors)](https://opencollective.com/yarte)
Yarte stands for **Y**et **A**nother **R**ust **T**emplate **E**ngine, 
is the fastest template engine. Uses a Handlebars-like syntax, 
well-known and intuitive for most developers. Yarte is an optimized, and easy-to-use 
rust crate, with which developers can create logic around their 
HTML templates using conditionals, loops, rust code and template composition. 

## Features
- Meta programming system with all Rust expressions, conditionals, loops or modules
- Recursion in partial at compile time
- Zero copy helpers
- `fmt::Display` and raw `memcopy` implementation for render to bytes
- Render numbers with [Itoa](https://github.com/dtolnay/itoa) and [Dtoa](https://github.com/dtolnay/dtoa) (thanks to dtolnay)
- A [fancy-text debug](https://asciinema.org/a/WEY4Hu17p8qn51b5DEpBVqLL1?autoplay=1) mode to visualize the code generated by Yarte
- Emit snipped annotations at error

### Is it really the fastest?
See it for yourself in the [benchmarks][bench]!

## Documentation
In order to  fully understand Yarte's capabilities take a look at the following documentation:
- [Tests](./yarte/tests)
- [Our book](https://yarte.netlify.com/)
- [Crate documentation](https://docs.rs/yarte/)
- Minimum supported Rust version: 1.42 or later

## Acknowledgment
Yarte is based on all previous templates engines, syntax as well as its documentation 
is highly influenced by [Handlebars][handlebars]. 

[bench]: https://github.com/botika/template-bench-rs#results
[handlebars]: https://handlebarsjs.com/ 

## Contributing

Please, contribute to Yarte! The more the better! Feel free to open an issue and/or contacting directly with the 
owner for any request or suggestion.

### Code of conduct
This Code of Conduct is adapted from the [Contributor Covenant][homepage], version 1.4, available at [http://contributor-covenant.org/version/1/4][version]

[homepage]: http://contributor-covenant.org
[version]: http://contributor-covenant.org/version/1/4/

### License
This project is distributed under the terms of both the Apache License (Version 2.0) and the MIT license, specified in 
[LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) respectively.
