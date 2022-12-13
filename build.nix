{ lib, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "chess-engine-rs";
  version = "0.1.0";

  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };

  meta = with lib; {
    description = "yet another chess engine, written in rust";
    homepage = "https://github.com/4825764518/chess-engine-rs";
    license = licenses.mit;
  };
}
