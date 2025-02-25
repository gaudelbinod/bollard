/// Example of using `std::error::Error` with bollard
extern crate bollard;

use bollard::Docker;

fn run() -> Result<(), Box<std::error::Error>> {
    #[cfg(unix)]
    let docker1 = Docker::connect_with_unix_defaults()?;
    #[cfg(windows)]
    let docker1 = Docker::connect_with_named_pipe_defaults()?;

    let env_var = std::env::var("ZOOKEEPER_ADDR")?;

    Ok(())
}

fn main() {
    run();

    ()
}
