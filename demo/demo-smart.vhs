# Smart Password Generation Demo
# Demonstrates phonetic and pattern-based generation

Set FontSize 18
Set Width 800
Set Height 500
Set Theme Dracula

Type "echo '🧠 PassGen - Smart Password Generation Demo'"
Sleep 1s
Enter
Sleep 500ms

Type "echo '🎵 Phonetic Password (easier to remember):'"
Sleep 1s
Enter
Sleep 500ms

Type "passgen --phonetic --detailed"
Sleep 3s
Enter
Sleep 2s

Type "echo '📝 Pattern-Based Password (structured):'"
Sleep 1s
Enter
Sleep 500ms

Type "passgen --pattern 'UULLDDSS' --detailed"
Sleep 3s
Enter
Sleep 2s

Type "echo '🔥 Compare with regular password:'"
Sleep 1s
Enter
Sleep 500ms

Type "passgen --detailed"
Sleep 3s
Enter
Sleep 2s
