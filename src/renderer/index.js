import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './styles/main.css';

// Create a custom style for the acrylic background effect
const applyAcrylicEffect = () => {
  // Create a style element
  const style = document.createElement('style');
  
  // Define CSS for the acrylic effect
  style.textContent = `
    .acrylic-background {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      z-index: -1;
      background: 
        linear-gradient(
          135deg, 
          rgba(18, 18, 18, 0.9) 0%, 
          rgba(30, 30, 30, 0.9) 100%
        );
      backdrop-filter: blur(20px);
      -webkit-backdrop-filter: blur(20px);
      pointer-events: none;
    }
    
    .acrylic-background::before {
      content: "";
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAyCAYAAAAeP4ixAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAYPSURBVGhD7ZpZqFRHFIY99yYx5jExUXFBTYwLuCRRXFBBQTBgouKGgiCIKKKgRFwQH8SgPrijD4JbFHxRFF98i0tEQRCJRMUFNS4xxjWaGJfELcZ+vl/p0/T0mTszd2buDOYPhpk6VaeqTlWdU1U9rQz+dxiWLlQ/NBqNj4Rh4TPRJtFHJf2DXWKfmCFGe6e82CT8r+KiWCna6Z1Kw9uiTuSNXxQzRW+XIg+iQHSKT/AuBcKNqDQel3KkD5eqwA3hRlQaz0s50odLVeATMU80hIeFxl7hR1SaEElh9Q/FHy5nxpMuVQNdTX1dnZ/X1OA1l1f7Zdj1eYG6u4Nja/3pGpP8sl3kSVWgOBKAcWKESx3BbtEtYjV2ik5hOKuIOELcEf+Ix+KhMPJN4c8P2g1MhDXEeA8xxPsVhtvCTWDdQ/GMnxEPxG1xxZl/SIwVn4sPxAh+iLz0g7D3uyLaxTsidhuKwqmiTXQKKvWbOCjWiRHuQ3aJ/uK6OC9eE0MFTxcXD9ExAV3FnwJvYFDRTDtWrFFOFp2fVbcYzHNigjOheHQXJ4WbkKS+mOqXreNdsVdER/BJlN/6Zex4XvBbJfsDf2lywqX0OKGy3oQoTJGLa6pqPBXk+qzIXnW5QzlT7HUmxCHlFnHCYXvcFpPEZuFNk+CW6O/MM8q+YoPIt5XPKidnEoGVJB4gvBVnwvEHi3x8mwm6TUFJxZsFRvwk5hY0q7mDUO8aBzqz7Vl9XYlXxTMOlI2wBdK9ovDhPIZUMG5xpGxuU5Cm3jMG25qY79JgIkIUNFcWvuimyJRSWPtlb2iWa29a8Jl4mw9J+aJ43eVSCLoVzqzPFb2cCbTPmSAdYajzAM2WOEAzl3U2q40tAoOdKQesBInNNfXTYqwHSuJ9Z4LTRCY3wz9ElsSnOFMOiI7wYgQkXWKbM+FgJojsU8Qaz5aCl53Jh8fcFNmdCQegIKcG/Kv4TRzPfBn+0ZlIkB9IB4y745nS0CSmZVb3HG6KJyJILLe7VBo+FL85EwXbYGdKI9rq7KQ/OrO4rDzKmZIwzJngS1wRH7tUGiYLc6FpuCTe9Jp0oCkYYXDDtYeB/kUAXZ0pibecuSlwLxzPRk2WIxQsHHe9Lk3Qm30pbnimOMYJC3STl9ZE2ib4lndvwkJnEsHq0eoxOBaJWEE/xE0eeF0LyCrWPM/X+6UPBSV1LxHgJtilwSqXSsPnzuQ8snM2cN6ZkvhRLHcmxCMR+D9wU0vPwGcOOVMSvnEmdpMBN8XNlBydyyqhCcqZAKg1sZo1yzqRxM/OnHXmlI9F0u5IvClTnAkxU/G+S82DOsOIw84E5x6O0GTlQufQUWemxoZk+IYzIejP38kvJvPLYiOHrnCLDnwnwJtF3mRKRQy2OhN7gNQfCvaSM86UhJPO5MWFZNzT80jlUjNIEpcdOBPC3HcMfLAE/9z3xbfepHmB7S6Fo/1a6WzROYXuUPwivBkSHcNyZ4oAHuKIR5kLPwAJhB0TH3epdRA52SrZXc4n4kQRqVghDBz72JsWjqecISLWvNrITuQ5cVLkBWY7nVNdXUAGnH+wy9lB4D2Xxfs8tqn2aJJo8a2OXQGTrxSstuxN3BDWY0xFHmcSSV0FQdbiEEnjZv89HkMIcniGXcrOJnK24m+x75lI0qf8z1ygG9F3EZVBrUX1/9LJkYyGPIHXtYpwJW3OBVoC/wQJnzO9VD2I6NvBM1F/gkSuXVD58sD09OeWXKAjd7o2QxI8Hv8TZ4TvYkv/sMZbIiG+CfJnBnZWgpPaJ0r+tR4OuP5dL14R3r2a8IpggrydRXF8AiQ6gp5nJNNxxC0+vdxblT0gLrD7cOoL//xIZMfE5DtHHIxUDLuNW0znEz7FE+ljZfYIPtZEXiPSs1Hx/XbUbXZEdnV4iUMQ4Xk3jUqSG3knxIGAn2C1OeDxIST5jq8YkWyOdYrIXG+uWsQ3QhKlJcKnbT4pwCfLXfF0gVWt6KxrFkMUfDz08C7QGhr+BWiE0J/cqVZRAAAAAElFTkSuQmCC');
      opacity: 0.02;
      pointer-events: none;
    }
  `;
  
  // Add the style to the document
  document.head.appendChild(style);
};

// Apply the acrylic effect when the page loads
applyAcrylicEffect();

// Render the application
const container = document.getElementById('root');
const root = createRoot(container);
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
); 