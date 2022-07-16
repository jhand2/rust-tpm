# rust-tpm

rust-tpm is a Rust implementation of the TPM 2.0 specification. It consists of
the following crates:
* lib: The TPM library. This implements the commands, structures, and
  functionality defined in the TPM 2.0 library specification.
* sim: A userspace TPM simulator. It exposes both the mstpm TCP interface and
  the swtpm pipe interface.
