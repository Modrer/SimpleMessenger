import React from 'react'
import classes from './Group.module.css'



function MyMessage({message}) {

    return (
        <div className={classes.MyMessage}>
            <div>{message.message}</div>
        </div>
    )
}

export default MyMessage;