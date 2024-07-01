import React from 'react'
import classes from './GroupSettings.module.css'
import {GetImageURL} from "../scripts/url.js";
import {RemoveMember} from "../scripts/Members.js";
import RoughImg from "../UI/RoughImg.jsx";

function Member({image, name, chat_id, id, token}) {


    return (
        <div className={classes.Member}>
            <RoughImg src={GetImageURL(image)} alt={`${name}\`s image`} width={50} height={50}/>
            <div> {name} </div>
            <img className={classes.right} src={"/remove-user.svg"} alt={`delete user icon`}
                 width={40} height={40}
                 onClick={() => {
                     RemoveMember(chat_id, id, token).then(r => {
                         if (!r.ok){
                             alert("cant remove")
                         }
                     })

                 }}
            />
        </div>
    )
}

export default Member;