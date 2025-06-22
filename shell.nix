{ pkgs ? import <nixpkgs> {}}: let 
  getLibFolder = pkg: "${pkg}.lib";
in 
  pkgs.stdenv.mkDerivation {
    name = "rget";
    nativeBuildInputs = with pkgs; [
      # LLVM & GCC
      gcc
      cmake
      gnumake
      pkg-config
      llvmPackages.llvm
      llvmPackages.clang

      # Rust
      rustc
      cargo
      rustfmt
      cargo-watch
      rust-analyzer
      yt-dlp
    ];

    buildInputs = with pkgs; [
      openssl
      pkg-config
    ];
    # Set Environment
    RUST_BACKTRACE = 1;
    NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)}";
    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
      pkgs.gcc
      pkgs.libiconv
      pkgs.llvmPackages.llvm
      pkgs.openssl
    ];

    shellHook = 
      ''
        echo "Welcome to rget"
        echo "Use cargo run"
      '';
  }