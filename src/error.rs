#[derive(Fail, Debug)]
pub enum ModelError {
    #[fail(display = "Model error")]
    DefaultError,
    // #[fail(display = "invalid toolchain name: {}", name)]
    // InvalidToolchainName {
    //     name: String,
    // },
    // #[fail(display = "unknown toolchain version: {}", version)]
    // UnknownToolchainVersion {
    //     version: String,
    // }
}
