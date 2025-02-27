import React, { useEffect, useState, useCallback, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { FaMemory, FaMicrochip, FaSync } from 'react-icons/fa';
import { BiArrowBack } from 'react-icons/bi';
import { ImSpinner8 } from 'react-icons/im';
import '../styles/components/process-scanner.css';

// Funktion zum Formatieren des CPU-Werts mit Berücksichtigung der CPU-Kerne
export const formatCpuPercentage = (value) => {
  if (value === undefined || value === null) return '0%';
  
  // Anzahl der CPU-Kerne dynamisch ermitteln
  const cpuCores = navigator.hardwareConcurrency || 1;
  
  // CPU-Wert pro Kern berechnen (sysinfo gibt Gesamtauslastung über alle Kerne)
  const valuePerCore = value / cpuCores;
  
  // Formatieren und zurückgeben
  return `${valuePerCore.toFixed(1)}%`;
};

// Funktion zum Formatieren des Speicherwerts
export const formatMemory = (bytes) => {
  if (bytes === undefined || bytes === null) return '0 MB';
  
  // Konvertiere KB zu MB
  const mb = bytes / 1024;
  
  if (mb < 1) {
    return `${(bytes / 1024).toFixed(2)} KB`;
  } else if (mb < 1024) {
    return `${mb.toFixed(2)} MB`;
  } else {
    return `${(mb / 1024).toFixed(2)} GB`;
  }
};

const ProcessScanner = ({ onProcessSelect, onProcessUpdate, onProcessNotFound }) => {
  const [processes, setProcesses] = useState([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState(null);
  const [scanInterval, setScanInterval] = useState(null);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [selectedProcess, setSelectedProcess] = useState(null);
  const [initialLoadComplete, setInitialLoadComplete] = useState(false);
  
  // Use refs to avoid stale closure issues with intervals
  const processesRef = useRef(processes);
  processesRef.current = processes;

  // Funktion zum Abrufen der Prozesse
  const fetchProcesses = useCallback(async (isManualRefresh = false) => {
    if (isManualRefresh || !initialLoadComplete) {
      setIsLoading(true);
    }
    setError(null);
    
    try {
      if (window.electron && window.electron.rustAPI) {
        const response = await window.electron.rustAPI.searchProcesses('aion.bin');
        
        if (response.success) {
          let processesData = response.result;
          
          // Falls das Ergebnis ein String ist, versuche es zu parsen
          if (typeof processesData === 'string') {
            processesData = JSON.parse(processesData);
          }
          
          if (Array.isArray(processesData)) {
            setProcesses(processesData);
            if (!initialLoadComplete) {
              setInitialLoadComplete(true);
            }
            
            // Update selectedProcess wenn vorhanden
            if (selectedProcess) {
              const updatedProcess = processesData.find(p => p.pid === selectedProcess.pid);
              if (updatedProcess) {
                setSelectedProcess(updatedProcess);
                if (onProcessUpdate) onProcessUpdate(updatedProcess);
              } else if (onProcessNotFound) {
                onProcessNotFound();
              }
            }
          } else {
            console.warn('Unerwartetes Datenformat für Prozesse:', processesData);
            setProcesses([]);
          }
        } else {
          // Wenn keine Aion Prozesse gefunden wurden, leeres Array setzen
          console.warn('Keine Aion Prozesse gefunden:', response.error);
          setProcesses([]);
        }
      } else {
        // Mock-Daten für Entwicklung ohne Electron
        setProcesses([
          { pid: 1234, name: 'aion.bin', memory_usage_kb: 150000, cpu_usage_percent: 2.5 },
          { pid: 5678, name: 'aion_client.bin', memory_usage_kb: 250000, cpu_usage_percent: 5.1 }
        ]);
      }
    } catch (err) {
      console.error('Fehler beim Abrufen der Prozesse:', err);
      setError('Fehler beim Abrufen der Prozesse. Bitte versuchen Sie es später erneut.');
    } finally {
      setIsLoading(false);
    }
  }, [selectedProcess, onProcessUpdate, onProcessNotFound, initialLoadComplete]);

  // Initialer Prozessabruf und Intervall einrichten
  useEffect(() => {
    fetchProcesses(true); // Beim ersten Aufruf als manuelle Aktualisierung behandeln
    
    // Aktualisiere die Prozessliste jede Sekunde
    const intervalId = setInterval(() => fetchProcesses(false), 1000);
    
    return () => clearInterval(intervalId);
  }, [fetchProcesses]);

  // Handle process selection and navigation
  const handleProcessClick = (process) => {
    setSelectedProcess(process);
    if (onProcessSelect) onProcessSelect(process);
  };
  
  // Format uptime to readable format
  const formatUptime = (seconds) => {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = Math.floor(seconds % 60);
    
    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    } else if (minutes > 0) {
      return `${minutes}m ${secs}s`;
    } else {
      return `${secs}s`;
    }
  };

  // Handle back button click
  const handleBackClick = () => {
    setSelectedProcess(null);
  };

  // Loading placeholders for process items
  const ProcessItemPlaceholders = () => (
    <>
      {[1, 2, 3, 4].map(index => (
        <div key={index} className="process-item-placeholder">
          <div className="placeholder-line name"></div>
          <div className="placeholder-line details"></div>
          <div>
            <div className="placeholder-line metrics"></div>
            <div className="placeholder-line metrics"></div>
          </div>
        </div>
      ))}
    </>
  );

  // Main loading screen - only shown on first load
  if (isLoading && !initialLoadComplete) {
    return (
      <div className="process-scanner">
        <div className="process-scanner-header">
          <motion.h1 
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
          >
            AION Prozesse
          </motion.h1>
          <p className="process-scanner-info">Suche nach AION Prozessen...</p>
        </div>
        <div className="flex justify-center items-center py-12">
          <ImSpinner8 className="animate-spin text-4xl text-teal-500" />
        </div>
      </div>
    );
  }

  // Error screen
  if (error) {
    return (
      <div className="process-scanner">
        <div className="process-scanner-header">
          <h1>AION Prozesse</h1>
          <div className="bg-red-900/20 border border-red-800 p-4 rounded-md mt-4">
            <h2 className="text-red-500 font-semibold mb-2">Fehler beim Scannen</h2>
            <p className="text-red-300">{error}</p>
          </div>
        </div>
      </div>
    );
  }

  // Process detail view
  if (selectedProcess) {
    return (
      <div className="process-scanner">
        <motion.div 
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
        >
          <div className="flex items-center mb-6">
            <button 
              onClick={handleBackClick}
              className="flex items-center text-gray-400 hover:text-white mr-4"
            >
              <BiArrowBack className="mr-2" /> Back to List
            </button>
            <h2 className="font-semibold text-xl">{selectedProcess.name}</h2>
          </div>
          
          <div className="bg-gray-800 p-6 rounded-md">
            <div className="grid grid-cols-2 gap-6">
              <div>
                <h3 className="text-gray-400 mb-2">Process Details</h3>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-gray-400 text-sm">PID</p>
                    <p className="font-semibold">{selectedProcess.pid}</p>
                  </div>
                  <div>
                    <p className="text-gray-400 text-sm">Status</p>
                    <p className="font-semibold flex items-center">
                      <span className="w-2 h-2 bg-green-400 rounded-full mr-2"></span>
                      Running
                    </p>
                  </div>
                  <div>
                    <p className="text-gray-400 text-sm">CPU Usage</p>
                    <p className="font-semibold text-teal-400">{formatCpuPercentage(selectedProcess.cpu_usage_percent)}%</p>
                  </div>
                  <div>
                    <p className="text-gray-400 text-sm">Memory Usage</p>
                    <p className="font-semibold text-purple-400">{formatMemory(selectedProcess.memory_usage_kb)}</p>
                  </div>
                </div>
              </div>
              
              <div>
                <h3 className="text-gray-400 mb-2">System Impact</h3>
                {/* System impact visualization would go here */}
                <p className="text-sm text-gray-400">
                  This process is currently consuming <span className="text-teal-400 font-semibold">{formatCpuPercentage(selectedProcess.cpu_usage_percent)}%</span> of 
                  your CPU and <span className="text-purple-400 font-semibold">{formatMemory(selectedProcess.memory_usage_kb)}</span> of memory.
                </p>
              </div>
            </div>
          </div>
        </motion.div>
      </div>
    );
  }

  // Process list view
  return (
    <div className="process-scanner">
      <div className="process-scanner-header">
        <h1>AION Prozesse</h1>
        <p className="process-scanner-info">
          Wählen Sie einen Prozess aus, um detaillierte Informationen zu sehen.
        </p>
      </div>
      
      <button onClick={() => fetchProcesses(true)} className="refresh-button">
        <FaSync /> Aktualisieren
      </button>
      
      {processes.length === 0 ? (
        <div className="empty-processes">
          <p>Keine AION Prozesse gefunden.</p>
          <p>Starten Sie das Spiel und klicken Sie auf "Aktualisieren".</p>
        </div>
      ) : (
        <div className="process-list-container">
          <div className="process-list">
            {processes.map((process) => (
              <div
                key={process.pid}
                className={`process-item ${
                  selectedProcess && selectedProcess.pid === process.pid ? 'selected' : ''
                }`}
                onClick={() => handleProcessClick(process)}
              >
                <div className="process-pid">PID: {process.pid}</div>
                <div className="process-name">{process.name}</div>
                <div className="process-details">
                  <div className="process-cpu">
                    <FaMicrochip style={{ color: '#4FD1C5' }} /> 
                    {formatCpuPercentage(process.cpu_usage_percent)}
                  </div>
                  <div className="process-memory">
                    <FaMemory style={{ color: '#9F7AEA' }} />
                    {formatMemory(process.memory_usage_kb)}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default ProcessScanner; 