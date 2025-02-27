import React from 'react';
import { formatCpuPercentage, formatMemory } from './ProcessScanner';

const TitleBar = ({ selectedProcess, currentPage }) => {
  const handleClose = () => {
    if (window.electron) {
      window.electron.app.windowControl('close');
    }
  };

  const handleMinimize = () => {
    if (window.electron) {
      window.electron.app.windowControl('minimize');
    }
  };

  const handleMaximize = () => {
    if (window.electron) {
      window.electron.app.windowControl('maximize');
    }
  };

  // Navigiere zum Prozess-Scanner - mit direkter Ausführung für Dashboard
  const navigateToProcessScanner = (event) => {
    console.log("Navigate to process scanner called");
    // Stoppe Event-Propagation, um sicherzustellen, dass das Event nicht vom übergeordneten Element abgefangen wird
    event.stopPropagation();
    
    if (window.electron && window.electron.app) {
      window.electron.app.navigateTo('process-scanner');
    } else {
      // Fallback für den Fall, dass kein Electron verfügbar ist
      window.location.hash = 'process-scanner';
    }
  };

  // Bestimme, ob Prozessinformationen angezeigt werden sollen und wie sie angezeigt werden sollen
  const showProcessInfo = selectedProcess && currentPage !== 'dashboard';
  const showDashboardProcessInfo = selectedProcess && currentPage === 'dashboard';

  return (
    <header className="title-bar">
      <div className="title-bar-drag-area">
        <div className="title-text">
          Noia
          {showProcessInfo && (
            <>
              <span className="app-separator">{" - "}</span>
              {/* Nicht klickbare PID im Process Scanner */}
              <div className="process-pid-regular">
                PID: {selectedProcess.pid}
              </div>
              <span className="separator">|</span>
              <div className="process-cpu">
                CPU: {formatCpuPercentage(selectedProcess.cpu_usage_percent)}
              </div>
              <span className="separator">|</span>
              <div className="process-memory">
                MEM: {formatMemory(selectedProcess.memory_usage_kb)}
              </div>
            </>
          )}
          {showDashboardProcessInfo && (
            <>
              <span className="app-separator">{" - "}</span>
              {/* Klickbare PID im Dashboard mit eindeutiger Klasse und Styling */}
              <div 
                className="dashboard-pid-link" 
                onClick={navigateToProcessScanner}
                style={{ 
                  cursor: 'pointer', 
                  color: '#38B2AC',
                  backgroundColor: 'rgba(56, 178, 172, 0.1)',
                  padding: '2px 8px',
                  borderRadius: '4px'
                }}
                title="Zurück zum Prozess-Scanner"
              >
                PID: {selectedProcess.pid}
              </div>
              <span className="separator">|</span>
              <div className="process-cpu">
                CPU: {formatCpuPercentage(selectedProcess.cpu_usage_percent)}
              </div>
              <span className="separator">|</span>
              <div className="process-memory">
                Memory: {formatMemory(selectedProcess.memory_usage_kb)}
              </div>
            </>
          )}
        </div>
      </div>
      <div className="window-controls">
        <button
          className="window-control minimize"
          onClick={handleMinimize}
          aria-label="Minimize"
        >
          <svg
            width="12"
            height="12"
            viewBox="0 0 12 12"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <rect
              x="2"
              y="5.5"
              width="8"
              height="1"
              rx="0.5"
              fill="currentColor"
            />
          </svg>
        </button>
        <button
          className="window-control maximize"
          onClick={handleMaximize}
          aria-label="Maximize"
        >
          <svg
            width="12"
            height="12"
            viewBox="0 0 12 12"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <rect
              x="2"
              y="2"
              width="8"
              height="8"
              rx="1"
              stroke="currentColor"
            />
          </svg>
        </button>
        <button
          className="window-control close"
          onClick={handleClose}
          aria-label="Close"
        >
          <svg
            width="12"
            height="12"
            viewBox="0 0 12 12"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M3 3L9 9M9 3L3 9"
              stroke="currentColor"
              strokeWidth="1.5"
              strokeLinecap="round"
            />
          </svg>
        </button>
      </div>
    </header>
  );
};

export default TitleBar; 