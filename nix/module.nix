{ config, pkgs, lib, ... }:
let
  inherit (lib) mkIf mkEnableOption mkOption mapAttrs mergeAttrsList;

  pkg = pkgs.rustPlatform.buildRustPackage {
    name = "oxidec";

    src = lib.cleanSource ./..;
    cargoLock.lockFile = ./../Cargo.lock;
  };

  cfg = config.oxidec;

  types = lib.types // (import ./types.nix { inherit lib; });

  mkFiles = group: mapAttrs (name: value: { text = builtins.toJSON value; target = "oxidec/${group}/${name}.json"; }) config.oxidec.${group};
in {
  options.oxidec = {
    enable = mkEnableOption "enable oxidec";

    colorschemes = mkOption {
      type = with types; attrsOf colorscheme;
      default = {};

      # TODO: example = [];
    };

    wallpapers = mkOption {
      type = with types; listOf wallpaper;
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
    home.packages = [ pkg ];

    xdg.configFile = mergeAttrsList (map (x: mkFiles x) [ "colorschemes" "themes" ]);
    # TODO: recommended aliases option
    # TODO: templates
    # TODO: activation scripts
  };
}
