
# Rusty IMG v1.0

A simple image processing tool built using Rust.
Note, RustyIMG v2.0 is under development. You can peek into the repo XD: https://github.com/FuryACE007/rusty-img-v2.0.git

## licenses

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/licenses/)
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)


## Author

- [@furyace007](https://github.com/FuryACE007)


## Contributing

Contributions are always welcome!
Just fork the repo and add your creativity to the tool XD


## Installation

You need cargo and Rust installed to run this project.

    
## Run Locally

Clone the project

```bash
  git clone https://github.com/FuryACE007/RustyImg
```

Go to the project directory

```bash
  cd RustyImg
```

Build the project

```bash
  cargo build --release
```

Commands:

```bash
  cargo run blur infile outfile blur_val
  cargo run brighten infile outfile brighten_val
  cargo run crop infile, outfile, x, y, width, height
  cargo run rotate infile, outfile, rotation_value
  cargo run invert infile, outfile
  cargo run ascii_converter infile, scale_value
  cargo run fractal outfile
```

