Implemented:
1. Our main parent MagLabApp is made out of tabs
2. Each tab holds a Child App-> Similar to a separate instance of the program
3. Each app renders a Grid of plugins(so far no functionality)

Next Session:
1. File Manager: Render the name of the plugin and the plugin properly
2. Add file selection to the File Manager listing dirs

TODO Housekeeping:
1. Implement creating a new tab
2. Implmenet deleting a tab
3. Create a default tab configuration
4. Create a default plugin configuration for the new tabs
5. Renaming a tab
6. The default tab name is the focus plugins name

TODO:
1. Add a strip for the controls you can use in the current plugin
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

HardTODO:
1. Stash changes like git(actions)

DONE:
1. Moving between tabs
2. Moving between plugins
3. Implement adding a new plugin
4. Implement removing a plugin
5. Both above are aware of the current selected block/plugin/window
6. Connect adding/removing a new plugin to UserInterface
