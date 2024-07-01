import React, { useRef } from 'react'
import classes from './SignInUp.module.css'
import Input from '../UI/Input';
import { Link, useNavigate } from 'react-router-dom';
import { Authorize } from '../scripts/Authorize';
function LogIn() {

  const nickname = useRef("");
  const password = useRef("");
  const navigate = useNavigate();

  const [isError, SetIsError] = React.useState(false);

  const Authorization = async (e) =>{
    e.preventDefault();
    let data = {
      login: nickname.current.value,
      password: password.current.value
    }
    await Authorize(data.login, data.password).then(resp => {
      //console.log(resp.status);
      if(resp.status === 200){
        
        resp.text().then((info) => {
          window.localStorage.setItem("userInfo", info);
          window.location = '/';
        });
       
      }
      else{
        SetIsError(true);
      }
      // navigate('/Unaftorize');
    })
    

  }

  return (
    <form className={classes.login_form}>
      <Input type="text" ref={nickname} placeholder="You'r login" autoComplete="name"></Input>
      <Input type="password" ref={password} placeholder="You'r password" autoComplete="current-password"></Input>
      {
        isError ?
        <div className={classes.failedText}>
          Password or login are wrong
        </div> 
        :
        null
      }
      <button onClick={Authorization}>login</button>
      <p className={classes.message}>Not registered? <Link to='/logIn/registrate'>Create an account</Link></p>
    </form>
  )
};

export default LogIn;