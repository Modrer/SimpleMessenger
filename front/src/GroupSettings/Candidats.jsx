import React from 'react'
import classes from './GroupSettings.module.css'
import Candidat from "./Candidat.jsx";

function Candidats({users, chat_id, user}) {
    return (
        <div className={classes.Body}>
            {users.filter(cd => cd.user_id !== user.id).map((member) =>
                <Candidat key={member.user_id} image={member.image} name={member.name}
                    chat_id={chat_id} id={member.user_id} token={user?.token}
                ></Candidat>)
            }
        </div>
    )
}

export default Candidats;