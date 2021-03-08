manifest:
{ lib, stdenv, fetchurl, makeWrapper, mono, sqlite, libmediainfo }:
stdenv.mkDerivation rec {
  name = "sonarr";
  version = manifest.version;

  src = fetchurl {
    url =
      "https://download.sonarr.tv/v3/phantom-develop/${version}/Sonarr.phantom-develop.${version}.linux.tar.gz";
    inherit (manifest) sha256;
  };

  nativeBuildInputs = [ makeWrapper ];

  installPhase = ''
    mkdir -p $out/bin $out/sonarr
    cp -r * $out/sonarr
    makeWrapper ${mono}/bin/mono $out/bin/Sonarr --add-flags $out/sonarr/Sonarr.exe --prefix LD_LIBRARY_PATH : "${
      lib.makeLibraryPath [ sqlite libmediainfo ]
    }"
  '';
}
