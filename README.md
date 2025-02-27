# Noia - Prozess-Monitoring-Tool

Noia ist eine Desktop-Anwendung zur Überwachung von Systemprozessen mit besonderem Fokus auf AION-Prozesse. Die Anwendung kombiniert die Vorteile von Electron für die plattformübergreifende Desktop-Umgebung, React für die Benutzeroberfläche und Rust für leistungsstarke Backend-Operationen.

![Noia Screenshot](docs/screenshot.png) <!-- Screenshot später hinzufügen -->

## Funktionen

- Echtzeit-Überwachung von AION-Prozessen
- Detaillierte Ansicht von CPU- und Speichernutzung
- Automatische Aktualisierung der Prozessinformationen
- Moderne, reaktionsfähige Benutzeroberfläche

## Technologie-Stack

- **Frontend**: React mit Framer Motion für Animationen
- **Desktop-Framework**: Electron
- **Backend**: Rust über Node.js Native Modules (napi-rs)
- **Styling**: CSS mit Tailwind-Integration

## Systemanforderungen

- Windows 10/11
- Node.js (v14 oder höher)
- Rust und Cargo für die Entwicklung
- 64-Bit Betriebssystem

## Installation

1. Repository klonen:
```
git clone https://github.com/yourusername/noia.git
cd noia
```

2. NPM-Abhängigkeiten installieren:
```
npm install
```

3. Rust-Modul erstellen:
```
cd src/rust
npm install
npm run build
cd ../..
```

4. Anwendung erstellen und starten:
```
npm run build
npm start
```

## Entwicklung

Siehe [CONTRIBUTING.md](CONTRIBUTING.md) für detaillierte Informationen zur Entwicklung und Mitwirkung am Projekt.

## Lizenz

ISC 