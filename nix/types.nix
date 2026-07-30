{ pkgs, config, lib, ... }:
let
  fileType = (import ./file-type.nix {
    inherit (config.home) homeDirectory;
    inherit pkgs lib;
  }).fileType;
  mkFileType = option: path: fileType "oxidec.${option}" "{env}`HOME`/${path}" "${config.home.homeDirectory}${lib.optionalString (path != "") "/"}${path}";
in rec {
  files = mkFileType "files" "";

  colorscheme = with lib.types; attrsOf str;

  wallpaperCache = lib.types.submodule {
    options = {
      path = lib.mkOption { type = lib.types.path; };

      mode = lib.mkOption {
        type = lib.types.enum [ "center" "fill" "max" "scale" "tile" ];
        default = "center";
      };
    };
  };

  theme = lib.types.submodule {
    options = {
      colorscheme = lib.mkOption { type = colorscheme; };
      wallpapers = lib.mkOption { type = lib.types.listOf wallpaperCache; };
    };
  };

  templates = mkFileType "templates" "oxidec/templates";
  reloaders = mkFileType "reloaders" "oxidec/reloaders";
}
