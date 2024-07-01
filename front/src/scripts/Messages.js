import {url as main_url} from "./url.js";
const url = `${main_url}/Messages`;
export function GetMessages(chat_id){
  // let data = {
  //   chat_id: chat_id
  // };
  const token = JSON.parse(window.localStorage.getItem("userInfo")).token;
    // let data = {
    //   login: nickname.current.value,
    //   password: password.current.value
    // }
    return fetch(`${url}/${chat_id}`, {
      method: "GET", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      //body: JSON.stringify(data), // body data type must match "Content-Type" header
    });
};
export function SendMessage(chat_id, message){

  let data = {
    chat_id: chat_id,
    message: message
  }

  const token = JSON.parse(window.localStorage.getItem("userInfo")).token;
    // let data = {
     // input.ChatId, input.Message
    //   ChatId: nickname.current.value,
    //   Message: password.current.value
    // }
    return fetch(url, {
      method: "POST", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify(data), // body data type must match "Content-Type" header
    });
};
