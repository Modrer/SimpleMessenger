import React from 'react'
import classes from './GroupSettings.module.css'
import {useNavigate} from "react-router-dom";
import Member from "./Member.jsx";

function Members({members, chat_id, user}) {

    return (
        <div className={classes.Body}>
            {members.filter(member => member.user_id !== user.id).map((member) =>
                <Member key={member.user_id} image={member.image} name={member.name}
                    chat_id={chat_id} id={member.user_id} token={user?.token}
                ></Member>)
            }
        </div>
    )
}

export default Members;