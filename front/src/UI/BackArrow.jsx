import React from 'react'
import classes from './UI.module.css'
const BackArrow = React.forwardRef((props,ref) => {
    return (
        <img {...props} width={40} height={40} src={"/arrow-left.svg"} alt={"arrow"}></img>
    )
});
export default BackArrow;
