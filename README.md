# PathBreaker

## Overview

The PathBreaker project aims to address privacy concerns in the Lightning Network by developing a tool to analyze blinded paths and identify potential recipients. The Lightning Network recently integrated the blinded paths proposal to enhance receiver privacy. However, even with this enhancement, certain aspects of the recipient's identity remain exposed. PathBreaker aims to expose this vulnerability by analyzing blinded paths and comparing them against the public graph to deduce potential recipients.

## Project Structure

The project comprises a program named "Pathbreaker" designed to take a blinded path and a description of the public graph as input. It then processes this information to output a list of possible recipients along with their probabilities of being the intended recipient.

## Features

- **Input Format**: Accepts input in JSON format for both the public graph description and the blinded path representation.
- **Path Analysis**: Analyze the blinded path and public graph to identify potential recipients.
- **Probability Calculation**: Utilizes multivariate normal distribution to calculate the probability of each node being the recipient.
- **Output Format**: Outputs a key-value list of possible recipients along with their respective probabilities.

## Approach

For details on the approach check out [Notes](pathbreaker/notes.md)

## Usage

To use PathBreaker, follow these steps:

1. Ensure you have Rust installed on your system. [Rust's official website](https://www.rust-lang.org/tools/install).
2. Clone the repository to your local machine:

```
git clone git@github.com:shaavan/pathbreaker.git
```

3. Provide the required inputs: JSON description of the public graph and the blinded path. (For reference on format checkout [Test Vectors](pathbreaker/test_vectors/))
4. Build the project using Cargo: `cargo build`
5. Run the `pathbreaker` program: `cargo run -- <input-files>`
6. View the output to identify possible recipients and their probabilities.

## Contributing

Contributions to PathBreaker are welcome! If you have suggestions for improvements or new features, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Authors

- [shaavan](https://github.com/shaavan)

## Acknowledgments

Special thanks to the Chaincode Bitcoin FOSS program for resources and inspiration.

## Disclaimer

PathBreaker is a research project and should be used for educational purposes only. The developers and contributors are not responsible for any misuse of this tool.
