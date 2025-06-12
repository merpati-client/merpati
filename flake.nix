{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      rust-overlay,
      naersk,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = (
        {
          pkgs,
          system,
          ...
        }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          naersk' = pkgs.callPackage naersk {
            cargo = toolchain;
            rustc = toolchain;
            clippy = toolchain;
          };

          merpati = naersk'.buildPackage {
            name = "merpati";
            version = "0.1.0";
            src = ./.;
          };

          buildInputs = with pkgs; [
            expat
            fontconfig
            freetype
            freetype.dev
            libGL
            pkg-config
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            wayland
            libxkbcommon
          ];
        in
        {
          formatter = pkgs.nixfmt-rfc-style;

          packages.default = merpati;

          devShells.default = pkgs.mkShell {
            name = "merpati";
            buildInputs = buildInputs ++ [ toolchain pkgs.openssl pkgs.pkg-config ];
            LD_LIBRARY_PATH = builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;
          };
        }
      );
    };
}
