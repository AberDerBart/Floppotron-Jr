let 
  oxalica_overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ 
    (self: super: {
      openocdpico = super.callPackage ./firmware/openocd/with_picoprobe.nix {}; 
    })
    oxalica_overlay
  ]; };
  rust_channel = pkgs.rust-bin.stable."1.56.1".default;
in pkgs.mkShell {
  buildInputs = [
    (rust_channel.override {
      extensions = [ "rust-src" ];
      targets = [ "thumbv6m-none-eabi" ];
    })
    pkgs.gdb-multitarget
    pkgs.openocdpico
    pkgs.screen
    pkgs.probe-run
    pkgs.flip-link
    # pkgs.pkgsCross.arm-embedded.buildPackages.gcc
    # pkgs.pkgsCross.arm-embedded.buildPackages.gdb
  ];
  nativeBuildInputs = [
    pkgs.pkgsCross.arm-embedded.buildPackages.gcc
  ];
}
