# How to contribute to Cornucopia

#### **Did you find a bug?**

* **Do not open up a GitHub issue if the bug is a security vulnerability in Cornucopia**, and instead refer to our [security policy](https://github.com/cornucopia-rs/cornucopia/security/policy).

* **Ensure the bug was not already reported** by searching on GitHub under [Issues](https://github.com/cornucopia-rs/cornucopia/issues).

* If you're unable to find an open issue addressing the problem, [open a new one](https://github.com/cornucopia-rs/cornucopia/issues/new). Be sure to include a **title and clear description**, as much relevant information as possible, and a **code sample** or an **executable test case** demonstrating the expected behavior that is not occurring.

* If possible, provide:
  * Your PostgreSQL version
  * Your Rust version (`rustc --version`)
  * A minimal SQL query that reproduces the issue
  * The generated Rust code (if applicable)
  * The full error message with backtrace

#### **Did you write a patch that fixes a bug?**

* Open a new GitHub pull request with the patch.

* Ensure the PR description clearly describes the problem and solution. Include the relevant issue number if applicable.

* Before submitting, please ensure:
  * You have added tests for your fix
  * All tests pass (`cargo test --all`)
  * Code is formatted (`cargo fmt`)
  * No clippy warnings (`cargo clippy`)

#### **Did you fix whitespace, format code, or make a purely cosmetic patch?**

Changes that are cosmetic in nature and do not add anything substantial to the stability, functionality, or testability of Cornucopia will generally not be accepted. Please focus on meaningful improvements.

#### **Do you intend to add a new feature or change an existing one?**

* First, check if the feature has already been discussed in [Issues](https://github.com/cornucopia-rs/cornucopia/issues).

* Open a new issue describing your proposed feature and start a discussion.

* Do not start working on the feature until you have collected positive feedback about the change.

* For significant changes, consider:
  * How it affects existing users
  * Performance implications
  * Documentation needs
  * Backwards compatibility

#### **Do you have questions about the source code?**

* Check the [README](README.md) and existing documentation first.

* For questions about using Cornucopia, please open a [Discussion](https://github.com/cornucopia-rs/cornucopia/discussions).

#### **Do you want to contribute to the Cornucopia documentation?**

* Documentation improvements are always welcome! This includes:
  * Fixing typos or clarifying existing docs
  * Adding examples
  * Improving API documentation
  * Writing tutorials or guides

* For small changes, feel free to submit a PR directly.

* For larger documentation efforts, please open an issue first to discuss the scope.

#### **Development Setup**

1. Ensure you have Rust installed
2. Clone the repository: `git clone https://github.com/cornucopia-rs/cornucopia.git`
3. Run `cargo run --package test_integration -- --apply-codegen` to codegen the repo and all workspace packages
4. Run `cargo test --all` to ensure all tests pass

#### **Coding Standards**

* Follow Rust naming conventions and idioms
* Use `cargo fmt` to format your code
* Address all `cargo clippy` warnings
* Write tests for new functionality
* Keep commits focused and atomic
* Write clear, descriptive commit messages

Cornucopia is a volunteer effort. We encourage you to pitch in and help make PostgreSQL query generation in Rust even better!
