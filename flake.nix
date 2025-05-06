{
  description = "Rust devShell with rust-analyzer support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = pkgs.rust-bin.stable.latest.complete.override {
          extensions = [ "rust-src" ]; # needed for rust-analyzer
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rust
            pkgs.rust-analyzer
          ];

          # Optional: tell rust-analyzer where rust-src is
          RUST_SRC_PATH = "${rust}/lib/rustlib/src/rust/library";

          shellHook = ''
            echo "✅ Rust devShell ready"
          '';
        };
      });
}
