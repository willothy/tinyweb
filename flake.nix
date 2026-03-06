{
  description = "Freestyle development shell for NixOS";

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
          pkg-config
          clang
          lld

          # Cap'n Proto compiler for capnpc build.rs
          capnproto

          rustc
          cargo
          rust-analyzer
        ];

        buildInputs = with pkgs; [
          # SSL/TLS
          openssl
          openssl.dev

          libmnl
          libnftnl

          sqlite

          libgit2
        ];

        libPath = pkgs.lib.makeLibraryPath buildInputs;

      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            export LD_LIBRARY_PATH="${libPath}:$LD_LIBRARY_PATH"
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"

            export OPENSSL_DIR="${pkgs.openssl.dev}"
            export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
            export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"

            export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"

            export RUSTFLAGS="--cfg tracing_unstable -C link-arg=-fuse-ld=lld"

            # Ensure capnpc finds the capnp compiler
            export CAPNP="${pkgs.capnproto}/bin/capnp"

            exec $(awk -F: -v user="$USER" '$1 == user {print $NF}' /etc/passwd)
          '';
        };
      }
    );
}
