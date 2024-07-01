import React from 'react'
import classes from './GroupSettings.module.css'
import {useNavigate} from "react-router-dom";
import MyMessage from "./MyMessage.jsx";
import Message from "./Message.jsx";
import user from "../SearchUser/User.jsx";

function Messages({messages, user_id}) {


    const navigate = useNavigate()
    console.log(messages)
    return (
        <div className={classes.MessagesList}>
            {messages.map((message) => message.sender_id === user_id?
            <MyMessage key={message.id} message={message}></MyMessage>
                :
                <Message key={message.id} message={message} user_id={user_id}></Message>
            )}

        </div>
    )
}

export default Messages;