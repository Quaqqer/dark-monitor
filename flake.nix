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

        # Create our nightly rustPlatform
        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal);
          rustc = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.minimal);
        };
      in
      {
        defaultPackage = rustPlatform.buildRustPackage {
          pname = "dark-monitor";
          version = "0.1.0";

          src = ./.;

          cargoSha256 = "dfCbtnZaMK0fTcDHCmCXp81eEb6qcbZqC7IvjBwOin8=";

          meta = {
            buildInputs = [ pkgs.dbus ];
            nativeBuildInputs = [ pkgs.pkg-config ];
          };
        };
      }
    );
}
