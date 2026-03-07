{
  description = "tinyweb development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rust-analyzer
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs;
        };
      }
    );
}
