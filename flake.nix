{
  inputs = {
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
      };
      pico-sdk-full = pkgs.pico-sdk.override {
        withSubmodules = true;
      };
    in
    {
      packages.x86_64-linux.elf2uf2-rs = pkgs.callPackage ./elf2uf2.nix { };
      devShell.x86_64-linux = pkgs.mkShell {
        nativeBuildInputs = [
          pkgs.openocd-rp2040
        ];
        buildInputs = [
          pkgs.cmake
          pkgs.git
          pkgs.gcc-arm-embedded
          pico-sdk-full
          
          pkgs.rtmidi
          pkgs.alsa-utils

          pkgs.python311
          pkgs.python311Packages.pycryptodomex
          pkgs.python311Packages.mido
          pkgs.python311Packages.rtmidi-python
        ];

        PICO_SDK_PATH = "${pico-sdk-full}/lib/pico-sdk";
      };
    };
}
