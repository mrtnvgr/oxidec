{ lib, ... }:
let
  inherit (lib) mkOption mkOptionType isString hasPrefix types removePrefix;

  # Credits: nix-colors <3
  hexBaseType = mkOptionType {
    name = "hex-color";
    descriptionClass = "noun";
    description = "RGB color in hex format";
    check = x: isString x && !(hasPrefix "#" x);
  };
in rec {
  hex = with types; coercedTo str (removePrefix "#") hexBaseType;

  colorscheme = with types; submodule {
    options = {
      palette = mkOption { type = attrsOf hex; };
    };
  };

  wallpaperCache = with types; submodule {
    options = {
      path = mkOption { type = path; };

      mode = mkOption {
        type = enum [ "center" "centre" "fill" "max" "scale" "tile" ];
        default = "center";
      };
    };
  };

  theme = with types; submodule {
    options = {
      colorscheme = mkOption { type = colorscheme; };
      wallpapers = mkOption { type = listOf wallpaperCache; };
    };
  };

  raw = with types; submodule {
    options = {
      templates = mkOption {
        type = attrsOf str;
        default = {};
      };

      reloaders = mkOption {
        type = attrsOf str;
        default = {};
      };
    };
  };
}
