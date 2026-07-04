# OS-nixCfg

A reproducible personal NixOS and Home Manager configuration with modules, machine topology, and deployment notes.

## Repository topology

The repository stores hosts, users, overlays, secrets, and shared modules. It is optimized for maintaining one person's laptop and server fleet.

## Machines

Hosts are arranged by hardware profile. The flake outputs include nixosConfigurations and homeConfigurations.

## Installation

Clone the repo, edit host variables, and run `nixos-rebuild switch --flake .#hostname`.
