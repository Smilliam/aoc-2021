RUSTC = rustc

SRC = sonar.rs
EXECUTABLE = ./sonar

.PHONY: test build run

$(SRC): %: %.rs

test: build
	$(EXECUTABLE) test.txt

run: build
	$(EXECUTABLE) input.txt

build:
	$(RUSTC) $(SRC)
