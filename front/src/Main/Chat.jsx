import React from 'react'
import classes from './Main.module.css'
import {GetImageURL} from "../scripts/url.js";
import Message from "./Message.jsx";

function Chat({onClick, image, name, messages, last_unread_message_id, current_user_id}) {


    let last_message = messages.at(-1)

    let unread_number = messages.length - 1 - messages.findLastIndex((message) => message.id === last_unread_message_id )
    console.log(last_message, last_message)

    return (
        <div className={classes.Chat} onClick={onClick}>
            <img
                className={classes.ChatImage}

                src={GetImageURL(image)}
                alt={`${name}\`s image`}
                />
            <div className={classes.texts}>
                <div className={classes.left}> {name} </div>

                <Message message={last_message} unread_number={unread_number} current_user_id={current_user_id}></Message>
            </div>
        </div>
    )
}

export default Chat;