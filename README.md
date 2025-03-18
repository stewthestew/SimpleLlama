> [!NOTE]
> Simple Llama is still in development, not even in it's alpha stage, so expect stuff to break.
> For now use Ollama-rs if you want something that is reliable.
# Simple Llama
Simple Llama is an Ollama wrapper for rust, that makes using Ollama simple yet customizable.


## Nix support
I have gone through so much pain with nix

here is the nix-shell I use.

Keep in mind that every nix system is different

so this might not work with you.

```nix
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
```
