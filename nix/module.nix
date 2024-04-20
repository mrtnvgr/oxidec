{ config, pkgs, lib, ... }:
with lib;
let
  types = lib.types // (import ./types.nix { inherit pkgs config lib; });

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

    raw = mkOption {
      type = types.raw;
      default = {};
    };

    files = mkOption {
      type = types.file;
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
      mkJSONFile = group: mapAttrs (name: value: { text = builtins.toJSON value; target = "oxidec/${group}/${name}.json"; }) cfg.${group};
      JSONFiles = mergeAttrsList (map (x: mkJSONFile x) [ "colorschemes" "themes" ]);

      wallpapers = listToAttrs (map (wallpaper: {
        name = "oxidec/wallpapers/${wallpaper.name}";
        value = { source = wallpaper; };
      }) cfg.wallpapers);

      reloaders = let
        rawReloaders = mapAttrs' (name: value: nameValuePair name (value // { executable = true; })) cfg.raw.reloaders;
        fileReloader = let
          mkFile = name: "ln -s $HOME/oxidec/templates/${replaceStrings ["/"] ["^"] name} $HOME/${name}";
          files = map (name: mkFile name) (attrNames cfg.files);
        in concatStringsSep "\n" files;
      in
        if cfg.files != {} then rawReloaders // { "oxidec/reloaders/nix-files.sh".text = fileReloader; } else rawReloaders;

        templates = let
          rawTemplates = cfg.raw.templates;
          # TODO: broken
          fileTemplates = mapAttrs' (name: value: nameValuePair "oxidec/templates/${replaceStrings ["/"] ["^"] name}" value) cfg.files;
        in rawTemplates // fileTemplates;
    in
      JSONFiles // templates // reloaders // wallpapers;

    home.shellAliases = mapAttrs (n: v: "oxidec ${v}") cfg.aliases;

    # TODO: activation scripts
  };
}
