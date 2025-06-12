{
  description = "A devShell example";

  inputs = {
    # nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    nixpkgs.url      = "github:NixOS/nixpkgs/063f43f2dbdef86376cc29ad646c45c46e93234c";
    # rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.url = "github:oxalica/rust-overlay/1cbe817fd8c64a9f77ba4d7861a4839b0b15983e";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      # Pulling lib into outputs
      inherit (nixpkgs) lib;

      # Define the system you're targeting explicitly
      system = "x86_64-linux"; # Or your desired system, e.g., "aarch64-darwin" for Apple Silicon Macs

      # Define the overlays
      overlays = [ (import rust-overlay) ];


      # Import nixpkgs for the specified system with the overlays
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      # Use Python 3.12 from nixpkgs
      python = pkgs.python312;
    in
    {
      # Directly define the devShells for the 'system'
      devShells.${system} = {
        rust = pkgs.mkShell {
          buildInputs = [
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
            })
          ];
        };
        python = pkgs.mkShell {
          packages = [
            python
            pkgs.uv
          ];
          env =
            {
              # Prevent uv from managing Python downloads
              UV_PYTHON_DOWNLOADS = "never";
              # Force uv to use nixpkgs Python interpreter
              UV_PYTHON = python.interpreter;
            }
            // lib.optionalAttrs pkgs.stdenv.isLinux {
              # Python libraries often load native shared objects using dlopen(3).
              # Setting LD_LIBRARY_PATH makes the dynamic library loader aware of libraries without using RPATH for lookup.
              LD_LIBRARY_PATH = lib.makeLibraryPath pkgs.pythonManylinuxPackages.manylinux1;
            };
          shellHook = ''
            unset PYTHONPATH
          '';
        };
      };
    };
}
