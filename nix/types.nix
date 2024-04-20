{ pkgs, config, lib, ... }:
let
  inherit (lib) mkOption mkOptionType isString hasPrefix types removePrefix optionalString;

  # TODO: get latest version from home-manager repo
  fileType = (import ./file-type.nix {
    inherit (config.home) homeDirectory;
    inherit pkgs lib;
  }).fileType;
  mkFileType = option: path: fileType "oxidec.${option}" "{env}`HOME`/${path}" "${config.home.homeDirectory}${optionalString (path != "") "/"}${path}";
in rec {
  # Credits: nix-colors <3
  hex = mkOptionType {
    name = "hex-color";
    descriptionClass = "noun";
    description = "RGB color in hex format";
    check = x: isString x && hasPrefix "#" x;
  };

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
        type = mkFileType "raw.templates" "oxidec/templates";
        default = {};
      };

      reloaders = mkOption {
        type = mkFileType "raw.reloaders" "oxidec/reloaders";
        default = {};
      };
    };
  };

  file = mkFileType "files" "";
}
