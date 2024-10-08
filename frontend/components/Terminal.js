import React, { useState, useRef, useEffect } from 'react';
import styled from 'styled-components';
import { Upload, Play, ChevronUp, Minimize2, Maximize2, X } from 'react-feather';

const TerminalContainer = styled.section`
  background-color: #1e1e1e;
  color: #d4d4d4;
  font-family: 'Consolas', 'Courier New', monospace;
  border-radius: 8px;
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
  margin: 2rem auto;
  max-width: 1200px;
  overflow: hidden;
  transition: all 0.3s ease;
`;

const TerminalHeader = styled.div`
  background-color: #252526;
  padding: 0.75rem 1rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px solid #3e3e42;
`;

const TerminalTitle = styled.h2`
  margin: 0;
  font-size: 1.2rem;
  color: #ffffff;
  font-weight: 600;
`;

const TerminalBody = styled.div`
  display: flex;
  height: ${props => props.$isMinimized ? '0' : '500px'};
  overflow: hidden;
  transition: height 0.3s ease;
`;

const Panel = styled.div`
  flex: 1;
  padding: 1rem;
  overflow-y: auto;
  position: relative;

  &::-webkit-scrollbar {
    width: 8px;
  }

  &::-webkit-scrollbar-track {
    background: #1e1e1e;
  }

  &::-webkit-scrollbar-thumb {
    background-color: #3e3e42;
    border-radius: 4px;
  }

  &::-webkit-scrollbar-thumb:hover {
    background-color: #4e4e52;
  }
`;

const CodeEditor = styled.textarea`
  width: 100%;
  height: calc(100% - 50px);
  background-color: #1e1e1e;
  color: #d4d4d4;
  border: none;
  resize: none;
  font-family: 'Consolas', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.5;
  padding: 10px;

  &:focus {
    outline: none;
  }

  &::-webkit-scrollbar {
    width: 8px;
  }

  &::-webkit-scrollbar-track {
    background: #1e1e1e;
  }

  &::-webkit-scrollbar-thumb {
    background-color: #3e3e42;
    border-radius: 4px;
  }

  &::-webkit-scrollbar-thumb:hover {
    background-color: #4e4e52;
  }
`;

const ButtonContainer = styled.div`
  display: flex;
  justify-content: space-between;
  margin-top: 10px;
`;

const Button = styled.button`
  background-color: #0e639c;
  color: white;
  border: none;
  padding: 10px 15px;
  cursor: pointer;
  display: flex;
  align-items: center;
  font-size: 14px;
  border-radius: 4px;
  transition: all 0.3s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  border: 1px solid #10b981;

  &:hover {
    background-color: #1177bb;
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
  }

  &:active {
    transform: translateY(0);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  &:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
    border-color: #6c757d;
  }
`;

const HiddenInput = styled.input`
  display: none;
`;

const ResultPanel = styled.pre`
  margin: 0;
  white-space: pre-wrap;
  word-wrap: break-word;
`;

const GreenBold = styled.span`
  color: #4caf50;
  font-weight: bold;
`;

const ScrollUpButton = styled.button`
  position: fixed;
  bottom: 20px;
  right: 20px;
  background-color: #0e639c;
  color: white;
  border: none;
  border-radius: 50%;
  width: 50px;
  height: 50px;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  border: 1px solid #10b981;

  &:hover {
    background-color: #1177bb;
    transform: translateY(-3px);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
  }

  &:active {
    transform: translateY(0);
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  }
`;

const MinimizeButton = styled.button`
  background: none;
  border: none;
  color: #d4d4d4;
  cursor: pointer;
  font-size: 1.2rem;
  display: flex;
  align-items: center;
  transition: color 0.3s ease;

  &:hover {
    color: #ffffff;
  }
`;

const ClearButton = styled.button`
  background: none;
  border: none;
  color: #d4d4d4;
  cursor: pointer;
  font-size: 1.2rem;
  display: flex;
  align-items: center;
  transition: color 0.3s ease;
  position: absolute;
  top: 10px;
  right: 10px;
  z-index: 2;

  &:hover {
    color: #ffffff;
  }
`;

