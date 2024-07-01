import React from 'react'
import classes from './SignInUp.module.css'
import Input from '../UI/Input'
import { Link, useNavigate } from 'react-router-dom'
import { Authorize, Registration } from '../scripts/Authorize';

export default function SignUp() {
  const name = React.useRef();
  const email = React.useRef();
  const password = React.useRef();

  const [isError, SetIsError] = React.useState(false);

  const navigate = useNavigate();

  function Register(e){
    e.preventDefault();
    let regInfo = {
      name: name.current.value,
      email: email.current.value,
      password: password.current.value,
    }
    Registration(regInfo.email, regInfo.name, regInfo.password).then(resp => {
      
      if(resp.status === 200){
        
        Authorize(regInfo.name, regInfo.password).then(resp => {
          if(resp.status === 200){
        
            resp.text().then((info) => {
              window.localStorage.setItem("userInfo", info);
              window.location = '/';
            });
           
          }
          else{
            alert("Something goes wrong");
          }
        })
      }
      else{
        alert("Can`t registrate user");
      }

      // navigate('/Unaftorize');
    });
  }

  return (
    <form className={classes.register_form}>
      <Input ref={name} type="text" placeholder="name"></Input>
      <Input ref={password} type="password" placeholder="password"></Input>
      <Input ref={email} type="text" placeholder="email address"></Input>
      {
        isError ?
        <div className={classes.failedText}>
          User with this name or email already exist
        </div> 
        :
        null
      }
      <button onClick={Register}>create</button>
      <p className={classes.message}>Already registered? <Link to='/logIn/login'>Sign In</Link></p>
    </form>
  )
}
