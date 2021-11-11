{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    nixpkgs.lib.recursiveUpdate
      (flake-utils.lib.eachSystem [ "x86_64-linux" ]
        (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
            versions = builtins.fromJSON (builtins.readFile ./versions.json);
            mkSonarr = import ./pkgs/sonarr.nix;
            mkRadarr = import ./pkgs/radarr.nix;
            mkJellyfin = import ./pkgs/jellyfin.nix;
            # This is for Radarr (from Nixpkgs commit c522fec2743ffb95f2bc296f249232d73ae57dd1)
            lttng-ust = pkgs.callPackage (import ./pkgs/lttng-ust-2.10.5.nix) { liburcu = pkgs.callPackage (import ./pkgs/liburcu-0.12.1.nix) { }; };
          in

          {
            packages = {
              sonarr = pkgs.callPackage (mkSonarr versions.sonarr.v3-stable) { };
              sonarr-nightly = pkgs.callPackage (mkSonarr versions.sonarr.v3-nightly) { };
              sonarr-preview = pkgs.callPackage (mkSonarr versions.sonarr.v3-preview) { };

              radarr = pkgs.callPackage (mkRadarr versions.radarr.master) { inherit lttng-ust; };
              radarr-develop = pkgs.callPackage (mkRadarr versions.radarr.develop) { inherit lttng-ust; };
              radarr-nightly = pkgs.callPackage (mkRadarr versions.radarr.nightly) { inherit lttng-ust; };

              jellyfin = pkgs.callPackage (mkJellyfin versions.jellyfin.stable) { };
              jellyfin-rc = pkgs.callPackage (mkJellyfin versions.jellyfin.stable-rc) { };
            };
          }))
      (flake-utils.lib.eachDefaultSystem (system:
        let pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          packages.updater = pkgs.callPackage (import ./pkgs/updater.nix)
            {
              Security = pkgs.darwin.apple_sdk.frameworks.Security;
            };
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rustc
              cargo
              clippy
              openssl
              pkg-config
            ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [ libiconv darwin.apple_sdk.frameworks.Security ]);
          };
        }));
}
