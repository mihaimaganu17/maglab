Implemented:
1. Our main parent MagLabApp is made out of tabs
2. Each tab holds a Child App-> Similar to a separate instance of the program
3. Each app renders a Grid of plugins(so far no functionality)

TODO:
1. Implement adding a new plugin
2. Implement removing a plugin
3. Both above should be aware of the current selected block/plugin/window
4. Connect adding/removing a new plugin to UserInterface
5. Create a trait for a plugin
6. Render the plugin name in the block instead of junk names
7. Implement 3 example plugins(FileManager, HexView, Dissassembleview, Hexview)
    - Connect plugins to input
    - Make plugins output something
    - Figure out how you render different outputs
