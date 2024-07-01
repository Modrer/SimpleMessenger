import React from 'react'
import classes from './Settings.module.css'
import {ChangeUser, SearchUser} from "../scripts/User.js";



function ChangeInfoForm({user, setCurrentUser}) {

    const name = React.useRef()
    const image = React.useRef()

    function Change_Info(){

        let user_name = name.current?.value

        if (!user_name) {
            user_name = user.name
        }

        let new_image = image.current?.files[0]

        ChangeUser(user_name, new_image, user.token).then((data) => {
            if(data.ok){
                return data.json()
            }
            return user
        }

        ).then((user) => {
            console.log("user", user)
            window.localStorage.setItem("userInfo", JSON.stringify(user))
            setCurrentUser(user)
        });
    }

    const [showNameError, setShowNameError] = React.useState(false)
    //const [showNameError, setShowNameError] = React.useState(false)

    return (
        <div className={classes.Body}>
            <label htmlFor={"user_image"} className={classes.custom_file_upload}>
                <i className="fa fa-cloud-upload"></i>
                Choose new image</label>
            <input id={"user_image"} ref={image} type={"file"} name={"image"}/>
            <label htmlFor={"user_name"}> Select new name</label>
            <input id={"user_name"} ref={name} onClick={() => {
                let new_name = name.current?.value;
                if(new_name || new_name !== "") {
                    SearchUser(new_name).then(data => {
                        if(data.ok){
                            return data.json()
                        }
                        return []

                    }).then(data => {
                        if(data.findLast((user) => {
                                console.log(
                                    user.name.toLocaleLowerCase(),
                                    new_name.toLocaleLowerCase(),
                                    user.name.toLocaleLowerCase() === new_name.toLocaleLowerCase()
                                )
                                return user.name.toLocaleLowerCase() === new_name.toLocaleLowerCase()
                            }
                        )){
                            setShowNameError(true)
                        }
                        else {
                            setShowNameError(false)
                        }
                    })
                }
            }} type={"text"} name={"name"}/>
            {
                showNameError? <div>this name is used</div> : <></>
            }
            <button onClick={() => {
                Change_Info()

            }}> Change
            </button>
        </div>

    )
}

export default ChangeInfoForm;