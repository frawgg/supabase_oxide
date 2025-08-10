{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
  flake-utils.lib.eachDefaultSystem
  (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
    with pkgs; {
      devShells.default = mkShell {
        buildInputs = [
          pkg-config
          openssl
          cargo-whatfeatures
          (rust-bin.stable.latest.default.override {
            extensions = [ "rust-analyzer" "rust-src" ];
          })
        ];
      };
    }
  );
}
