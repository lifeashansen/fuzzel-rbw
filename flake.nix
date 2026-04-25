{
  description =
    "a minimal rust-powered tool that brings the power of Bitwarden to your Wayland desktop using rbw and fuzzel";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.package.name;
          version = manifest.package.version;

          cargoLock.lockFile = ./Cargo.lock;
          src = ./.;

          cargoBuildFlags = [ "--workspace" ];

          postInstall = ''
            install -Dm644 .assets/bitwarden.png $out/share/pixmaps/bitwarden.png
          '';

          meta = {
            description =
              "A minimal Rust-powered Bitwarden frontend for Fuzzel using rbw";
            homepage = "https://github.com/lifeashansen/fuzzel-rbw";
            license = pkgs.lib.licenses.mit;
            platforms = pkgs.lib.platforms.linux;
            mainProgram = "frbw";
          };

        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            clippy
            rust-analyzer
            rustfmt
            clippy
          ];
        };

      });
}
