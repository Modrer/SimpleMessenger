import React from 'react'
import classes from './CreateGroup.module.css'
import {ChangeUser} from "../scripts/User.js";



function ChangeInfoForm({user, setCurrentUser}) {

    const name = React.useRef()
    const email = React.useRef()
    const image = React.useRef()

    function Change_Info(){

        let user_name = name.current?.value

        if (!user_name) {
            user_name = user.name
        }
        let user_email = email.current?.value

        if (!user_email) {
            user_email = user.email
        }

        let new_image = image.current?.files[0]

        ChangeUser(user_name, user_email, new_image, user.token).then((data) => {
            if(data.ok){
                return data.json()
            }
            return user
        }

        ).then((user) => {

            window.localStorage.setItem("userInfo", user)
            setCurrentUser(user)
        });
    }

    return (
        <div className={classes.Body}>
            <label htmlFor={"user_image"}> Select new image</label>
            <input id={"user_image"} ref={image} type={"file"} name={"image"}/>
            <label htmlFor={"user_email"}> Select new email</label>
            <input id={"user_email"} ref={email} type={"text"} name={"email"}/>
            <label htmlFor={"user_name"}> Select new name</label>
            <input id={"user_name"} ref={name} type={"text"} name={"name"}/>
            <button onClick={() => {
               Change_Info()

            }}> Change
            </button>
        </div>

    )
}

export default ChangeInfoForm;