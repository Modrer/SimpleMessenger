import React from 'react'
import classes from './Settings.module.css'
import {change_password} from "../scripts/Authorize.js";



function ChangePassword({user}) {

    const old_password = React.useRef()
    const password = React.useRef()
    const password_copy = React.useRef()

    function Change_Password(){

        let new_password = password.current?.value
        let new_password_copy = password_copy.current?.value

        let old_password_str = old_password.current?.value

        if (!new_password && new_password !== new_password_copy){
            console.log("wrong new password", new_password, new_password_copy)
            return;
        }

        if (!old_password_str){
            console.log("wrong old password")
            return;
        }

        change_password(old_password_str, new_password, user.token)
        .then((data) => {
                console.log(data.status)
            if(data.ok){
                alert("Password changed")
            }
            return undefined
        }

        )
    }

    return (
        <div className={classes.Body}>
            <label htmlFor={"old_password"}> Input old password</label>
            <input id={"old_password"} ref={old_password} type={"password"} name={"password"}/>
            <label htmlFor={"password"}> Select new password</label>
            <input id={"password"} ref={password} type={"password"} name={"password"}/>
            <label htmlFor={"password_copy"}> Repeat new password</label>
            <input id={"password_copy"} ref={password_copy} type={"password"} name={"password"}/>
            <button onClick={() => {
                Change_Password()

            }}> Change
            </button>
        </div>

    )
}

export default ChangePassword;