const MinimizedPlaceholder = styled.div`
  background-color: #252526;
  color: #d4d4d4;
  padding: 1rem;
  text-align: center;
  font-style: italic;
  border-top: 1px solid #3e3e42;
`;

function Terminal() {
  const [code, setCode] = useState('');
  const [result, setResult] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [showScrollUp, setShowScrollUp] = useState(false);
  const [isMinimized, setIsMinimized] = useState(false);
  const fileInputRef = useRef(null);
  const terminalRef = useRef(null);

  useEffect(() => {
    const handleScroll = () => {
      setShowScrollUp(window.scrollY > 300);
    };

    window.addEventListener('scroll', handleScroll);

    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const handleFileUpload = (event) => {
    const file = event.target.files[0];
    if (file && file.name.endsWith('.rs')) {
      const reader = new FileReader();
      reader.onload = (e) => setCode(e.target.result);
      reader.readAsText(file);
    } else {
      alert('Please upload a .rs file');
    }
  };

  const handleAudit = async () => {
    if (!code.trim()) {
      alert('Please enter or upload some code first.');
      return;
    }

    setIsLoading(true);
    setResult('Auditing...');

    const blob = new Blob([code], { type: 'text/rust' });
    const formData = new FormData();
    formData.append('contractFile', blob, 'contract.rs');

    try {
      const response = await fetch('http://localhost:3000/audit/upload', {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data = await response.text();
      setResult(formatResult(data));
    } catch (error) {
      console.error('Error:', error);
      setResult(`Error: ${error.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  const formatResult = (result) => {
    return result.replace(/FLAW:/g, '\n<GreenBold>FLAW:</GreenBold>');
  };

  const scrollToTop = () => {
    window.scrollTo({
      top: 0,
      behavior: 'smooth'
    });
  };

  const toggleMinimize = () => {
    setIsMinimized(!isMinimized);
  };

  const clearCode = () => {
    setCode('');
    setResult('');
  };

  return (
    <>
      <TerminalContainer ref={terminalRef}>
        <TerminalHeader>
          <TerminalTitle>Smart Contract Auditor</TerminalTitle>
          <MinimizeButton onClick={toggleMinimize}>
            {isMinimized ? <Maximize2 size={18} /> : <Minimize2 size={18} />}
          </MinimizeButton>
        </TerminalHeader>
        
        <TerminalBody $isMinimized={isMinimized}>
          <Panel>
              <CodeEditor
                value={code}
                onChange={(e) => setCode(e.target.value)}
                placeholder="Enter your Rust smart contract code here or upload a .rs file"
              />
              <ClearButton onClick={clearCode}>
                <X size={18} />
              </ClearButton>
            <ButtonContainer>
              <Button onClick={() => fileInputRef.current.click()}>
                <Upload size={14} style={{ marginRight: '5px' }} />
                Upload .rs File
              </Button>
              <HiddenInput
                type="file"
                ref={fileInputRef}
                onChange={handleFileUpload}
                accept=".rs"
              />
              <Button onClick={handleAudit} disabled={isLoading}>
                <Play size={14} style={{ marginRight: '5px' }} />
                {isLoading ? 'Auditing...' : 'Start Audit'}
              </Button>
            </ButtonContainer>
          </Panel>
          <Panel>
            <ResultPanel dangerouslySetInnerHTML={{ __html: result }} />
          </Panel>
        </TerminalBody>
        {isMinimized && (
          <MinimizedPlaceholder>
            &quot;Code is like humor. When you have to explain it, it&apos;s bad.&quot; - Cory House
          </MinimizedPlaceholder>
        )}
      </TerminalContainer>
      {showScrollUp && (
        <ScrollUpButton onClick={scrollToTop}>
          <ChevronUp size={24} />
        </ScrollUpButton>
      )}
    </>
  );
}

export default Terminal;