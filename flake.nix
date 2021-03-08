{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.naersk.url = "github:nmattia/naersk";
  
  outputs = { self, nixpkgs, naersk }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
      naersk-lib = naersk.lib."x86_64-linux";
      versions = builtins.fromJSON (builtins.readFile ./versions.json);
      mkSonarr = import ./pkgs/sonarr.nix;
      mkRadarr = import ./pkgs/radarr.nix;
      mkJellyfin = import ./pkgs/jellyfin.nix;
    in rec {
      packages."x86_64-linux" = {
        updater = with pkgs; naersk-lib.buildPackage {
          pname = "updater";
          root = ./.;

          nativeBuildInputs = [ pkg-config ];
          buildInputs = [ openssl ];
        };

        sonarr = pkgs.callPackage (mkSonarr versions.sonarr) { };

        radarr = pkgs.callPackage (mkRadarr versions.radarr.master) { };
        radarr-develop =
          pkgs.callPackage (mkRadarr versions.radarr.develop) { };
        radarr-nightly =
          pkgs.callPackage (mkRadarr versions.radarr.nightly) { };

        jellyfin-rc =
          pkgs.callPackage (mkJellyfin versions.jellyfin.stable-rc) { };
      };

      devShell."x86_64-linux" = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rustc
          cargo
          rust-analyzer
          pkg-config
          openssl
        ];
      };
    };
}
