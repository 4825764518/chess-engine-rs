{ lib, rustPlatform, pname, version, src, cargoLock }:

rustPlatform.buildRustPackage {
  inherit pname version src cargoLock;

  meta = with lib; {
    description = "yet another chess engine, written in rust";
    homepage = "https://github.com/4825764518/chess-engine-rs";
    license = licenses.mit;
  };
}
