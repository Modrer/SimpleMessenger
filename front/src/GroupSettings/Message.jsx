import React from 'react'
import classes from './GroupSettings.module.css'
import {GetImageURL} from "../scripts/url.js";
import {GetUserById} from "../scripts/User.js";


function Message({message, user_id}) {
    const [sender, setSender] = React.useState({
        image: "default.png",
        name: "unknown_user"
    })



    React.useEffect(() => {
        GetUserById(message.sender_id).then((resp) => resp.json()).then((data) => setSender(data))
    })

    return (
        <div className={classes.Message}>
            <img src={GetImageURL(sender.image)} alt={`${sender.name}\`s image`} width={50} height={50}/>
            <div> {sender.name} </div>
            <div>{message.message}</div>
        </div>
    )
}

export default Message;