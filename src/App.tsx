//  Copyright 2026 J. Bannach

//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at

//      http://www.apache.org/licenses/LICENSE-2.0

//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.


import { ReactElement, useRef, useState } from "react";

import ReactMarkdown from "react-markdown";
import { Prism } from 'react-syntax-highlighter';
import { oneDark } from 'react-syntax-highlighter/dist/esm/styles/prism';
import type { Components } from 'react-markdown';

import { invoke } from "@tauri-apps/api/core";

import "./App.css";
import ChatPrompt from "./components/ChatPrompt/ChatPrompt";

function App() {
  const [chatHistory, setChatHistory] = useState<ReactElement[]>([]);
  const [prompt, setPrompt] = useState<string>("");

  const isGenereating = useRef<boolean>(false);
  const textAreaRef = useRef<HTMLTextAreaElement>(null);
  const chatHistoryRef = useRef<HTMLDivElement>(null);


  const CustomMarkdownComponents: Components = {
    code({ className, children }) {
      const matchLanguge = /language-(\w+)/.exec(className || "");
      const isCodeBlock = Boolean(matchLanguge);

      return isCodeBlock ? (
        <Prism
          style={oneDark}
          language={matchLanguge![1]}
          PreTag="div"
        >
          {String(children).replace(/\n$/, '')}
        </Prism>
      ) :


        (<code className={className}>
          {children}
        </code>);
    }
  }


  const delay = (ms: number) => new Promise(res => setTimeout(res, ms));

  function addMessage(message: ReactElement) {
    setChatHistory((prev) => [...prev, message])
  }


  async function handlePrompt() {
    if (!isGenereating.current) {
      isGenereating.current = true

      addMessage(
        <div
          key={chatHistory.length}
          className="chat-history-entry chat-history-entry-user">
          <div className="chat-history-entry-wrapper chat-history-entry-wrapper-user">
            <p>{prompt}</p>
          </div>
        </div>
      );

      await delay(100)
      if (chatHistoryRef.current) {
        chatHistoryRef.current.scrollTo({
          top: chatHistoryRef.current.scrollHeight,
          behavior: "smooth"
        });
      }


      if (textAreaRef.current)
        textAreaRef.current.value = "";

      const modelResponse: string = await invoke("prompt", { text: prompt });
      const isReasoningResponse: boolean = modelResponse.includes("</think>");


      if (isReasoningResponse) {
        const modelResponseSplit = modelResponse.split("<\/think>").filter(Boolean);

        addMessage(
          <div
            key={chatHistory.length + 1}
            className="chat-history-entry chat-history-entry-bot">
            <div className="chat-history-entry-wrapper chat-history-entry-wrapper-bot">

              <details className="chat-history-entry-wrapper-bot-reasoning">
                <summary>Thought Process</summary>
                <p>{modelResponseSplit[0]}</p>
              </details>

              <div className="chat-history-entry-wrapper-bot-reasponse">
                <ReactMarkdown
                  components={CustomMarkdownComponents}
                >{modelResponseSplit[1]}</ReactMarkdown>
              </div>
            </div>
          </div>
        );
      }

      else {
        addMessage(
          <div
            key={chatHistory.length + 1}
            className="chat-history-entry chat-history-entry-bot">
            <div className="chat-history-entry-wrapper chat-history-entry-wrapper-bot">
              <div className="chat-history-entry-wrapper-bot-reasponse">
                <ReactMarkdown
                  components={CustomMarkdownComponents}
                >{modelResponse}</ReactMarkdown>
              </div>
            </div>
          </div>
        );
      }

      await delay(100)
      if (chatHistoryRef.current) {
        chatHistoryRef.current.scrollTo({
          top: chatHistoryRef.current.scrollHeight,
          behavior: "smooth"
        });
      }

      isGenereating.current = false;
    }
  }

  return (
    <main className="container">
      <div
        className="chat-history"
        ref={chatHistoryRef}
      >
        {chatHistory}
      </div>

      <ChatPrompt textAreaRef={textAreaRef} handlePrompt={handlePrompt} setPrompt={setPrompt} />
    </main >
  );
}

export default App;