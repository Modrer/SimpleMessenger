import React from 'react'
import classes from './Menu.module.css'
import {GetImageURL} from "../scripts/url.js";
import BackArrow from "../UI/BackArrow.jsx";
import {useNavigate} from "react-router-dom";
import RoughImg from "../UI/RoughImg.jsx";


function Menu({user, setCurrentUser}) {
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
                <div style={{textAlign: "left"}}>
                    <RoughImg src={GetImageURL(user.image)} alt={`${user.name}\`s image`} width={50} height={50}/>
                    <div>login: {user.name}</div>
                    <div>email: {user.email}</div>
                </div>


            </div>

            <button onClick={() => navigate("/settings")}>Settings</button>
            <button onClick={() => navigate("/contacts")}>Contacts</button>
            <button onClick={() => navigate("/new_group")}>New group</button>
            <button onClick={() => {
                window.localStorage.removeItem("userInfo")
                setCurrentUser(undefined)
                navigate("/logIn/login")
            }}>Log out</button>
        </div>

    )
}

export default Menu;