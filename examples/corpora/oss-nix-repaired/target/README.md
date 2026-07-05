# OS-nixCfg

OS-nixCfg is a reproducible NixOS and Home Manager flake template that lets one person declare, rebuild, and share every machine they own — laptop, server, or WSL — from a single repository.

## Quickstart

Clone the repo, edit host variables, and run `nixos-rebuild switch --flake .#hostname`.

## Use cases

- A reproducible flake template for a personal NixOS and Home Manager machine fleet.
- An example workflow for keeping macOS (nix-darwin), NixOS, and WSL hosts in one flake.
- When to use: you want declarative, reproducible machines without writing a flake layout from scratch.

## Repository topology

The repository stores hosts, users, overlays, secrets, and shared modules. It is optimized for maintaining one person's laptop and server fleet.

## Machines

Hosts are arranged by hardware profile. The flake outputs include nixosConfigurations and homeConfigurations.
