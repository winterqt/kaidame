manifest:
{ lib, stdenv, fetchurl, makeWrapper, mono, sqlite, libmediainfo }:
stdenv.mkDerivation rec {
  name = "sonarr";
  version = manifest.version;

  src = fetchurl { inherit (manifest) url sha256; };

  nativeBuildInputs = [ makeWrapper ];

  installPhase = ''
    mkdir -p $out/bin $out/sonarr/bin
    cp -r * $out/sonarr/bin
    echo "${''
      PackageAuthor=[kaidame](https://github.com/winterqt/kaidame)
      UpdateMethod=External
      UpdateMethodMessage=check the commit log and run `nix flake update`
      Branch=${manifest.branch}
      PackageVersion=${version}
    ''}" > $out/sonarr/package_info
    makeWrapper ${mono}/bin/mono $out/bin/Sonarr --add-flags $out/sonarr/bin/Sonarr.exe --prefix LD_LIBRARY_PATH : "${
      lib.makeLibraryPath [ sqlite libmediainfo ]
    }"
  '';
}
