{ config, pkgs, lib, ... }:
let
  inherit (lib) mkIf mkEnableOption mkOption;

  pkg = pkgs.rustPlatform.buildRustPackage {
    name = "oxidec";

    src = lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;
  };

  cfg = config.oxidec;


  types = lib.types // (import ./types.nix { inherit lib; });
in {
  options.oxidec = {
    enable = mkEnableOption "enable oxidec";

    colorschemes = mkOption {
      type = with types; attrsOf colorschemes;
      default = {};

      # TODO: example = [];
    };

    wallpapers = mkOption {
      type = with types; listOf wallpapers;
      default = [ ];

      # TODO: example = [];
    };

    themes = mkOption {
      type = with types; attrsOf theme;
      default = {};

      # TODO: example = {};
    };
  };

  config = mkIf cfg.enable {
    environment.systemPackages = [ pkg ];
  };
}
