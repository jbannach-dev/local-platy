import ReactMarkdown from "react-markdown";
import { Prism } from 'react-syntax-highlighter';
import { oneDark } from 'react-syntax-highlighter/dist/esm/styles/prism';
import "./ChatHistory.css";

import type { Components } from 'react-markdown';
import { ReactElement, ReactNode } from "react";

function ChatHistoryItem({ text, participant, thinking = false }: {
    text: string
    participant: string
    thinking?: boolean
}) {

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

    function ChatHistoryItemTemplate({ participant, children }: {
        participant: string
        children: ReactNode
    }): ReactElement {
        return (
            <div
                className={"chat-history-entry chat-history-entry-" + participant}>
                <div className={"chat-history-entry-wrapper chat-history-entry-wrapper-" + participant}>
                    {children}
                </div>
            </div>)
    }

    if (participant === "bot" && thinking) {
        const textSplit = text.split("<\/think>").filter(Boolean);
        return (
            <ChatHistoryItemTemplate participant={participant}>
                <details className={"chat-history-entry-wrapper-" + participant + "-reasoning"}>
                    <summary>Thought Process</summary>
                    <p>{textSplit[0]}</p>
                </details>

                <div className={"chat-history-entry-wrapper-" + participant + "-reasponse"}>
                    <ReactMarkdown
                        components={CustomMarkdownComponents}
                    >{textSplit[1]}</ReactMarkdown>
                </div>
            </ChatHistoryItemTemplate>)
    } else if (participant === "bot") {
        return <ChatHistoryItemTemplate participant={participant}>
            <div className={"chat-history-entry-wrapper-" + participant + "-reasponse"}>
                <ReactMarkdown
                    components={CustomMarkdownComponents}
                >{text}</ReactMarkdown>
            </div>
        </ChatHistoryItemTemplate>
    }

    return (<ChatHistoryItemTemplate participant={participant}>
        <p>{text}</p>
    </ChatHistoryItemTemplate>)
}

export default ChatHistoryItem;