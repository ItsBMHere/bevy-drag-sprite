# run with nix-shell .
# https://github.com/bevyengine/bevy/blob/master/docs/linux_dependencies.md
{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
    buildInputs = [
        pkgs.pkgconfig
        pkgs.alsaLib
        pkgs.libudev
        pkgs.lutris
        pkgs.vulkan-headers
        pkgs.vulkan-loader
        pkgs.vulkan-tools
        pkgs.vulkan-validation-layers
        pkgs.x11
        pkgs.linuxPackages.nvidia_x11
        pkgs.xorg.libXcursor
        pkgs.xorg.libXi
        pkgs.xorg.libXrandr
        #pkgs.nvtop
    ];

#    shellHook =
#    ;
#    extraCmds = ''
#    '';
}
