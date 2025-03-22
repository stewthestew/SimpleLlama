{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    buildInputs = with pkgs; [
        nodejs
        lazygit
        openssl 
        pkg-config
    ];
    shellHook = ''
        echo "Openssl shell"
    '';

}
