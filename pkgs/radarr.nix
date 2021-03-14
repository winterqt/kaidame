manifest:
{ lib, buildFHSUserEnv, stdenv, fetchurl, makeWrapper, autoPatchelfHook, zlib
, lttngUst, curl, icu, openssl, sqlite, libmediainfo }:
let
  pkg = stdenv.mkDerivation rec {
    name = "radarr";
    version = manifest.version;

    src = fetchurl {
      name = "radarr.tar.gz"; # see nixpkgs/107515
      inherit (manifest) url sha256;
    };

    nativeBuildInputs = [ makeWrapper autoPatchelfHook ];

    buildInputs = [ stdenv.cc.cc zlib lttngUst curl ];

    installPhase = ''
      mkdir -p $out/bin $out/radarr
      cp -r * $out/radarr
      echo "${''
        PackageAuthor=[kaidame](https://github.com/winterqt/kaidame)
        UpdateMethod=External
        UpdateMethodMessage=check the commit log and run \`nix flake update\`
        Branch=${manifest.branch}
        PackageVersion=${version}
      ''}" > $out/radarr/package_info
      makeWrapper $out/radarr/Radarr $out/bin/Radarr --prefix LD_LIBRARY_PATH : "${
        lib.makeLibraryPath [ icu openssl sqlite libmediainfo ]
      }"
    '';
  };
  # an FHS is used to use the musl detection, will remove when the fix hits stable
in buildFHSUserEnv {
  name = "Radarr";

  targetPkgs = pkgs: [ pkg ];

  runScript = "/bin/Radarr";
}
