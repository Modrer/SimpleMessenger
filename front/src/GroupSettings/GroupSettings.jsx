import React from 'react'
import classes from './GroupSettings.module.css'
import {GetImageURL} from "../scripts/url.js";
import BackArrow from "../UI/BackArrow.jsx";
import SearchIcon from "../UI/SearchIcon.jsx";
import {useNavigate} from "react-router-dom";
import Messages from "./Messages.jsx";
import MessageSender from "./MessageSender.jsx";
import ChangeGroupForm from "./ChangeGroupForm.jsx";
import Members from "./Members.jsx";
import Candidats from "./Candidats.jsx";
import {RemoveMember} from "../scripts/Members.js";
import RoughImg from "../UI/RoughImg.jsx";

function GroupSettings({chats, chat_id, user, contacts}) {

    let chat;
    chat = chats.findLast((chat) => chat.id === chat_id);

    console.log("chat", chat)

    const navigate = useNavigate()

    return (
        <div className={classes.Body}>
            <div className={classes.Header}>
                <BackArrow onClick={
                    () => {
                        navigate("/group")
                    }
                }></BackArrow>
                <RoughImg src={GetImageURL(chat.image)} alt={`${chat.name}\`s image`} width={50} height={50}/>
                <div>
                    {chat ? chat.name : ""}
                </div>


            </div>
            <ChangeGroupForm id={chat.id} user={user} default_name={chat.name}></ChangeGroupForm>
            <Members members={chat.members} chat_id={chat.id} user={user}></Members>
            <div>Contact, that you can add to group</div>
            <Candidats
                chat_id={chat.id}
                user={user}
                users={
                contacts.filter((contact) =>
                    !chat.members?.findLast(member => member.user_id === contact.user_id))
            }></Candidats>
            <button onClick={() => {
                RemoveMember(chat_id, user.id, user.token).then(response => {
                    if(response.ok){
                        navigate("/")
                    }
                    else{
                        alert("Error when try to leave group")
                    }
                })

            }}>Leave group</button>
        </div>
    )
}

export default GroupSettings;