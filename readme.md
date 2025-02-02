# JDbrowser - Sqlite Database Browser

Browse an Sqlite database from the terminal.

Licence: GNU GPLv3

# Installation

No Configuration needed.

A binary can be directly built with:

`cargo build --release` 

# Usage

- Run the application from the directory containing the database.

- Select your database from the initial file menu.

- Use the keybindings to browse the tables and views of your database.

## Key Binds

### File Menu

Up: [ **k** ] - Down: [ **j** ] - Select: [ **Enter** ]

### Main view left tab bar

Tab left: [ **q** ] - Tab right: [ **e** ] 

### Main view right tab bar

Tab left: [ **h** ] - Tab right: [ **l** ] 

### Main view left side navigation

Up: [ **k** ] - Down: [ **j** ]  

### Table View

Up [ **shift + k** ] - Down [ **shift + j** ] - left [ **shift + h** ] - right [ **shift + l** ]  

Page Down Half: [ **d** ] - Page Up Half: [ **u** ]  

# Screen Shots

![file_menu](docs/filemenu.png) 

![table_view](docs/table_view.png) 

![schema_view](docs/schema_view.png) 

# TODO

- error handling popup
- handle blob data
- show preview window
- key binding hints 

