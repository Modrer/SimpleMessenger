import React from 'react'
import classes from './Group.module.css'
import {useNavigate} from "react-router-dom";
import MyMessage from "./MyMessage.jsx";
import Message from "./Message.jsx";
import user from "../SearchUser/User.jsx";

function Messages({messages, user_id, last_read_id, setLastRead}) {

    const listRef = React.useRef();
    function IsDown(e){

        const result =
            Math.abs(e.target.scrollHeight -
            Math.ceil(e.target.scrollTop) -
            e.target.clientHeight) <= 2;
        console.log(e.target.scrollHeight,Math.ceil(e.target.scrollTop), e.target.clientHeight);
        console.log("IsDown",result);
        return result;

    }
    function HandleScroll(e){
        let message = messages.findLast(() => {return true});
        console.log("handle scroll")
        if(!message){
            return;
        }
        console.log(message.id, last_read_id, IsDown(e))
        if(IsDown(e) && message.id > last_read_id){
            console.log('message',message.id);
            setLastRead(message.id);
        }
    }
    React.useEffect(()=>{
        let list = listRef.current;
        const result = list.scrollHeight - Math.ceil(list.scrollTop) === list.clientHeight;

        let message = messages?.at(-1)

        if(result && message?.id > last_read_id){
            setLastRead(message.id);
        }

    },[JSON.stringify(messages)])
    console.log(messages)
    return (
        <div ref={listRef} className={classes.MessagesList} onScroll={(e) => HandleScroll(e)}>
            {messages.map((message) => message.sender_id === user_id?
            <MyMessage key={message.id} message={message}></MyMessage>
                :
                <Message key={message.id} message={message} user_id={user_id}></Message>
            )}

        </div>
    )
}

export default Messages;