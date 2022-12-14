{
  description = "Flake for chess-engine-rs";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
      in {
        devShells.default = import ./shell.nix { inherit pkgs; };

        packages = let
          version = "0.1.0";
          cargoLock = { lockFile = ./Cargo.lock; };
        in rec {
          chess-engine-rs = import ./build.nix {
            inherit (pkgs) lib rustPlatform;
            inherit version cargoLock;
            pname = "chess-engine-rs";
            src = ./.;
          };
          default = chess-engine-rs;
        };
      });
}
