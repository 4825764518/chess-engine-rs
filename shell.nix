{ pkgs }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    bashInteractive

    # 🤢
    (pkgs.rust-bin.stable.latest.default.override {
      extensions = [ "rust-src" "cargo" "rustc" ];
    })
  ];
  buildInputs = with pkgs; [ rust-analyzer rustfmt clippy ];

  # 🤢
  RUST_SRC_PATH = "${
      pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" ];
      }
    }/lib/rustlib/src/rust/library";
}
