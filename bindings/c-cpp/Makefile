# Makefile for PQC Binary Format C/C++ bindings

# Compiler settings
CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -I../../include
LDFLAGS = -L../../target/release -lpqc_binary_format -ldl -lpthread -lm

# Targets
.PHONY: all clean run

all: example

example: example.cpp
	@echo "Building C++ example..."
	$(CXX) $(CXXFLAGS) -o example example.cpp $(LDFLAGS)

run: example
	@echo "Running C++ example..."
	LD_LIBRARY_PATH=../../target/release:$$LD_LIBRARY_PATH ./example

clean:
	@echo "Cleaning..."
	rm -f example

# Help
help:
	@echo "PQC Binary Format C/C++ Bindings"
	@echo "================================="
	@echo ""
	@echo "Available targets:"
	@echo "  all    - Build the C++ example (default)"
	@echo "  run    - Build and run the C++ example"
	@echo "  clean  - Remove built artifacts"
	@echo "  help   - Show this help message"
	@echo ""
	@echo "Prerequisites:"
	@echo "  1. Build the Rust library first:"
	@echo "     cd ../.. && cargo build --release"
	@echo "  2. Generate C header (if not exists):"
	@echo "     cd ../.. && cbindgen --config cbindgen.toml --output include/pqc_binary_format.h"
