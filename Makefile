.PHONY: crates/eagle_tcp_client
crates/eagle_tcp_client:
	@cd crates/eagle_tcp_client && cargo build --target x86_64-pc-windows-gnu
	@cp target/x86_64-pc-windows-gnu/debug/EagleTcpClient.dll '/home/arjix/Games/invaxion/drive_c/Program Files/音灵 INVAXION/INVAXION_Data/Plugins/'

.PHONY: crates/rustaxion
crates/rustaxion:
	@cd crates/rustaxion && cargo build

.PHONY: build
build: crates/eagle_tcp_client crates/rustaxion

.PHONY: serve
serve: crates/rustaxion
	@cargo run
