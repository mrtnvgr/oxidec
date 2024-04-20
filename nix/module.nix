{ config, pkgs, lib, ... }:
let
  inherit (lib) mkIf mkEnableOption mkOption mapAttrs mergeAttrsList optionalString listToAttrs;

  types = lib.types // (import ./types.nix { inherit lib; });

  cfg = config.oxidec;
in {
  options.oxidec = {
    enable = mkEnableOption "enable oxidec";

    aliases = mkOption {
      type = with types; attrsOf str;
      description = "Shell aliases";
      default = {
        cs = "colorscheme";
        wl = "wallpaper";
        wp = "wallpaper";
        th = "theme";
      };
    };

    colorschemes = mkOption {
      type = with types; attrsOf colorscheme;
      default = {};
    };

    wallpapers = mkOption {
      type = with types; listOf path;
      default = [ ];
    };

    themes = mkOption {
      type = with types; attrsOf theme;
      default = {};
    };

    raw = mkOption { type = types.raw; };

    files = mkOption {
      type = with types; attrsOf str;
      default = {};
    };
  };

  config = mkIf cfg.enable {
    home.packages = [
      (pkgs.rustPlatform.buildRustPackage {
        name = "oxidec";

        src = lib.cleanSource ./..;
        cargoLock.lockFile = ./../Cargo.lock;
      })
    ];

    xdg.configFile = let
      mkJSONFile = group: mapAttrs (name: value: { text = builtins.toJSON value; target = "oxidec/${group}/${name}.json"; }) config.oxidec.${group};
      JSONFiles = mergeAttrsList (map (x: mkJSONFile x) [ "colorschemes" "themes" ]);

      wallpapers = listToAttrs (map (wallpaper: {
        name = "oxidec/wallpapers/${wallpaper.name}";
        value = { source = wallpaper; };
      }) config.oxidec.wallpapers);

      templates = mapAttrs (name: value: { text = value; target = "oxidec/templates/${name}"; }) config.oxidec.raw.templates;
      reloaders = mapAttrs (name: value: { text = value; target = "oxidec/reloaders/${name}"; executable = true; }) config.oxidec.raw.reloaders;
    in
      JSONFiles // templates // reloaders // wallpapers;

    home.shellAliases = mapAttrs (n: v: "oxidec ${v}") cfg.aliases;

    # TODO: activation scripts
  };
}
