run: tpm-sim
	../sim/target/debug/tpm-sim &
	@sleep 0.5
	go build
	./tpm-tests
	kill $$(ps aux | grep 'tpm-sim' | head -n1 | awk '{print $$2}')

tpm-sim:
	cd ../sim && cargo build
