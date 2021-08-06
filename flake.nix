{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:winterqt/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          versions = builtins.fromJSON (builtins.readFile ./versions.json);
          mkSonarr = import ./pkgs/sonarr.nix;
          mkRadarr = import ./pkgs/radarr.nix;
          mkJellyfin = import ./pkgs/jellyfin.nix;
        in

        {
          packages = {
            sonarr = pkgs.callPackage (mkSonarr versions.sonarr.v3-stable) { };
            sonarr-nightly = pkgs.callPackage (mkSonarr versions.sonarr.v3-nightly) { };
            sonarr-preview = pkgs.callPackage (mkSonarr versions.sonarr.v3-preview) { };

            radarr = pkgs.callPackage (mkRadarr versions.radarr.master) { };
            radarr-develop = pkgs.callPackage (mkRadarr versions.radarr.develop) { };
            radarr-nightly = pkgs.callPackage (mkRadarr versions.radarr.nightly) { };

            jellyfin = pkgs.callPackage (mkJellyfin versions.jellyfin.stable) { };
            jellyfin-rc = pkgs.callPackage (mkJellyfin versions.jellyfin.stable-rc) { };

            updater = pkgs.callPackage (import ./pkgs/updater.nix)
              {
                Security = pkgs.darwin.apple_sdk.frameworks.Security;
              };
          };

          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rustc
              cargo
              openssl
              pkg-config
            ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [ libiconv darwin.apple_sdk.frameworks.Security ]);
          };
        });
}
