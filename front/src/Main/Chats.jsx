import React from 'react'
import Chat from "./Chat.jsx";
import {useNavigate} from "react-router-dom";

function Chats({setActiveChatId, chats, current_user_id}) {

    const navigate = useNavigate()

    return (
        <>
            {chats.map((chat) =>
            <Chat
                key={chat.id}
                name={chat.name}
                image={chat.image}
                messages={chat.messages}
                last_unread_message_id={chat.last_read_message_id}
                current_user_id={current_user_id}
                onClick={() => {

                    setActiveChatId(chat.id)
                    navigate("/group")
                }}
            ></Chat>)}
        </>

    )
}

export default Chats;