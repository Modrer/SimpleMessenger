import {url as main_url} from "./url.js";
const url = `${main_url}/Contact`;

export function GetContacts(){

  const token = JSON.parse(window.localStorage.getItem("userInfo"))?.token;
    return fetch(url, {
      method: "GET", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
    });
};
export function AddContact(contactId){

  const token = JSON.parse(window.localStorage.getItem("userInfo")).token;
    let data = {
      "id": contactId
    };
    return fetch(url, {
      method: "PUT", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify(data), // body data type must match "Content-Type" header
    });
};
export function RemoveContact(contactId){

  const token = JSON.parse(window.localStorage.getItem("userInfo")).token;
    let data = {
      "id": contactId
    };
    return fetch(url, {
      method: "DELETE", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify(data), // body data type must match "Content-Type" header
    });
};