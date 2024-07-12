{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";

    # We need a rust overlay to use nightly
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        # Create pkgs with rust-overlay
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        # Create our nightly rustPlatform
        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal);
          rustc = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal);
        };

        dark-monitor = rustPlatform.buildRustPackage {
          pname = "dark-monitor";
          version = cargoToml.package.version;

          src = ./.;

          cargoHash = "sha256-/NC8AlIk+itdW+l1Ys5tjYkh3mVXeTYaKpyiPHma+l4=";

          meta = {
            buildInputs = [ pkgs.dbus ];
            nativeBuildInputs = [ pkgs.pkg-config ];
          };

          nativeBuildInputs = [
            pkgs.installShellFiles
            pkgs.tree
          ];

          postInstall = ''
            # Find build output directory
            BUILD_OUT_DIR="target/*/release/build/${cargoToml.package.name}-*/out"

            tree $BUILD_OUT_DIR
            ls -l $BUILD_OUT_DIR

            installShellCompletion --cmd dark-monitor \
              --bash $BUILD_OUT_DIR/dark-monitor.bash \
              --zsh $BUILD_OUT_DIR/_dark-monitor \
              --fish $BUILD_OUT_DIR/dark-monitor.fish
          '';
        };
      in
      {
        packages = {
          default = dark-monitor;
          inherit dark-monitor;
        };
      }
    );
}
