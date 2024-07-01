import React from 'react'
import classes from './SearchUser.module.css'
import {GetImageURL} from "../scripts/url.js";
import {useNavigate} from "react-router-dom";
import BackArrow from "../UI/BackArrow.jsx";
import {SearchUser as search} from "../scripts/User.js"
import User from "./User.jsx";
import {AddContact} from "../scripts/Contacts.js";
function SearchUser({contacts, addContact, user_id}) {

    const navigate = useNavigate();
    const [userName, setUserName] = React.useState("")
    const [users, setUsers] = React.useState([])


    return (
        <div className={classes.Body}>
            <div className={classes.Header}>
            <BackArrow onClick={
                () =>
                navigate("/contacts")
            }></BackArrow>
            <input onChange={
                (event) => {
                    if (event.target.value === "") {
                        setUsers([])
                        return
                    }

                    search(event.target.value).then((response) => {
                        if (response.ok){
                            return response.json()
                        }
                    }).then((json) => {
                        if (!json){
                            setUsers([])
                            return
                        }
                        setUsers(json.filter(user => user.user_id !== user_id))
                        console.log("users",json, json.filter(user => user.user_id !== user_id),user_id)
                    })


                }}/>
            </div>
            {users.map((user) => <User
                key={user.user_id}
                image={user.image}
                name={user.name}
                isContact={contacts.findLast((contact) => contact.user_id === user.user_id)}
                onClick={
                    (e) => {
                        console.log(user.user_id);
                        AddContact(user.user_id).then((r) => {
                                if (r.ok) {
                                    addContact(user)
                                }
                            }
                        )
                    }
                }
            ></User>)}
        </div>
    )
}

export default SearchUser;