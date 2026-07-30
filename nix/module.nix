{ config, pkgs, lib, ... }:
let
  types = lib.types // (import ./types.nix { inherit pkgs config lib; });

  cfg = config.oxidec;
in {
  options.oxidec = {
    enable = lib.mkEnableOption "enable oxidec";

    aliases = lib.mkOption {
      type = with types; attrsOf str;
      description = "Shell aliases";
      default = {
        cs = "colorscheme";
        wl = "wallpaper";
        wp = "wallpaper";
        th = "theme";
      };
    };

    colorschemes = lib.mkOption {
      type = with types; attrsOf colorscheme;
      default = {};
    };

    # TODO: { darkforrest = ...; }
    wallpapers = lib.mkOption {
      type = with types; listOf path;
      default = [ ];
    };

    themes = lib.mkOption {
      type = with types; attrsOf theme;
      default = {};
    };

    templates = lib.mkOption {
      type = types.templates;
      default = {};
    };

    reloaders = lib.mkOption {
      type = types.reloaders;
      default = {};
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [
      (pkgs.rustPlatform.buildRustPackage {
        name = "oxidec";

        src = lib.cleanSource ./..;
        cargoLock.lockFile = ./../Cargo.lock;
      })
    ];

    xdg.configFile = let
      mkJSONFile = group: lib.mapAttrs (name: value: { text = builtins.toJSON value; target = "oxidec/${group}/${name}.json"; }) cfg.${group};
      JSONFiles = lib.mergeAttrsList (map (x: mkJSONFile x) [ "colorschemes" "themes" ]);

      wallpapers = lib.listToAttrs (map (wallpaper: {
        name = "oxidec/wallpapers/${wallpaper.name}";
        value = { source = wallpaper; };
      }) cfg.wallpapers);

      reloaders = lib.mapAttrs' (name: value: lib.nameValuePair name (value // { executable = true; })) cfg.reloaders;
      templates = cfg.templates;
    in
      JSONFiles // templates // reloaders // wallpapers;

    home.shellAliases = lib.mapAttrs (n: v: "oxidec ${v}") cfg.aliases;

    # TODO: activation scripts
  };
}
