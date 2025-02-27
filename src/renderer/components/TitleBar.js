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

  // Navigiere zur Startseite
  const navigateToHome = () => {
    if (window.electron && window.electron.app) {
      window.electron.app.navigateTo('home');
    } else {
      // Fallback für den Fall, dass kein Electron verfügbar ist
      window.location.hash = 'home';
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
              {" - "}
              <span className="process-pid">PID: {selectedProcess.pid}</span>
              <span className="process-cpu">CPU: {formatCpuPercentage(selectedProcess.cpu_usage_percent)}</span>
              <span className="process-memory">Memory: {formatMemory(selectedProcess.memory_usage_kb)}</span>
            </>
          )}
          {showDashboardProcessInfo && (
            <>
              {" - "}
              <span 
                className="process-pid" 
                onClick={navigateToHome}
                style={{ cursor: 'pointer' }}
                title="Zurück zur Startseite"
              >
                PID: {selectedProcess.pid}
              </span>
              <span className="process-cpu">CPU: {formatCpuPercentage(selectedProcess.cpu_usage_percent)}</span>
              <span className="process-memory">Memory: {formatMemory(selectedProcess.memory_usage_kb)}</span>
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