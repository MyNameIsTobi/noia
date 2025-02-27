# Mitwirken an Noia

Vielen Dank für Ihr Interesse, zur Entwicklung von Noia beizutragen! Dieses Dokument beschreibt die Richtlinien und Standards für die Entwicklung in diesem Projekt.

## Entwicklungsumgebung

- **Betriebssystem**: Windows mit PowerShell als Terminal
- **Entwicklungssprachen**: JavaScript/React, Rust
- **IDE**: Visual Studio Code (empfohlen) oder eine andere IDE Ihrer Wahl

## Grundlegende Entwicklungsprinzipien

### Daten & Werte

- **Echte Daten verwenden** – Verwenden Sie niemals Schätzungen oder Annahmen und niemals Mock-Daten oder Prozesse.
- **Systeminformationen abfragen** – Greifen Sie auf tatsächliche Systemwerte zu (z.B. `navigator.hardwareConcurrency` für CPU-Kerne)
- **Rohdaten bevorzugen** – Zeigen Sie Daten im Originalformat an und formatieren Sie nur bei Bedarf
- **Debug-Ausgaben implementieren** – Bei komplexen Berechnungen sollten die Rohwerte für Debugging-Zwecke verfügbar sein

### Code-Änderungen

- **Stiele Verändern** – Stile werden nicht verändert, nur Funktionalität
- **Minimale Änderungen** – Ändern Sie Code nur dort, wo es wirklich nötig ist
- **Bestehende Struktur respektieren** – Keine kompletten Neuentwicklungen, wenn gezielte Änderungen möglich sind
- **Inkrementell arbeiten** – Ziehen Sie kleine, nachvollziehbare Änderungen großen Umstrukturierungen vor
- **Funktionalität vor Ästhetik** – Nehmen Sie keine "Verschönerungen" ohne funktionalen Mehrwert vor

### Dateiorganisation

- **Dateien aufteilen** – Bei Komponenten über 300 Zeilen sollte eine Aufteilung in Erwägung gezogen werden
- **Modulare Struktur** – Unterteilen Sie Funktionalität in logische Einheiten
- **Namenskonventionen einhalten**:
  - React-Komponenten: PascalCase (z.B. `ProcessItem.js`)
  - Hilfsfunktionen: camelCase (z.B. `formatMemory.js`)
  - CSS-Dateien: kebab-case (z.B. `process-scanner.css`)
- **Imports organisiert halten** – Gruppieren Sie Imports nach Typ (z.B. React, Bibliotheken, eigene Komponenten)

### Technische Entscheidungen

- **Entscheidungen dokumentieren** – Erklären Sie, warum Sie eine bestimmte Implementierung gewählt haben
- **Performance berücksichtigen** – Achten Sie auf unnötige Renders und effiziente Aktualisierungsintervalle
- **Browserkompatibilität** – Implementieren Sie Fallbacks für moderne Browser-APIs
- **Fehlerbehandlung** – Fangen Sie Fehler ab und zeigen Sie benutzerfreundliche Fehlermeldungen an

## Architektur der Anwendung

### Frontend (React)

- Komponentenbasierter Aufbau mit funktionalen Komponenten und Hooks
- Globaler Zustand wird bei Bedarf mit React Context verwaltet
- Styling durch komponentenspezifische CSS-Dateien

### Electron

- `main.js`: Hauptprozess, lädt das Rust-Modul und stellt IPC-Kommunikation bereit
- `preload.js`: Stellt sichere Brücke zwischen Renderer und Hauptprozess her
- `index.js`: Lädt die React-Anwendung und initialisiert die UI

### Rust-Backend

- System-Level-Operationen zur Prozessüberwachung
- Effizienter Zugriff auf Betriebssystemressourcen
- Integration über napi-rs als Node.js-Nativmodul

## Pull-Request-Workflow

1. Fork des Repositories erstellen
2. Feature-Branch erstellen (`git checkout -b feature/meine-neue-funktion`)
3. Änderungen committen (`git commit -am 'Neue Funktion: XYZ'`)
4. Branch pushen (`git push origin feature/meine-neue-funktion`)
5. Pull Request erstellen

## Codeformatierung

- **JavaScript/React**: [Prettier](https://prettier.io/) mit Standardkonfiguration
- **Rust**: `rustfmt` mit Standardkonfiguration
- **CSS**: Sortierten und gruppierten Eigenschaften nach Typ

## Tests

- **React-Komponenten**: Jest und React Testing Library
- **Rust**: Einheitstests mit dem integrierten Rust-Testsystem

## Versionierung

Wir verwenden [Semantic Versioning](https://semver.org/):
- MAJOR: Inkompatible API-Änderungen
- MINOR: Neue Funktionen (abwärtskompatibel)
- PATCH: Bugfixes (abwärtskompatibel) 