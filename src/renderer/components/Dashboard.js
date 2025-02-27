import React from 'react';
import { motion } from 'framer-motion';
import '../styles/components/dashboard.css';

const Dashboard = ({ selectedProcess }) => {
  // Leere Dashboard-Seite nach Auswahl eines Prozesses anzeigen
  // Keine Informationen und kein Zurück-Button wie gefordert
  return (
    <motion.div 
      className="dashboard-page"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.3 }}
    >
      {/* Hier absichtlich leerer Inhalt, damit wir später erweitern können */}
    </motion.div>
  );
};

export default Dashboard; 