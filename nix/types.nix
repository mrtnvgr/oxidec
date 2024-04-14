{ lib, ... }:
let
  inherit (lib) mkOptionType isString hasPrefix types;

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
      special = mkOption {
        type = (submodule {
          options = {
            background = { types = hex; };
            foreground = { types = hex; };
            cursor = { types = hex; };
          };
        });
      };

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
      name = mkOption { type = oneLineStr; };
      # TODO: assert if colorscheme exists
      colorscheme = mkOption { type = oneLineStr; };
      wallpapers = mkOption { type = listOf (wallpaperCache); };
    };
  };
}
