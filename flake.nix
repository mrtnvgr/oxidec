{
  inputs = {};

  outputs = _: rec {
    homeManagerModules = rec {
      oxidec = import ./nix/module.nix;
      default = oxidec;
    };

    homeManagerModule = homeManagerModules.default;
  };
}
