import React from 'react'
import classes from './GroupSettings.module.css'
import {change_password} from "../scripts/Authorize.js";
import {MakeChat, UpdateChat} from "../scripts/Chats.js";



function ChangeGroupForm({user, id, default_name}) {

    const image = React.useRef()
    const name = React.useRef()

    function Update_Chat(){

        let new_name = name.current?.value

        if(!new_name){
            new_name = default_name
        }

        let new_image = image.current?.files[0]
        console.log(new_name)
        UpdateChat(id, new_name, new_image, user?.token).then((data) => {
            if(data.ok){
                return data.json()
            }
            alert("cant update group")
        })

    }

    return (
        <div className={classes.Body}>
            <label htmlFor={"chat_new_image"} className={classes.custom_file_upload}>
                <i className="fa fa-cloud-upload"></i>
                Choose new image</label>

            <input id={"chat_new_image"} ref={image} type={"file"} name={"image"}/>
            <label htmlFor={"chat_new_name"}> Input new group name</label>
            <input id={"chat_new_name"} ref={name} type={"text"} name={"name"}/>
            <button onClick={() => {
                Update_Chat()
            }}> Change
            </button>
        </div>

    )
}

export default ChangeGroupForm;