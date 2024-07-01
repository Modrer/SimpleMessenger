import React from 'react'
import classes from './GroupSettings.module.css'
import {GetImageURL} from "../scripts/url.js";
import {RemoveContact} from "../scripts/Contacts.js";
import {AddMember} from "../scripts/Members.js";
import RoughImg from "../UI/RoughImg.jsx";

function Contact({image, name, chat_id, id, token}) {


    return (
        <div className={classes.Member}>
            <RoughImg src={GetImageURL(image)} alt={`${name}\`s image`} width={50} height={50}/>
            <div> {name} </div>
            <img className={classes.right} src={"/add-user.svg"} alt={`delete user icon`}
                 width={40} height={40}
                 onClick={() => {
                     AddMember(chat_id, id, token).then((response) => {
                         if (response.ok){

                             return
                         }
                         alert("Error")
                     })

                 }}
            />
        </div>
    )
}

export default Contact;