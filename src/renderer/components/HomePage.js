import React from 'react';
import { motion } from 'framer-motion';
import { FaChevronLeft, FaCog, FaPalette } from 'react-icons/fa';
import { TbActivityHeartbeat } from 'react-icons/tb';
import { MdOutlineInfo } from 'react-icons/md';
import '../styles/components/home.css';

const HomePage = () => {
  const handleScanProcesses = () => {
    // Navigiere zur Prozess-Scanner-Seite
    if (window.electron && window.electron.app) {
      window.electron.app.navigateTo('process-scanner');
    } else {
      // Fallback für den Fall, dass kein Electron verfügbar ist
      window.location.hash = 'process-scanner';
    }
  };

  return (
    <motion.div 
      className="home-page"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.5 }}
    >      
      <h1 className="text-center text-xl font-semibold mb-6">Willkommen bei Noia</h1>
      <p className="text-center text-gray-400 mb-8">
        Finden und überwachen Sie AION Prozesse auf Ihrem System.
      </p>
      
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <motion.div 
          className="card"
          whileHover={{ x: 4, borderColor: '#38B2AC' }}
          transition={{ type: "spring", stiffness: 300 }}
          onClick={handleScanProcesses}
          style={{ cursor: 'pointer' }}
        >
          <div className="flex items-center mb-2">
            <MdOutlineInfo className="mr-2 text-xl text-blue-400" />
            <h2>Prozesse scannen</h2>
          </div>
          <p>Suchen und überwachen Sie AION Prozesse auf Ihrem System.</p>
        </motion.div>
        
        <motion.div 
          className="card"
          whileHover={{ x: 4, borderColor: '#38B2AC' }}
          transition={{ type: "spring", stiffness: 300 }}
        >
          <div className="flex items-center mb-2">
            <TbActivityHeartbeat className="mr-2 text-xl text-red-400" />
            <h2>Performance Monitor</h2>
          </div>
          <p>Überwachen Sie Echtzeit-Performance-Metriken von AION-Prozessen.</p>
        </motion.div>
        
        <motion.div 
          className="card"
          whileHover={{ x: 4, borderColor: '#38B2AC' }}
          transition={{ type: "spring", stiffness: 300 }}
        >
          <div className="flex items-center mb-2">
            <FaCog className="mr-2 text-xl text-gray-400" />
            <h2>Einstellungen</h2>
          </div>
          <p>Konfigurieren Sie Anwendungseinstellungen und Präferenzen.</p>
        </motion.div>
        
        <motion.div 
          className="card"
          whileHover={{ x: 4, borderColor: '#38B2AC' }}
          transition={{ type: "spring", stiffness: 300 }}
        >
          <div className="flex items-center mb-2">
            <FaPalette className="mr-2 text-xl text-purple-400" />
            <h2>Theme-Anpassung</h2>
          </div>
          <p>Passen Sie das Erscheinungsbild der Anwendung mit verschiedenen Themes an.</p>
        </motion.div>
      </div>
    </motion.div>
  );
};

export default HomePage; 