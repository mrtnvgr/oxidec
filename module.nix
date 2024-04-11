{ config, pkgs, lib, ... }:
let
  inherit (lib) mkIf mkEnableOption;

  pkg = pkgs.rustPlatform.buildRustPackage {
    name = "oxidec";

    src = lib.cleanSource ./.;
    cargoLock.lockFile = ./Cargo.lock;
  };

  cfg = config.oxidec;
in {
  options.oxidec = {
    enable = mkEnableOption "enable oxidec";
    themes = {};
  };

  config = mkIf cfg.enable {
    environment.systemPackages = [ pkg ];
  };
}
