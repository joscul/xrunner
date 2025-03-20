.PHONY: all build package clean

# The default target builds and packages the project.
all: package

# Build the Windows binary using Cargo.
build:
	cargo build --release --target x86_64-pc-windows-gnu

# Package the executable along with the maps and sprites folders into a zip archive.
package: build
	@echo "Packaging release..."
	# Remove any existing 'release' folder.
	rm -rf release
	# Create a fresh folder to hold the files.
	mkdir release
	# Copy the binary from Cargo's output directory.
	cp target/x86_64-pc-windows-gnu/release/xrunner.exe release/
	# Copy the directories.
	cp -r maps release/
	cp -r sprites release/
	# Create a zip archive containing all the packaged files.
	zip -r xrunner.zip release/*

# Clean up build artifacts.
clean:
	cargo clean
	rm -rf release xrunner.zip

