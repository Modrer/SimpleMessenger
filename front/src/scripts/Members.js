import {url as main_url} from "./url.js";

const url = `${main_url}/Members`;

export function GetMembers(chat_id){

  const token = JSON.parse(window.localStorage.getItem("userInfo")).token;
    // let data = {
    //   chat_id: chat_id
    // };
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
export function RemoveMember(chatId, memberId, token){

    return fetch(`${url}/${chatId}/${memberId}`, {
      method: "DELETE", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },

    });
}
export function AddMember(chatId, memberId, token){

    return fetch(`${url}/${chatId}/${memberId}`, {
      method: "PUT", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
    });
}

export function SetReadMessage(chatId, messageId){

  const token = JSON.parse(window.localStorage.getItem("userInfo")).token;

    return fetch(`${url}/${chatId}/${messageId}`, {
      method: "POST", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
    });
};