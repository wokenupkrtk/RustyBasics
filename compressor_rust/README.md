# Rust-File-Compressor
This small Rust program implements a file compression utility using gzip compression. Here is a breakdown of its key components and functionality:


##### a) Dependencies: 
* The program uses the flate2 crate for gzip compression.
* Standard library modules are used for file I/O, argument parsing, and time measurement.



##### b) Command-line Interface:
* The program expects two command-line arguments: source file and target file.
* It checks for the correct number of arguments and exits with an error message if incorrect.



##### c) File Handling:
* Opens the source file for reading, wrapped in a BufReader for efficient reading.
Creates the target file for writing.



##### d) Compression:
* Utilizes GzEncoder from the flate2 crate with default compression level.
Copies data from the input file to the encoder, which writes compressed data to the output file.



##### e) Performance Measurement:
* Uses Instant::now() to measure the elapsed time for the compression process.



##### f) Output:
* Prints the original file size, compressed file size, and elapsed time.




##### g) Error Handling:
* Uses unwrap() for simplicity, which will panic on errors. In a production environment, more robust error handling would be advisable.




##### h) Efficiency:
* Employs buffered reading for improved I/O performance.





This program demonstrates a straightforward implementation of file compression using Rust's ecosystem, showcasing file I/O, third-party crate usage, and basic performance measurement. It's suitable for command-line use but could be enhanced with better error handling and more configuration options for production use.