const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('path');
const url = require('url');

// Import Rust native module (will be available after building)
let rustModule;
try {
  rustModule = require('../rust'); // This will resolve to ../rust/index.js
  console.log('Rust module loaded successfully with functions:', Object.keys(rustModule));
} catch (err) {
  console.error('Failed to load Rust module:', err.message);
  // Provide a minimal implementation if the module isn't built yet
  rustModule = {
    sampleFunction: (input) => `Mock response (Rust module not built): ${input}`,
    addNumbers: (a, b) => a + b
  };
}

// Global variable to store current page
let currentPage = 'process-scanner';

// Keep a global reference to prevent garbage collection
let mainWindow;

// Create main application window
function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1024,
    height: 768,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.js')
    },
    // Flaches Design für Windows
    backgroundColor: '#1E1E1E',
    frame: false,
    titleBarStyle: 'hidden',
    show: false,                 // Don't show until ready
    transparent: false,          // Transparenz deaktivieren für scharfe Kanten
    roundedCorners: false,       // Abgerundete Ecken explizit deaktivieren
    thickFrame: false,           // Standardrahmen deaktivieren
    // Windows-spezifische Einstellungen
    ...(process.platform === 'win32' ? {
      autoHideMenuBar: true,     // Verstecke die Menüleiste
      fullscreenable: true,
      resizable: true,
      maximizable: true,
      minimizable: true
    } : {})
  });

  // Zusätzliche Eigenschaftssetzung für Windows (um sicherzustellen, dass abgerundete Ecken weg sind)
  if (process.platform === 'win32') {
    // Setze zusätzliche Stiloptionen für Windows 10/11
    mainWindow.setMenuBarVisibility(false);
  }

  // Show window when ready to prevent visual flashing
  mainWindow.once('ready-to-show', () => {
    mainWindow.show();
  });

  // Load the application based on environment
  const startUrl = process.env.ELECTRON_START_URL || url.format({
    pathname: path.join(__dirname, '../renderer/dist/index.html'),
    protocol: 'file:',
    slashes: true,
    hash: currentPage // Pass the current page as hash
  });

  mainWindow.loadURL(startUrl);

  // Open DevTools in development mode
  if (process.env.NODE_ENV === 'development') {
    mainWindow.webContents.openDevTools();
  }

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
}

// Initialize app when Electron is ready
app.whenReady().then(() => {
  createWindow();

  // On macOS, recreate window when dock icon is clicked
  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

// Quit when all windows are closed, except on macOS
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

// Set up IPC for communication with Rust native module
ipcMain.handle('rust-function', async (event, functionName, ...args) => {
  try {
    console.log(`Calling Rust function: ${functionName} with args:`, args);
    
    if (!rustModule[functionName]) {
      throw new Error(`Function ${functionName} not found in Rust module`);
    }
    
    const result = await rustModule[functionName](...args);
    return { success: true, result };
  } catch (error) {
    console.error('Error calling Rust function:', error);
    return { 
      success: false, 
      error: error.message 
    };
  }
});

// Handle navigation requests
ipcMain.on('navigate', (event, page) => {
  currentPage = page;
  if (mainWindow) {
    // Send a navigate-to event to the renderer
    mainWindow.webContents.send('navigate-to', page);
    
    // Also update the hash in the URL
    mainWindow.webContents.executeJavaScript(`window.location.hash = "${page}";`)
      .catch(err => console.error('Error setting location hash:', err));
  }
});

// Handle window control events
ipcMain.on('window-control', (event, action) => {
  if (!mainWindow) return;
  
  switch (action) {
    case 'minimize':
      mainWindow.minimize();
      break;
    case 'maximize':
      if (mainWindow.isMaximized()) {
        mainWindow.unmaximize();
      } else {
        mainWindow.maximize();
      }
      break;
    case 'close':
      mainWindow.close();
      break;
    default:
      console.warn(`Unknown window control action: ${action}`);
  }
}); 