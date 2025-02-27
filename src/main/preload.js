const { contextBridge, ipcRenderer } = require('electron');

// Expose protected methods that allow the renderer process to use
// the ipcRenderer without exposing the entire object
contextBridge.exposeInMainWorld('electron', {
  rustAPI: {
    // Expose a function to call any Rust function by name
    callFunction: (functionName, ...args) => ipcRenderer.invoke('rust-function', functionName, ...args),
    
    // Provide convenience methods for specific Rust functions
    addNumbers: (a, b) => ipcRenderer.invoke('rust-function', 'addNumbers', a, b),
    
    // Process monitoring functions
    findProcess: (processName) => ipcRenderer.invoke('rust-function', 'findProcess', processName),
    searchProcesses: (namePattern) => ipcRenderer.invoke('rust-function', 'searchProcesses', namePattern),
    getProcessInfo: (pid) => ipcRenderer.invoke('rust-function', 'getProcessInfo', pid),
    getAllProcesses: () => ipcRenderer.invoke('rust-function', 'getAllProcesses'),
    getSystemInfo: () => ipcRenderer.invoke('rust-function', 'getSystemInfo')
  },
  
  // Add any other APIs you want to expose to the renderer here
  app: {
    // Navigate to different pages within the app
    navigateTo: (page) => ipcRenderer.send('navigate', page),
    
    // Window control functions (minimize, maximize, close)
    windowControl: (action) => ipcRenderer.send('window-control', action),
    
    // Event listeners
    onNavigate: (callback) => {
      ipcRenderer.on('navigate-to', (_, page) => callback(page));
      return () => {
        ipcRenderer.removeAllListeners('navigate-to');
      };
    }
  }
}); 