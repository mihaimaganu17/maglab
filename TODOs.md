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
8. Make possible so that the user can modifiy his key configuration(use serde).
9. Make possible so that the user can add new keys with certain functionality
    - Warn the user if the functionality already exists
10. Make a state configuration(use serde and jsons):
    - Save the current tab and application layout/rendering state and
      functional state so that when the user opens it again, it will be the
        same place where he left it
11. Be able to undo a change:
    - Vim style: Save the last `n` changes in cache/memory so that we can undo
      and redo things
12. Create different layout modes to be selected:
    - For example, make a master window that permanently ocupies half the
        space and the other plugins the rest of the space
