{ pkgs, config, lib, ... }:
let
  inherit (lib) mkOption mkOptionType isString hasPrefix types optionalString;

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

  colorscheme = types.submodule {
    options = {
      palette = mkOption { type = types.attrsOf hex; };
    };
  };

  wallpaperCache = types.submodule {
    options = {
      path = mkOption { type = types.path; };

      mode = mkOption {
        type = types.enum [ "center" "centre" "fill" "max" "scale" "tile" ];
        default = "center";
      };
    };
  };

  theme = types.submodule {
    options = {
      colorscheme = mkOption { type = colorscheme; };
      wallpapers = mkOption { type = types.listOf wallpaperCache; };
    };
  };

  raw = types.submodule {
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

  files = mkFileType "files" "";
}
