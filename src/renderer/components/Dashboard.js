import React, { useState } from 'react';
import { motion } from 'framer-motion';
import DashboardMenu from './dashboard/DashboardMenu';
import DashboardContent from './dashboard/DashboardContent';
import '../styles/components/dashboard.css';

const Dashboard = ({ selectedProcess }) => {
  const [currentPage, setCurrentPage] = useState('home');
  const [isMenuExpanded, setIsMenuExpanded] = useState(false);

  const handlePageChange = (page) => {
    setCurrentPage(page);
    setIsMenuExpanded(false);
  };

  return (
    <motion.div 
      className="dashboard-page"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ duration: 0.3 }}
    >
      <DashboardMenu 
        currentPage={currentPage}
        onPageChange={handlePageChange}
        isExpanded={isMenuExpanded}
        onToggleExpand={() => setIsMenuExpanded(!isMenuExpanded)}
      />
      <DashboardContent currentPage={currentPage} />
    </motion.div>
  );
};

export default Dashboard; 