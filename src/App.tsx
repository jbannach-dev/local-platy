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
import ChatPrompt from "./components/ChatPrompt/ChatPrompt";
import ChatHistory from "./components/ChatHistory/ChatHistory";
import ChatHistoryItem from "./components/ChatHistory/ChatHistoryItem";

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
        <ChatHistoryItem key={chatHistory.length} text={prompt} participant="user" />
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
        addMessage(
          <ChatHistoryItem key={chatHistory.length + 1} text={modelResponse} participant="bot" thinking={true} />
        );
      } else {
        addMessage(
          <ChatHistoryItem key={chatHistory.length + 1} text={modelResponse} participant="bot" thinking={true} />
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
      <ChatHistory chatHistory={chatHistory} chatHistoryRef={chatHistoryRef} />
      <ChatPrompt textAreaRef={textAreaRef} handlePrompt={handlePrompt} setPrompt={setPrompt} />
    </main >
  );
}

export default App;