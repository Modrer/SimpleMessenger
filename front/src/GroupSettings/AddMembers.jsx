import React from 'react'
import classes from './GroupSettings.module.css'
import {useNavigate} from "react-router-dom";
import Member from "./Member.jsx";

function AddMembers({members, chat_id}) {

    return (
        <div className={classes.Body}>
            {members.map((member) =>
                <Member key={member.user_id} image={member.image} name={member.name}
                    chat_id={chat_id} id={member.user_id}
                ></Member>)
            }
        </div>
    )
}

export default AddMembers;