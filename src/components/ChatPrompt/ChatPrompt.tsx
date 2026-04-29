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