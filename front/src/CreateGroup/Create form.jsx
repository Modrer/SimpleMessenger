import React from 'react'
import classes from './CreateGroup.module.css'
import {change_password} from "../scripts/Authorize.js";
import {MakeChat} from "../scripts/Chats.js";
import {useNavigate} from "react-router-dom";



function CreateForm({user}) {

    const image = React.useRef()
    const name = React.useRef()

    const navigate = useNavigate()

    function Create_chat(){

        let new_name = name.current?.value
        let new_image = image.current?.files[0]

        MakeChat(new_name, new_image, user?.token).then((data) => {
            if(data.ok){
                navigate("/")
                return data.json()
            }
            alert("cant create group")
        })

    }

    return (
        <div className={classes.Body}>
            <label htmlFor={"chat_image"}> Select group image</label>
            <input id={"chat_image"} ref={image} type={"file"} name={"image"}/>
            <label htmlFor={"chat_name"}> Input group name</label>
            <input id={"chat_name"} ref={name} type={"text"} name={"name"}/>
            <button onClick={() => {
                Create_chat()
            }}> Create
            </button>
        </div>

    )
}

export default CreateForm;