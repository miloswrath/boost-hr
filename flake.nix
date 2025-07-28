{
  description = "A Nix-flake-based Python, R, and D3 development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default self.overlays.default ];
        };
      });

      version = "3.13";
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          let
            rust = prev.rust-bin;
          in
          if builtins.pathExists ./rust-toolchain.toml then
            rust.fromRustupToolchainFile ./rust-toolchain.toml
          else if builtins.pathExists ./rust-toolchain then
            rust.fromRustupToolchainFile ./rust-toolchain
          else
            rust.stable.latest.default.override {
              extensions = [ "rust-src" "rustfmt" ];
            };
      };
      devShells = forEachSupportedSystem ({ pkgs }:
        let
          concatMajorMinor = v:
            pkgs.lib.pipe v [
              pkgs.lib.versions.splitVersion
              (pkgs.lib.sublist 0 2)
              pkgs.lib.concatStrings
            ];

          python = pkgs."python${concatMajorMinor version}";

        in
        {
          default = pkgs.mkShellNoCC {
            venvDir = ".venv";


            packages = [
              # python environment
              python.pkgs.venvShellHook
              python.pkgs.pip
              python.pkgs.pandas
              python.pkgs.numpy
              python.pkgs.openpyxl
              # for implementing rust functoins
              pkgs.maturin

              # python visualization
              python.pkgs.matplotlib
              python.pkgs.seaborn
              python.pkgs.plotly

              # jupyter & interactive
              python.pkgs.jupyterlab-widgets
              python.pkgs.jupyterlab
              python.pkgs.ipython
              python.pkgs.ipywidgets

              # web development
              pkgs.nodejs               # node runtime
              pkgs.corepack            # corepack for managing package managers
              pkgs.nodePackages.typescript # typescript needed for the language server
              pkgs.nodePackages.typescript-language-server # typescript language server


              # rust environment
              pkgs.rustToolchain
              pkgs.openssl
              pkgs.pkg-config
              pkgs.cargo-deny
              pkgs.cargo-edit
              pkgs.cargo-watch
              pkgs.rust-analyzer

              # r

              # general tools
              pkgs.git
            ];

            env = {
              # Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };
          };
        }
      );
    };
}

