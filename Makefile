include .env

default:
	cargo build
	cp -r ./shaders target/debug
	./target/debug/fluids-simulation