{ config, pkgs, lib, ... }:
let
  types = lib.types // (import ./types.nix { inherit pkgs config lib; });

  cfg = config.oxidec;

  oxidec = pkgs.rustPlatform.buildRustPackage {
    name = "oxidec";
    src = lib.cleanSource ./..;
    cargoLock.lockFile = ./../Cargo.lock;
  };
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

    wallpapers = lib.mkOption {
      type = with types; attrsOf path;
      default = {};
    };

    themes = lib.mkOption {
      type = with types; attrsOf theme;
      default = {};
    };

    files = lib.mkOption {
      type = types.files;
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
    home.packages = [ oxidec ];

    xdg.configFile = let
      mkJSONFile = group: lib.mapAttrs (name: value: { text = builtins.toJSON value; target = "oxidec/${group}/${name}.json"; }) cfg.${group};
      JSONFiles = lib.mergeAttrsList (map (x: mkJSONFile x) [ "colorschemes" "themes" ]);

      wallpapers = lib.mapAttrs'
        (name: value: lib.nameValuePair "oxidec/wallpapers/${name}" { source = value; })
        cfg.wallpapers;

      # ensure executable permissions
      reloaders = lib.mapAttrs' (name: value: lib.nameValuePair name (value // { executable = true; })) cfg.reloaders;
    in
      JSONFiles // cfg.templates // reloaders // wallpapers;

    oxidec.templates = lib.mapAttrs' (name: value:
      let
        escaped = lib.replaceStrings ["/"] ["^"] name;
      in
        lib.nameValuePair escaped (value // { target = escaped; })
    ) (lib.filterAttrs (_: value: value.enable) cfg.files);

    oxidec.reloaders = lib.optionalAttrs (cfg.files != {}) {
      "_nix.sh" = {
        text = let
          enabled = lib.filterAttrs (_: v: v.enable) cfg.files;

          mkCopy = name: value: let
            escaped = lib.replaceStrings ["/"] ["^"] name;
          in
            ''
              mkdir -p "$(dirname "$HOME/${value.target}")"
              cp "$CACHE/${escaped}" "$HOME/${value.target}"
              ${lib.optionalString (value.executable == true) ''chmod +x "$HOME/${value.target}"''}
            '';
        in ''
          #!/bin/sh
          CACHE="$HOME/.cache/oxidec/templates"
          ${lib.concatStringsSep "\n" (lib.mapAttrsToList mkCopy enabled)}
        '';
      };
    };

    home.shellAliases = lib.mapAttrs (n: v: "oxidec ${v}") cfg.aliases;

    home.activation.oxidec = lib.hm.dag.entryAfter ["writeBoundary"] ''
      ${oxidec}/bin/oxidec colorscheme reload
      ${oxidec}/bin/oxidec wallpaper reload
    '';
  };
}
