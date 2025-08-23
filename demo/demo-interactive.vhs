# Interactive Mode Demo
# Shows the interactive password building wizard

Set FontSize 18
Set Width 800
Set Height 500
Set Theme Dracula

Type "echo '🎯 PassGen - Interactive Mode Demo'"
Sleep 1s
Enter
Sleep 500ms

Type "passgen --interactive"
Sleep 2s
Enter
Sleep 500ms

# Simulate user input for interactive mode
Sleep 1s
Type "20"
Sleep 500ms
Enter
Sleep 500ms

# Select character sets (press space to select, enter to continue)
Sleep 1s
Type " "
Sleep 200ms
Type " "
Sleep 200ms
Type " "
Sleep 200ms
Type " "
Sleep 200ms
Enter
Sleep 500ms

# Answer about ambiguous characters
Sleep 1s
Type "n"
Sleep 500ms
Enter
Sleep 2s
