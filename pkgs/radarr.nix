manifest:
{ lib, stdenv, fetchurl, makeWrapper, autoPatchelfHook, zlib, lttngUst, curl
, icu, openssl, sqlite, libmediainfo }:
stdenv.mkDerivation rec {
  name = "radarr";
  version = manifest.version;

  src = fetchurl {
    name = "radarr.tar.gz"; # see nixpkgs/107515
    inherit (manifest) url sha256;
  };

  nativeBuildInputs = [ makeWrapper autoPatchelfHook ];

  buildInputs = [ stdenv.cc.cc zlib lttngUst curl ];

  installPhase = ''
    mkdir -p $out/bin $out/radarr/bin
    cp -r * $out/radarr/bin
    echo "${''
      PackageAuthor=[kaidame](https://github.com/winterqt/kaidame)
      UpdateMethod=External
      UpdateMethodMessage=[check the commit log](https://github.com/winterqt/kaidame/commits/main) and run \`nix flake update\`
      Branch=${manifest.branch}
      PackageVersion=${version}
    ''}" > $out/radarr/package_info
    makeWrapper $out/radarr/bin/Radarr $out/bin/Radarr --prefix LD_LIBRARY_PATH : "${
      lib.makeLibraryPath [ icu openssl sqlite libmediainfo ]
    }"
  '';
}
