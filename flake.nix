{
  description = "raytracer";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
          NIX_SHELL_MESSAGE = "raytracer";
          buildInputs = [
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
            pkg-config
            cargo-nextest
            libGL
            libxkbcommon
            wayland
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
          RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
