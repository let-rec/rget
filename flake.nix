{
  description = "CLI downloader from url";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { 
    self,
    nixpkgs,
    flake-utils,
    ...
    }:
     flake-utils.lib.eachDefaultSystem
    (
      system: let 
        pkgs = nixpkgs.legacyPackages.${system};

        rget = pkgs.rustPlatform.buildRustPackage {
          pname = "rget";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildInputs = [ 
            yt-dlp
            pkgs.openssl
          ];
          meta = with pkgs.lib; {
            description = "CLI to download YouTube videos or files";
            maintainers = with maintainers; [ ];
          };
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
        yt-dlp = pkgs.yt-dlp;
      in {
        formatter = pkgs.alejandra;
        devShells.default = import ./shell.nix {inherit pkgs;};
        packages.default = rget;

        apps.default = {
          flake-utils.lib.mkApp = { drv = rget; };
          type = "app";
          program = "${self.packages.${system}.default}/bin/rget";
        };
      }
    );
}
