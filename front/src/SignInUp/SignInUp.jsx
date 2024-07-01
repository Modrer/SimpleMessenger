import React, { useRef } from 'react'
import LogIn from './LogIn'
import SignUp from './SignUp'
import classes from './SignInUp.module.css'
import { Route, Routes } from 'react-router';

export default function SignInUp() {
  const formRef = useRef();
  // useEffect(() => {
  //   [...document.getElementsByTagName('a')].forEach(elem => {
  //     elem.addEventListener('click', () => {
  //       formRef.current.animate({height: "toggle", opacity: "toggle"}, "slow")
  //     })
  //   });
  // });
  React.useEffect(() =>{
  
  },[])
  

  return (
    <div ref={formRef} className={classes.login_page}>
      <div className={classes.form}>
        <Routes>
          <Route path='/login' element={<LogIn></LogIn>}></Route>
          <Route path='/registrate' element={<SignUp></SignUp>}></Route>
        </Routes>
      </div>
    </div>
  )
}
