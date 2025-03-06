import React from 'react';
import Home from './pages/Home';
import CodeEditor from './pages/CodeEditor';
import Settings from './pages/Settings';
import '../../styles/components/dashboard/dashboard-content.css';

const DashboardContent = ({ currentPage }) => {
  const renderContent = () => {
    switch (currentPage) {
      case 'home':
        return <Home />;
      case 'code-editor':
        return <CodeEditor />;
      case 'settings':
        return <Settings />;
      default:
        return <Home />;
    }
  };

  return (
    <div className="dashboard-content">
      {renderContent()}
    </div>
  );
};

export default DashboardContent; 