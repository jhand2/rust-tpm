# rust-tpm

rust-tpm is a Rust implementation of the TPM 2.0 specification. It consists of
the following crates:
* tpm: The TPM library. This implements the commands, structures, and
  functionality defined in the TPM 2.0 library specification.
* sim: A userspace TPM simulator. It exposes a simple unix pipe interface which
  can be used with go-tpm. Other TSS libraries may work but have not been
  tested.
