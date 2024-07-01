import React from 'react'
import classes from './Main.module.css'
import {GetImageURL} from "../scripts/url.js";
import {GetUserById} from "../scripts/User.js";

function Message({message, unread_number, current_user_id}) {


    let message_text = message? message.message : ""
    let [sender_name, setName] = React.useState("");

    React.useEffect(() => {
        console.log(message)
        if(message){
            if(current_user_id === message?.sender_id){
                setName("You")
            }
            else{
                GetUserById(message?.sender_id).then(data => {
                    if(data.ok){
                        return data.json()
                    }
                    return {
                        user_id: -1,
                        name: "",
                        image: "default.png",
                    }
                }).then(data => {
                    console.log("data", data)
                    setName(data.name)
                })
            }

        }
    },[message])



    return (
        <div className={classes.low_text}>
            <div className={classes.preview_part}>
                {
                    sender_name ? <div className={classes.bold_text}>{`${sender_name}: `}</div>
                        :
                        <></>
                }
                <div> {message_text.substring(0,10)} </div>
            </div>
            <div className={classes.right}> {unread_number} </div>
        </div>
    )
}

export default Message;