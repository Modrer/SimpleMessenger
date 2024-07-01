import React from 'react'

const MenuIcon = React.forwardRef((props, ref) => {
    return (
        <img {...props} width={30} height={30} src={"/menu.svg"} alt={"menu"}></img>
    )
});
export default MenuIcon;