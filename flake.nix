{
  description = "A control interface for some MiniDSP products";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        minidsp = pkgs.callPackage ./package.nix { };
      in
      {
        packages.default = minidsp;

        apps = {
          default = flake-utils.lib.mkApp {
            drv = minidsp;
          };
          minidspd = flake-utils.lib.mkApp {
            drv = minidsp;
            exePath = "/bin/minidspd";
          };
        };

        devShells.default = pkgs.mkShell {
          # The nix CC wrapper sets --sysroot=/nix/store/does/not/exist which prevents the rpath on libusb to be properly read
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.udev ];

          buildInputs = minidsp.buildInputs;
          nativeBuildInputs = minidsp.nativeBuildInputs;
        };
      });
}
