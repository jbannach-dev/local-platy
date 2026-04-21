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
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [chatHistory, setChatHistory] = useState<ReactElement[]>([]);
  const [prompt, setPrompt] = useState<string>("");

  const isGenereating = useRef<boolean>(false);
  const textAreaRef = useRef<HTMLTextAreaElement>(null);
  const chatHistoryRef = useRef<HTMLDivElement>(null);

  const delay = (ms: number) => new Promise(res => setTimeout(res, ms));

  function addMessage(message: ReactElement) {
    setChatHistory((prev) => [...prev, message])
  }


  async function handlePrompt() {
    if (!isGenereating.current) {
      isGenereating.current = true

      addMessage(
        <div className="chat-history-entry chat-history-entry-user">
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

      const newMessage: string = await invoke("prompt", { text: prompt });

      addMessage(
        <div className="chat-history-entry chat-history-entry-bot">
          <div className="chat-history-entry-wrapper chat-history-entry-wrapper-bot">
            <p >{newMessage}</p>
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

      isGenereating.current = false;
    }
  }

  function handleTextAreaShortcuts(e: React.KeyboardEvent<HTMLTextAreaElement>) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handlePrompt();
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

      <form
        className="chat-prompt"
        onSubmit={(e) => {
          e.preventDefault();
          handlePrompt();
        }}
      >

        <textarea
          id="chat-prompt-input"
          ref={textAreaRef}
          onChange={(e) => setPrompt(e.currentTarget.value)}
          onKeyDown={handleTextAreaShortcuts}
          placeholder="ask me"
        />

        <button type="submit">Send</button>
      </form>
    </main >
  );
}

export default App;
