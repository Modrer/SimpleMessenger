import React from 'react'
import classes from './SearchUser.module.css'
import {GetImageURL} from "../scripts/url.js";
import AddUser from "../UI/AddUser.jsx";
import {AddContact} from "../scripts/Contacts.js";
import RoughImg from "../UI/RoughImg.jsx";

function User({onClick, image, name, isContact}) {


    return (
        <div className={classes.User}>
            <RoughImg src={GetImageURL(image)} alt={`${name}\`s image`} width={50} height={50}/>
            <div> {name} </div>
            {
                isContact? <></> : <AddUser className={classes.right}  onClick={onClick}></AddUser>
            }
        </div>
    )
}

export default User;