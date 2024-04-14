{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { nixpkgs, ... }: rec {
    homeManagerModules = rec {
      oxidec = import ./nix/module.nix;
      default = oxidec;
    };

    homeManagerModule = homeManagerModules.default;
  };
}
