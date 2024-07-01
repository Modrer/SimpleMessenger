import React from 'react'
import classes from './Contacts.module.css'
import {GetImageURL} from "../scripts/url.js";
import {RemoveContact} from "../scripts/Contacts.js";
import RoughImg from "../UI/RoughImg.jsx";

function Contact({image, name, id, removeContact}) {


    return (
        <div className={classes.Contact}>
            <RoughImg  src={GetImageURL(image)} alt={`${name}\`s image`} width={50} height={50}/>
            <div> {name} </div>
            <img className={classes.right} src={"/remove-user.svg"} alt={`delete user icon`}
                 width={40} height={40}
                 onClick={() => {
                     RemoveContact(id).then((response) => {
                         if (response.ok){
                             removeContact(id)
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