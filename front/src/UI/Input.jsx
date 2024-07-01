import React from 'react'
import classes from './UI.module.css'
const Input = React.forwardRef((props,ref) => {
  return (
    <input  {...props} className={classes.input} ref={ref}></input>
  )
});
export default Input;
