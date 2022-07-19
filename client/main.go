package main

import (
	"fmt"
	"os"

	"github.com/google/go-tpm/tpm2"
)

func main() {
	var tpmname = "/tmp/rust-tpm"

	client, err := tpm2.OpenTPM(tpmname)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to open TPM device %s: %s\n", tpmname, err)
		return
	}

	err = tpm2.Startup(client, tpm2.StartupClear)
	if err != nil {
		fmt.Fprintf(os.Stderr, "TPM2_Startup failed: %s\n", err)
	}

	fmt.Printf("Success!\n")
}
