import React from 'react'
import classes from './GroupSettings.module.css'
import { SendMessage } from '../scripts/Messages.js';

export default function MessageSender({chatId, update}) {
  const message = React.useRef();

  function Send(){
    if(!chatId){
      return;
    }
    if(!message.current?.value){
        return;
    }
    SendMessage(chatId, message.current.value).then((data) => {
      //update();
    })
    message.current.value = "";
  }
  return (
    <div className={classes.Sender}>
      <input ref={message} placeholder='Write you`r message' 
      onKeyUp={(e) => {
        if(e.key === "Enter"){
          Send();
          message.current.value = "";
        }
        
      }
      } >
      </input>
        <img src={"/send.svg"} alt={"send"} onClick={Send} width={20} height={20}/>
      {/*<button onClick={Send}  type="button">*/}
      {/*  Send*/}
      {/*</button>*/}
    </div>
  )
}
