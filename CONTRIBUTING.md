# Contributing to Smartknob

First off, thank you for considering contributing to Smartknob! It's people like you that make Smartknob such a great project.

## Code of Conduct

This project and everyone participating in it is governed by the [Smartknob Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to [smartknob@fernando.mozmail.com](mailto:smartknob@fernando.mozmail.com).

## How Can I Contribute?

### Reporting Bugs

This is a great way to contribute to the project. Before submitting a bug report, please check the [issue tracker](https://github.com/FernandoPerezLara/smartknob/issues) to see if the bug has already been reported. If it has, please add a comment to the existing issue instead of creating a new one.

When submitting a bug report, please include the following information:

- A clear and descriptive title.
- A detailed description of the problem, including steps to reproduce the bug.
- The expected behavior and what actually happened.
- Your environment details (e.g., operating system, Rust version).

### Suggesting Enhancements

If you have an idea for a new feature or an improvement to an existing one, please open an issue in the [issue tracker](https://github.com/FernandoPerezLara/smartknob/issues).

When submitting an enhancement suggestion, please include the following information:

- A clear and descriptive title.
- A detailed description of the proposed enhancement.
- The motivation for the enhancement.

### Pull Requests

We love pull requests! If you're planning to work on a larger contribution, it's a good idea to open an issue first to discuss your ideas.

#### Development Setup

To get started with the development of Smartknob, you'll need to have [Rust](https://www.rust-lang.org/tools/install) and `just` installed.

1.  Fork the repository and clone it to your local machine.
2.  Navigate to the firmware directory: `cd firmware`
3.  Install the required dependencies: `cargo install espflash`
4.  Build the project: `just build`

#### Making Changes

1.  Fork the repository.
2.  Make your changes to the code in the `firmware/` directory.
3.  Navigate to the firmware directory: `cd firmware`
4.  Format your code: `just fmt`
5.  Lint your code: `just check`
6.  Commit your changes: `git commit -m "feat: add some feature"`
7.  Push your changes to your fork: `git push origin my-feature-branch`
8.  Open a pull request against the `main` branch of the original repository.

#### Submitting a Pull Request

When you're ready to submit a pull request, please make sure you have done the following:

-   Your code is well-formatted (run `just fmt` from the `firmware/` directory).
-   Your code is lint-free (run `just check` from the `firmware/` directory).
-   Your commit messages are descriptive.
-   You have updated the documentation if necessary.

## Styleguides

### Git Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

### Rust Style

We follow the official [Rust style guide](https://doc.rust-lang.org/1.0.0/style/index.html). Please run `just fmt` from the `firmware/` directory to format your code before submitting a pull request.
