### 0.1.0

- Clap used to manages CLI.
- The directory structure is built.
- Add cli.rs to separate logic of cli to main.
- cpu function is working.
- Now you can use flags in cpu function.

### 0.2.0

- Memory funcion was added, with 3 flags.
- Added frist README.
- Added the struct for "convert".
- Minor fixes in how prints information


### 0.3.0

- Frist utility added, "convert.rs" allow to convert between binary, octal hexadecimal and decimal.
- Now temp.rs is working, for now it reads "core temp", "cpu thermal" and "acpitz" but more services can be added.
  Another thing is that for now it only reads CPU temperatures; to add the GPU, an extra layer of search is needed, 
  but I don't have a dedicated graphics card to test it...