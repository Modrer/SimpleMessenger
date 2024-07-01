import React from 'react'
import classes from './UI.module.css'
const AddUser = React.forwardRef((props,ref) => {
    return (
        <img {...props} width={30} height={30} src={"/add-user.svg"} alt={"add-user"}></img>
    )
});
export default AddUser;
