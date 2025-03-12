{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    buildInputs = with pkgs; [
        nodejs
        lazygit
        openssl 
        pkg-config
    ];
    shellHook = ''
        echo "You are in a nix shell now, good luck trying to exit this and accidentally exiting the terminal!"
    '';

}
