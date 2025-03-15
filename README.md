# GNix - Graphical Nix
An all encompassing Graphical Management interface for Nix and Nixos. Designed to improve on the shortcomings of existing graphical interfaces

## Heavy Development
This project is still in the early stages of development. Due to the complicated nature of Nix and Nixos, and the endless rabbit holes that you can go down with the package manager, it is a very difficult to make an centralized configuration interface that is easy for anyone to use. Many have tried, however most end up unmaintained or restricted in their use cases.

## Goals
The philosophy is simple:
 - One stop shop for all things Nix
 - Not limited to an "app store"
 - Easy for any person to use, technical or not
 - Powerful tool for any Nix user

## Progress
The first stage of development is to get basic Nixos management functionality working. 
[x] - Nix language parser (JSON AST)
[ ] - Initialization of Nixos configuration with flakes, home manager, and git/github
[ ] - Simple package management, add/remove packages from a Nixos
[ ] - common configuration management, Gnome, Plasma, Nvidia drivers etc.

Notice how there has been no mention of a UI, the plan is to get the backend of the application semi-functional before even thinking about designing a user interface. This is so the front-end is designed in a way that feels like.

Yes it's in python, while python has it's... drawbacks, it makes development a whole lot easier. Any functionality that starts to slow down will be migrated to rust with PyO3, such as what's already been done with the Nixel parser

## 