# GNix - The Graphical Nix Project
<div style="display: flex; align-items: flex-start;">
  <img src="src/assets/images/gnix_robot.png" width="100" style="margin-right: 10px;" align="right">
  <p>
    The Graphical Nix Project is a project that aims to create effective, all-encompassing graphical interfaces for the Nix package manager and the Nix programming language. This project is still in the early stages of development. Due to the complicated nature of Nix and NixOS, and the endless rabbit holes that you can go down with the package manager, it is very difficult to make a centralized configuration interface that is easy for anyone to use. Many have tried; however, most end up unmaintained or restricted in their use cases. Let's see where this goes!
  </p>
</div>

## Goals
What Gnix is:
 - Easy for a non-technical person to use
 - Powerful tool for any Nix *user*
   
What Gnix is not:
 - Limited to an "app store" or simple package downloads, it manages configurations, versions, and more
 - a tool for development (if you are developing on nix, just learn to use it, move out of your comfort zone)
 - limited to NixOs [1]

[1] - currently it is, non-NixOs support will be added before 1st release

## Progress
The first stage of development is to get basic Nixos management functionality working. 
```markdown
 [x] - Nix language parser (JSON AST)
 [ ] - Initialization of Nixos configuration with flakes, home manager, and git/github
 [ ] - Simple package management, add/remove packages from a Nixos
 [ ] - common Nix options management, Gnome, Plasma, Nvidia drivers etc.
```

## Initial Backend Feature Plan

|                   | Must Have | Should Have | Could Have | Won't have |
| --------          | -------   | -------     | -------    | -------    |
| **Utility**       | initialisation of git tracked, flaked nixos config with folder structure/script templates<br><br>initialisation of home manager<br><br>add/remove packages from home manager/nixos config | intergrate with nixpkgs to search for packages<br><br>create/transpose configurations in a style that is readable by humans (no 1000 line .nix files etc.)  | preset configurations for common packages (Gnome, X11/Wayland, Nvidia drivers, etc.) | anything not listed prior
| **Effectiveness** | Time consuming backend features written with Rust bindings or bash <br><br> generated nix modules must make sense to a human reading them <br><br>  | move existing nixos configs into the newly initialised nixos config area | | anything not listed prior
| **Learnability**  | docs with text tutorials, quickstart and technical details | demo video | explaination video | anything not listed prior



