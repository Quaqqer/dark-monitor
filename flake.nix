{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      self,
      flake-utils,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        dark-monitor =
          let
            cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          in
          pkgs.rustPlatform.buildRustPackage {
            pname = "dark-monitor";
            version = cargoToml.package.version;

            src = ./.;

            useFetchCargoVendor = true;
            cargoHash = "sha256-cbWcLStXm9wuE+rLLBytn+F42Ctf4AsmZbO2j/YAirg=";

            meta = {
              description = "A desktop-agnostic monitor of the theme for Linux.";
              homepage = "https://github.com/Quaqqer/dark-monitor";

              buildInputs = [ pkgs.dbus ];
              nativeBuildInputs = [ pkgs.pkg-config ];
            };

            nativeBuildInputs = [
              pkgs.installShellFiles
            ];

            postInstall = ''
              # Find build output directory
              BUILD_OUT_DIR="target/*/release/build/${cargoToml.package.name}-*/out"

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
