# nxpkgrepo-lib

This crate contains most of the logic for the Nxpkgrepo binary and should only be consumed by the `nxpkg` crate.
The `nxpkg` crate handles building the CGO archive and linking it to the Rust code. These crates were split up so that we do not have to build the Go code to run the Rust tests.
