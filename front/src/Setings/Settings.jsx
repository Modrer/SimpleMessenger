import React from 'react'
import classes from './Settings.module.css'
import {GetImageURL} from "../scripts/url.js";
import BackArrow from "../UI/BackArrow.jsx";
import {useNavigate} from "react-router-dom";
import ChangeInfoForm from "./ChangeInfoForm.jsx";
import ChangePassword from "./ChangePassword.jsx";
import RoughImg from "../UI/RoughImg.jsx";


function Settings({user, setCurrentUser}) {

    const navigate = useNavigate()

    React.useEffect(() =>{
        if(!user){
            navigate("/logIn/login")
        }
    },[user])



    return (
        <div className={classes.Body}>
            <div className={classes.Header}>
                <div className={classes.left}>
                    <BackArrow onClick={() => navigate('/')}></BackArrow>
                </div>
                <div className={classes.Name}>Settings</div>
            </div>

            <div style={{textAlign: "left"}}>
                <RoughImg src={GetImageURL(user.image)} alt={`${user.name}\`s image`} width={50} height={50}/>
                <div>login: {user.name}</div>
                <div>email: {user.email}</div>
            </div>

            <div> Change user info</div>
            <ChangeInfoForm setCurrentUser={setCurrentUser} user={user}></ChangeInfoForm>
            <div> Change password</div>
            <ChangePassword user={user}></ChangePassword>

        </div>

    )
}

export default Settings;