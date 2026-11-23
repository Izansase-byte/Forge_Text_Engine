# Desde el repositorio
cargo run -p editor -- editor/test.txt

# Desde la carpeta del paquete, si renombras el binario a forge
./forge ejemplo.txt

# Forge Text Engine

Forge is the text engine behind this editor (I still don't have name for it 7-7).
It's fully written in Rust.

## Features

- Open and edit an existing UTF-8 text file.
- Insert and delete characters and lines.
- Move the cursor with the arrow keys.
- A status bar with file selected,line and column.
- Saving changes to the original file with ctrl + s.
- Warning when trying to leave a file unsaved.

## Run from source

You need

- Rust
- Cargo

From the repo root run then:

```
cargo run -p editor -- editor/test.txt
```

You can change `editor/test.txt` into any route to a txt file if you want.

## Controls

| Key | Action |
| --- | --- |
| Arrow keys | Move the cursor |
| Enter | Insert new lines |
| Backspace | Delete to the left of the cursor or delete the line and adds it to the upper one |
| Ctrl + S | Saves the current file |
| Esc | Exit without changes or see a warning |

## Future adds and current limitations

Forge edits one file per session right now and undo,redo,selection and supr are in work in progress right now.

