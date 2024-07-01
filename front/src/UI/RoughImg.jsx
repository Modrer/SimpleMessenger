import React from 'react'
import classes from './UI.module.css'
const RoughImg = React.forwardRef((props, ref) => {
    return (
        <img ref={ref} {...props} className={classes.RoughImg}></img>
    )
});
export default RoughImg;
