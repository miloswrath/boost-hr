{ pkgs }:

pkgs.mkShell {
  packages = [
    pkgs.rustToolchain
    pkgs.rust-analyzer
    pkgs.cargo-deny
    pkgs.cargo-edit
    pkgs.cargo-watch
    pkgs.openssl
    pkgs.pkg-config
  ];
}
