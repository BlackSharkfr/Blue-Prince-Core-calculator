# Blue Prince - Core calculator
A small Terminal-UI application to solve a puzzle in the video game `Blue Prince`

Game official website [https://www.blueprincegame.com/]  
Steam : [https://store.steampowered.com/app/1569580/Blue_Prince/]

This week-end project was mainly a pretext to learn how to create a Terminal UI application using [Rust](https://rust-lang.org/) and [ratatui](https://ratatui.rs/).
I am uploading this project following a [discussion on the steam forums](https://steamcommunity.com/app/1569580/discussions/0/599653842304370712/)
I will probably not update this project, ever.

Enjoy!

# Screenshots
<img src="screenshot/blueprince-corecalc1.png" width="50%" alt="Screenshot1" />
<img src="screenshot/blueprince-corecalc2.png" width="50%" alt="Screenshot2" />

# Features
- Decrypt a numeric core from 4 numbers
- Decrypt a numeric core from 4-letter words
- Encrypt all possible words that match a character

# Usage
Can be used either via a Terminal UI (see screenshots), or via CLI.

## Terminal UI
Launch the app in a terminal without arguments  
```
corecalculator.exe
```

## CLI

### Decrypt
```
corecalculator.exe decode HEAT
corecalculator.exe decode "HEAT TICK DATE"
corecalculator.exe decode "34 67 22 4"

```

### Encrypt

```
corecalculator.exe encode H
```
Encrypt output tends to be very long (5000+ lines). It it recommended to pipe the output into a file or an other program
```
corecalculator.exe encode J > file.txt
```

# Licence
MIT : do what you want with it. No warranty
