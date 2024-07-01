import React from 'react'
import classes from './Contacts.module.css'
import Contact from "./Contact.jsx";
import BackArrow from "../UI/BackArrow.jsx";
import SearchIcon from "../UI/SearchIcon.jsx";
import Input from "../UI/Input.jsx";
import Popup from 'reactjs-popup';
import {useNavigate} from "react-router-dom";
import 'reactjs-popup/dist/index.css';

function Contacts({contacts, removeContact}) {

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
            <button style={{marginBottom: "5px"}} onClick={ () => navigate("/searchuser")}> Add contact</button>
            {contacts.filter(
                (contact) => contact.name.toLowerCase().includes(filter.toLowerCase()))
                .map((contact) =>
                    <Contact key={contact.user_id} image={contact.image} name={contact.name}
                    removeContact={removeContact}
                    id={contact.user_id}></Contact>)
            }
        </div>
    )
}

export default Contacts;