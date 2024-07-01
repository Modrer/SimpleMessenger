import React from 'react'
import classes from './CreateGroup.module.css'
import {GetImageURL} from "../scripts/url.js";
import BackArrow from "../UI/BackArrow.jsx";
import {useNavigate} from "react-router-dom";
import ChangeInfoForm from "./ChangeInfoForm.jsx";
import CreateForm from "./Create form.jsx";


function CreateGroup({user, AddChat}) {

    const navigate = useNavigate()

    return (
        <div className={classes.Body}>
            <div className={classes.Header}>
                <div className={classes.left}>
                    <BackArrow onClick={() => navigate('/menu')}></BackArrow>
                </div>
            </div>

            <CreateForm user={user}></CreateForm>

        </div>

    )
}

export default CreateGroup;