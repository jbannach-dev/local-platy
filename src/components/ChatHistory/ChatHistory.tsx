import { ReactElement, Ref } from "react";
import "./ChatHistory.css";

function ChatHistory({ chatHistoryRef,
    chatHistory }: {
        chatHistoryRef: Ref<HTMLDivElement>
        chatHistory: ReactElement[]
    }) {

    return (<div
        className="chat-history"
        ref={chatHistoryRef}
    >
        {chatHistory}
    </div>)
}

export default ChatHistory;