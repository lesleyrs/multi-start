# Multi-Start

This project lets you launch your browser bookmarks directly from the terminal (without handle) simply by typing "bm" and any amount of numbers as an argument.
Use "bm ls" to see what numbers are tied to what bookmark.

It's recommended to pipe the output into [`rg`](https://github.com/BurntSushi/ripgrep) and/or [`bat`](https://github.com/sharkdp/bat).

Folders are ignored, and only the Windows + Chrome combination is supported for now.
However changing it to work on other OS and browsers shouldn't be too much work as they use a similar json file.
