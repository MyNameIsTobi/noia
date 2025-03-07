import React, { useState, useEffect, useRef } from 'react';
import { FaFolderOpen, FaSave, FaPlay } from 'react-icons/fa';
import Prism from 'prismjs';
import 'prismjs/components/prism-javascript';
import '../../../styles/components/dashboard/pages/page.css';
import '../../../styles/components/dashboard/pages/code-editor.css';

const CodeEditor = () => {
  const [content, setContent] = useState('');
  const [lineCount, setLineCount] = useState(1);
  const [currentFile, setCurrentFile] = useState(null);
  const [consoleContent, setConsoleContent] = useState([]);
  const lineNumbersRef = useRef(null);
  const codeInputRef = useRef(null);
  const highlightingRef = useRef(null);
  const [highlightedCode, setHighlightedCode] = useState('');

  // Aktualisiere die Zeilennummern und Syntax Highlighting wenn sich der Content ändert
  useEffect(() => {
    const lines = content.split('\n').length;
    setLineCount(lines);

    // Syntax Highlighting aktualisieren
    const highlighted = Prism.highlight(content, Prism.languages.javascript, 'javascript');
    setHighlightedCode(highlighted);

    // Synchronisiere Scroll-Position wenn neue Zeilen hinzugefügt wurden
    if (codeInputRef.current) {
      const { scrollHeight, clientHeight, scrollTop } = codeInputRef.current;
      const maxScroll = scrollHeight - clientHeight;
      
      // Wenn der Cursor nahe am Ende ist, scrolle automatisch nach unten
      if (maxScroll > 0 && scrollTop >= maxScroll - 50) {
        requestAnimationFrame(() => {
          if (codeInputRef.current) {
            const newScrollTop = codeInputRef.current.scrollHeight - codeInputRef.current.clientHeight;
            codeInputRef.current.scrollTop = newScrollTop;
            if (lineNumbersRef.current) {
              lineNumbersRef.current.scrollTop = newScrollTop;
            }
            if (highlightingRef.current) {
              highlightingRef.current.scrollTop = newScrollTop;
            }
          }
        });
      }
    }
  }, [content]);

  // Synchronisiere Scroll-Position zwischen allen Elementen
  const handleScroll = (e) => {
    const scrollTop = e.target.scrollTop;
    const scrollLeft = e.target.scrollLeft;
    
    if (lineNumbersRef.current) {
      lineNumbersRef.current.scrollTop = scrollTop;
    }
    if (highlightingRef.current) {
      highlightingRef.current.scrollTop = scrollTop;
      highlightingRef.current.scrollLeft = scrollLeft;
    }
  };

  // Event Handler
  const handleContentChange = (e) => {
    setContent(e.target.value);
  };

  const handleLoad = async () => {
    if (window.electron && window.electron.fileOps) {
      try {
        const result = await window.electron.fileOps.loadFile();
        if (result.success) {
          setContent(result.content);
          setCurrentFile(result.filePath);
        } else {
          console.error('Failed to load file:', result.error);
        }
      } catch (error) {
        console.error('Error loading file:', error);
      }
    }
  };

  const handleSave = async () => {
    if (window.electron && window.electron.fileOps) {
      try {
        const result = await window.electron.fileOps.saveFile(content);
        if (result.success) {
          setCurrentFile(result.filePath);
        } else {
          console.error('Failed to save file:', result.error);
        }
      } catch (error) {
        console.error('Error saving file:', error);
      }
    }
  };

  const handleExecute = () => {
    // TODO: Implementiere Code ausführen
    console.log('Execute code');
  };

  return (
    <div className="dashboard-page-content">
      <h1 className="page-title">
        Code Editor
        {currentFile && (
          <span className="current-file">
            {` - ${currentFile.split('\\').pop()}`}
          </span>
        )}
      </h1>
      <div className="title-separator" />
      
      <div className="code-editor">
        <div className="toolbar">
          <button className="toolbar-button" onClick={handleLoad}>
            <FaFolderOpen />
            <span>Load</span>
          </button>
          <button className="toolbar-button" onClick={handleSave}>
            <FaSave />
            <span>Save</span>
          </button>
          <button className="toolbar-button" onClick={handleExecute}>
            <FaPlay />
            <span>Execute</span>
          </button>
        </div>
        
        <div className="editor-container">
          <div className="code-section">
            <div className="line-numbers" ref={lineNumbersRef}>
              {Array.from({ length: lineCount }, (_, i) => i + 1).map(num => (
                <div key={num} className="line-number">{num}</div>
              ))}
            </div>
            <div className="code-area">
              <textarea
                ref={codeInputRef}
                className="code-input"
                value={content}
                onChange={handleContentChange}
                onScroll={handleScroll}
                spellCheck="false"
                autoCapitalize="none"
                autoComplete="off"
                autoCorrect="off"
              />
              <div 
                ref={highlightingRef}
                className="code-highlighting"
                dangerouslySetInnerHTML={{ __html: highlightedCode }}
              />
            </div>
          </div>
          
          <div className="console-section">
            <div className="console-header">
              <span className="console-title">Console</span>
            </div>
            <div className="console-content">
              {consoleContent.map((line, index) => (
                <div key={index}>{line}</div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default CodeEditor; 