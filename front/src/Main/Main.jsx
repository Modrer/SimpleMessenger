import React from 'react'
import classes from './Main.module.css'
import MenuIcon from "../UI/MenuIcon.jsx";
import {useNavigate} from "react-router-dom";
import Chat from "./Chat.jsx";
import Chats from "./Chats.jsx";

function Main({chats, setActiveChatId, current_user_id}) {
    const navigate = useNavigate()
    console.log(chats)
    return (
        <div className={classes.Body}>
            <div className={classes.Header}>
                <MenuIcon onClick={() =>{ navigate("/menu") }}></MenuIcon>
                <div className={classes.Name}

                >Simple Messenger</div>
            </div>
            <Chats chats={chats} setActiveChatId={setActiveChatId} current_user_id={current_user_id}></Chats>

        </div>
    )
}

export default Main;