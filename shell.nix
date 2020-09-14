{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # add wasm32 support with the mozilla overlay
    (
      latest.rustChannels.nightly.rust.override {
        targets = [ "wasm32-unknown-unknown" ];
      }
    )
    cargo
    wasm-pack
    rustc
    rustfmt
    rustPackages.clippy
    cargo-web
    nodejs
    yarn
  ];
}
