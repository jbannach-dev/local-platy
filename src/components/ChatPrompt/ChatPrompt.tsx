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

import { Ref } from "react";
import "./ChatPrompt.css";

function ChatPrompt({ textAreaRef, handlePrompt, setPrompt }: {
    textAreaRef: Ref<HTMLTextAreaElement>
    handlePrompt: Function
    setPrompt: Function

}) {

    function handleTextAreaShortcuts(e: React.KeyboardEvent<HTMLTextAreaElement>) {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handlePrompt();
        }
    }

    return (
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
    )
}


export default ChatPrompt;