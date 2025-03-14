import React, { useState, useEffect, createContext } from 'react';
import TitleBar from './components/TitleBar';
import ProcessScanner from './components/ProcessScanner';
import Dashboard from './components/Dashboard';
import './styles/App.css';

// Theme context for app-wide theme state
export const ThemeContext = createContext();

const App = () => {
  const [currentPage, setCurrentPage] = useState('process-scanner');
  const [theme, setTheme] = useState('dark');
  const [selectedProcess, setSelectedProcess] = useState(null);
  const [isProcessActive, setIsProcessActive] = useState(false);

  // Hilfsfunktion für die Navigation zum Prozess-Scanner - auf Komponenten-Level definiert
  const navigateToProcessScanner = () => {
    console.log("Navigation zum Prozess-Scanner");
    
    // Zum Process-Scanner navigieren (direkt oder über Electron)
    if (window.electron && window.electron.app) {
      try {
        window.electron.app.navigateTo('process-scanner');
      } catch (e) {
        console.error('Fehler bei Electron-Navigation:', e);
        // Fallback bei Electron-Fehler
        window.location.hash = 'process-scanner';
        setCurrentPage('process-scanner');
      }
    } else {
      // Manuell navigieren, wenn Electron nicht verfügbar ist
      window.location.hash = 'process-scanner';
      setCurrentPage('process-scanner');
    }
    
    // Prozess als inaktiv markieren
    setIsProcessActive(false);
    // Prozessauswahl zurücksetzen
    setSelectedProcess(null);
  };

  // Handle hash-based routing
  useEffect(() => {
    const handleHashChange = () => {
      const hash = window.location.hash.substring(1);
      if (hash) {
        setCurrentPage(hash);
        
        // Prozessauswahl zurücksetzen, wenn wir zum Process-Scanner navigieren
        if (hash === 'process-scanner') {
          setSelectedProcess(null);
          setIsProcessActive(false);
        }
      } else {
        setCurrentPage('process-scanner');
        // Auch hier Prozessauswahl zurücksetzen, da wir zum Process-Scanner navigieren
        setSelectedProcess(null);
        setIsProcessActive(false);
      }
    };

    window.addEventListener('hashchange', handleHashChange);
    handleHashChange();

    return () => {
      window.removeEventListener('hashchange', handleHashChange);
    };
  }, []);

  // Register IPC listeners for navigation if Electron is available
  useEffect(() => {
    if (window.electron && window.electron.app) {
      const removeListener = window.electron.app.onNavigate((page) => {
        setCurrentPage(page);
        window.location.hash = page;
        
        // Prozessauswahl zurücksetzen, wenn wir zum Process-Scanner navigieren
        if (page === 'process-scanner') {
          setSelectedProcess(null);
          setIsProcessActive(false);
        }
      });
      
      return () => {
        if (removeListener) removeListener();
      };
    }
  }, []);

  // Aktualisiere den ausgewählten Prozess jede Sekunde, wenn ein Prozess ausgewählt ist
  useEffect(() => {
    if (!selectedProcess) return;
    
    const updateProcess = async () => {
      try {
        if (window.electron && window.electron.rustAPI && selectedProcess?.pid) {
          // Prüfe zuerst, ob die PID gültig ist
          if (!selectedProcess.pid || isNaN(selectedProcess.pid)) {
            console.warn('Ungültige PID:', selectedProcess.pid);
            // Direkt navigieren, ohne auf Callbacks zu warten
            navigateToProcessScanner();
            return;
          }
          
          // Hole aktuelle Prozessinformationen basierend auf der PID
          const response = await window.electron.rustAPI.getProcessInfo(selectedProcess.pid);
          
          if (response.success) {
            let processData = response.result;
            
            // Falls das Ergebnis ein String ist, parsen
            if (typeof processData === 'string') {
              try {
                processData = JSON.parse(processData);
                
                // Zusätzliche Prüfung: Stellen Sie sicher, dass die Prozessdaten gültig sind
                if (!processData || !processData.pid) {
                  console.warn('Ungültige Prozessdaten erhalten:', processData);
                  navigateToProcessScanner();
                  return;
                }
                
                // Aktualisiere den ausgewählten Prozess
                setSelectedProcess(processData);
              } catch (parseError) {
                console.error('Fehler beim Parsen der Prozessdaten:', parseError);
                navigateToProcessScanner();
              }
            } else {
              // Aktualisiere den ausgewählten Prozess
              setSelectedProcess(processData);
            }
          } else {
            // Prozess nicht mehr gefunden - kritisch, direktes Umleiten!
            console.warn('Prozess nicht mehr gefunden:', response.error);
            navigateToProcessScanner();
          }
        } else {
          // Wenn Electron oder rustAPI nicht verfügbar sind, auch zum Scanner zurückkehren
          console.warn('Electron oder rustAPI nicht verfügbar');
          navigateToProcessScanner();
        }
      } catch (err) {
        console.error('Fehler beim Aktualisieren der Prozessinformationen:', err);
        // Bei jedem Fehler zum Prozess-Scanner zurückkehren - direkt
        navigateToProcessScanner();
      }
    };
    
    // Aktualisiere direkt beim Laden
    updateProcess();
    
    // Starte das Intervall zur regelmäßigen Aktualisierung (1 Sekunde)
    const intervalId = setInterval(updateProcess, 1000);
    
    return () => clearInterval(intervalId);
  }, [selectedProcess?.pid]);

  // Handle process selection
  const handleProcessSelect = (process) => {
    setSelectedProcess(process);
    setIsProcessActive(true);
    setCurrentPage('dashboard');
  };

  // Handle process update
  const handleProcessUpdate = (process) => {
    if (selectedProcess && process.pid === selectedProcess.pid) {
      setSelectedProcess(process);
    }
  };

  // Handle process not found
  const handleProcessNotFound = () => {
    console.log("Prozess nicht mehr gefunden, leite zum Process-Scanner um via handleProcessNotFound");
    // Verwende die Hilfsfunktion
    navigateToProcessScanner();
  };

  // Render the appropriate content based on the current page
  const renderContent = () => {
    switch (currentPage) {
      case 'process-scanner':
        return (
          <ProcessScanner 
            onProcessSelect={handleProcessSelect} 
            onProcessUpdate={handleProcessUpdate}
            onProcessNotFound={handleProcessNotFound}
          />
        );
      case 'dashboard':
        return selectedProcess ? (
          <Dashboard 
            process={selectedProcess}
            isActive={isProcessActive}
          />
        ) : (
          <ProcessScanner 
            onProcessSelect={handleProcessSelect}
            onProcessUpdate={handleProcessUpdate}
            onProcessNotFound={handleProcessNotFound}
          />
        );
      default:
        return (
          <ProcessScanner 
            onProcessSelect={handleProcessSelect}
            onProcessUpdate={handleProcessUpdate}
            onProcessNotFound={handleProcessNotFound}
          />
        );
    }
  };

  return (
    <ThemeContext.Provider value={{ theme, setTheme }}>
      <div className={`app ${theme}`}>
        <TitleBar selectedProcess={selectedProcess} currentPage={currentPage} />
        <div className="app-content">
          {renderContent()}
        </div>
      </div>
    </ThemeContext.Provider>
  );
};

export default App; 