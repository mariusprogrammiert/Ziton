# Ziton

Ziton ist eine einfache Notizverwaltung für die Konsole.

## Technik

Die Anwendung ist eine Konsolenanwendung, welche in Rust entwickelt wurde.  
Die Notizen werden im JSON-Format in der Datei *notes.json* gespeichert.

## Build und Benutzung

Das Programm kann mit

```
cargo build --release
```

kompiliert werden.
Anschließend kann es im Unterordner target/release/ aufgerufen werden.  
Je nachdem, ob man eine neue Notiz anlegen, alle Notizen anzeigen, eine bestehende Notiz bearbeiten oder löschen möchte, ruft man die Anwendung mit einem der folgenden Befehle auf.

```
./ziton new  
./ziton list  
./ziton edit 3  
./ziton delete 5
```
Die Befehle *edit* und *delete* benötigen die ID der jeweiligen Notiz als zweiten Parameter.
