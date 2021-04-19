# kaidame

An Nix flake providing auto-updated packages for:
- [Sonarr](https://sonarr.tv)
- [Radarr](https://radarr.video)
- [Jellyfin](https://jellyfin.org)

## Usage

In your `flake.nix`:
```nix
{
    inputs.kaidame.url = "github:winterqt/kaidame";
}
```

Use packages as `kaidame.packages."x86_64-linux".<pkg name>`.

## TODO
- [ ] Provide NixOS modules
- [ ] Provide packages for systems other than `x86_64-linux`
- [ ] Provide better documentation