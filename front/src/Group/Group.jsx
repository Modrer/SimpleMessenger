import React from 'react'
import classes from './Group.module.css'
import {GetImageURL} from "../scripts/url.js";
import BackArrow from "../UI/BackArrow.jsx";
import SearchIcon from "../UI/SearchIcon.jsx";
import {useNavigate} from "react-router-dom";
import Messages from "./Messages.jsx";
import MessageSender from "./MessageSender.jsx";
import RoughImg from "../UI/RoughImg.jsx";

function Group({chats, chat_id, user_id, setLastRead}) {
    const navigate = useNavigate()

    React.useEffect(() => {
        if(!chat_id){
            navigate("/")
        }
    },[chat_id])

    let chat = chats.findLast((chat) => chat.id === chat_id)
    console.log("chat", chat)
    console.log("lrm", setLastRead)

    return (
        <div className={classes.Body}>
            <div className={classes.Header} onClick={() => {
                navigate("/group_settings")
            }}>
                <BackArrow onClick={
                    (e) => {
                        e.stopPropagation()
                        navigate("/")
                    }
                }></BackArrow>
                <RoughImg src={GetImageURL(chat.image)} alt={`${chat.name}\`s image`}/>
                <div>
                    {chat ? chat.name : ""}
                </div>


            </div>

                <Messages
                messages={chat.messages}
                user_id={user_id}
                last_read_id={chat.last_read_message_id}
                setLastRead={(message_id) => {
                    setLastRead(chat_id, message_id)
                } }
            ></Messages>

            <MessageSender chatId={chat_id}></MessageSender>
        </div>
    )
}

export default Group;