import React from 'react'
import classes from './Group.module.css'
import Group from "./Group.jsx";
import BackArrow from "../UI/BackArrow.jsx";
import SearchIcon from "../UI/SearchIcon.jsx";
import Input from "../UI/Input.jsx";
import {useNavigate} from "react-router-dom";

function Contacts({contacts}) {

    const [isSearch, setSearch] = React.useState(false)
    const [filter, setFilter] = React.useState("")
    const navigate = useNavigate();
    return (
        <div className={classes.Body}>
            <div className={classes.Header}>
                <BackArrow onClick={
                    () =>
                    { isSearch? setSearch(false): navigate("/")}
                }></BackArrow>
                { isSearch?
                    <input onChange={(event) => setFilter(event.target.value)}/>
                    :
                    <div>
                    Contacts
                </div>
                }

                { isSearch?  <></> : <SearchIcon onClick={() => setSearch(true)} ></SearchIcon>}
            </div>
            <button onClick={ () => navigate("/searchuser")}> Add contact</button>
            {contacts.filter(
                (contact) => contact.name.toLowerCase().includes(filter.toLowerCase()))
                .map((contact) =>
                    <Group key={contact.user_id} image={contact.image} name={contact.name}></Group>)
            }
        </div>
    )
}

export default Contacts;