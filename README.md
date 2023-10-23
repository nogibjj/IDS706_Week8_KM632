[![cicd](https://github.com/nogibjj/IDS_Rust_Template_KM632/actions/workflows/rust.yml/badge.svg)](https://github.com/nogibjj/IDS_Rust_Template_KM632/actions/workflows/rust.yml)

## Week 8 Comparing Python to Rust

This project compares the Python and Rust image processing capabilities. 

Both programs takes an image as input and resize and blur the image. It then saves the image as an output. 

Python uses Pillow library whereas Rust uses Image library. 

Devcontianer is created with Rust as the base image. It then installs Python3 as extensions and its dependent libraries.

### To run the Rust program in Codespaces: 

1. Create Github Codespaces
2. Run "gargo build"
3. Run "cargo run"

To run the Python program: 

1. Create Github Codespaces if you haven't already
2. cd into the "python program" directory
3. Run "python3 main.py"

Alternatively, you can clone the repo and complete the steps 2-3. 

### Performace comparison:

Interestengly, I found that Python performed much faster than Rust for image processing. 

**Python: **

Execution Time: 66551336 nanoseconds

CPU Utilization: 15.4%

**Rust:** 

Execution Time: 543938953 nanoseconds

CPU Utilization: 48.00%


