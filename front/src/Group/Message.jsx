import React from 'react'
import classes from './Group.module.css'
import {GetImageURL} from "../scripts/url.js";
import {GetUserById} from "../scripts/User.js";
import RoughImg from "../UI/RoughImg.jsx";


function Message({message, user_id}) {
    const [sender, setSender] = React.useState({
        image: "default.png",
        name: ""
    })

    React.useEffect(() => {
        GetUserById(message.sender_id).then((resp) => resp.json()).then((data) => setSender(data))
    }, [message])

    return (
        <div className={classes.FullMessage}>
            <RoughImg src={GetImageURL(sender.image)} alt={`${sender.name}\`s image`}/>
            <div className={classes.Message}>
                <div> {sender.name} </div>
                <div>{message.message}</div>
            </div>

        </div>
    )
}

export default Message;