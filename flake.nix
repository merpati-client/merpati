{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    git-hooks-nix.url = "github:cachix/git-hooks.nix";
    git-hooks-nix.inputs.nixpkgs.follows = "nixpkgs";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";

    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      imports = [
        inputs.git-hooks-nix.flakeModule
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        { pkgs, system, ... }:
        let
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          icedBuildInputs = with pkgs; [
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
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };

          treefmt.programs = {
            nixfmt.enable = true;
            rustfmt.enable = true;
            rustfmt.package = toolchain;
          };

          pre-commit.settings.hooks = {
            clippy.enable = true;
            clippy.packageOverrides.cargo = toolchain;
            clippy.packageOverrides.clippy = toolchain;
          };

          devShells.default = pkgs.mkShell {
            name = "merpati";
            buildInputs = icedBuildInputs ++ [
              toolchain
              pkgs.openssl
              pkgs.pkg-config
            ];
            LD_LIBRARY_PATH = builtins.foldl' (
              a: b: "${a}:${b}/lib"
            ) "${pkgs.vulkan-loader}/lib" icedBuildInputs;
          };
        };
    };
}
