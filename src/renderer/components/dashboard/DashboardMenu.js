import React from 'react';
import { motion } from 'framer-motion';
import { FaHome, FaCode, FaCog } from 'react-icons/fa';
import { HiMenuAlt2 } from 'react-icons/hi';
import '../../styles/components/dashboard/dashboard-menu.css';

const menuItems = [
  { id: 'home', icon: FaHome, label: 'Home' },
  { id: 'code-editor', icon: FaCode, label: 'Code Editor' },
  { id: 'settings', icon: FaCog, label: 'Settings' }
];

const DashboardMenu = ({ currentPage, onPageChange, isExpanded, onToggleExpand }) => {
  return (
    <motion.div 
      className={`dashboard-menu ${isExpanded ? 'expanded' : ''}`}
      initial={false}
      animate={{ width: isExpanded ? '200px' : '48px' }}
      transition={{ duration: 0.3 }}
    >
      <button className="menu-toggle" onClick={onToggleExpand}>
        <HiMenuAlt2 />
      </button>
      <nav className="menu-items">
        {menuItems.map(({ id, icon: Icon, label }) => (
          <button
            key={id}
            className={`menu-item ${currentPage === id ? 'active' : ''}`}
            onClick={() => onPageChange(id)}
          >
            <Icon className="menu-icon" />
            <span className="menu-label">{label}</span>
          </button>
        ))}
      </nav>
    </motion.div>
  );
};

export default DashboardMenu; 