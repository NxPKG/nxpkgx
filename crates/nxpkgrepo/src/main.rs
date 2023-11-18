#![feature(panic_info_message)]
#![deny(clippy::all)]

mod panic_handler;

use std::{
    env::{consts, current_exe},
    process,
    process::Stdio,
};

use anyhow::Result;
use dunce::canonicalize as fs_canonicalize;
use tracing::{debug, error, trace};
use nxpkgrepo_lib::{spawn_child, ExecutionState, Payload};

use crate::panic_handler::panic_handler;

fn run_go_binary(execution_state: ExecutionState) -> Result<i32> {
    // canonicalize the binary path to ensure we can find go-nxpkg
    let nxpkg_path = fs_canonicalize(current_exe()?)?;
    let mut go_binary_path = nxpkg_path.clone();
    go_binary_path.pop();
    #[cfg(windows)]
    go_binary_path.push("go-nxpkg.exe");
    #[cfg(not(windows))]
    go_binary_path.push("go-nxpkg");

    if go_binary_path.exists() {
        debug!("Found go binary at {:?}", go_binary_path);
    } else {
        error!("Unable to find Go binary. Please report this issue at https://github.com/nxpkg/nxpkg/issues and include your package manager and version along with the following information:
        os={os}
        arch={arch}
        nxpkg-version={nxpkg_version}
        nxpkg-bin={nxpkg_bin}
        go-nxpkg-bin={go_nxpkg_bin}
        ",
            os = consts::OS,
            arch = consts::ARCH,
            nxpkg_version = nxpkgrepo_lib::get_version(),
            nxpkg_bin = nxpkg_path.display(),
            go_nxpkg_bin = go_binary_path.display()
        );
        // return an error
        return Err(anyhow::anyhow!(
            "Failed to execute nxpkg (Unable to locate Go binary)."
        ));
    }

    if execution_state.cli_args.test_run {
        let serialized_args = serde_json::to_string_pretty(&execution_state)?;
        println!("{}", serialized_args);
        return Ok(0);
    }

    let serialized_args = serde_json::to_string(&execution_state)?;
    trace!("Invoking go binary with {}", serialized_args);
    let mut command = process::Command::new(go_binary_path);
    command
        .arg(serialized_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let child = spawn_child(command)?;
    let exit_code = child.wait()?.code().unwrap_or(2);

    Ok(exit_code)
}

// This function should not expanded. Please add any logic to
// `nxpkgrepo_lib::main` instead
fn main() -> Result<()> {
    std::panic::set_hook(Box::new(panic_handler));

    let exit_code = match nxpkgrepo_lib::main() {
        Payload::Rust(res) => res.unwrap_or(1),
        Payload::Go(base) => {
            let execution_state = (&*base).try_into()?;
            run_go_binary(execution_state)?
        }
    };

    process::exit(exit_code)
}
