import React from 'react'
import classes from './UI.module.css'
const SearchIcon = React.forwardRef((props, ref) => {
    return (
        <img {...props} width={40} height={40} src={"/search1.svg"} alt={"arrow"}></img>
    )
});
export default SearchIcon;
