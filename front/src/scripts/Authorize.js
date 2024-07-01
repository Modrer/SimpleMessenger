import {url} from "./url.js";

const Authorization = `${url}/Authorization`;
export function Authorize(login,password ){

    let data = {
      login: login,
      password: password
    }
    return fetch(Authorization, {
      method: "POST", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify(data), // body data type must match "Content-Type" header
    });
};
export function Registration(email, name, password){

  let data = {
    "email": email,
    "name": name,
    "password": password
  }
  // let data = {
  //   login: nickname.current.value,
  //   password: password.current.value
  // }
  return fetch(Authorization, {
    method: "PUT", // *GET, POST, PUT, DELETE, etc.
    headers: {
      "Content-Type": "application/json",
      // 'Content-Type': 'application/x-www-form-urlencoded',

    },
    body: JSON.stringify(data), // body data type must match "Content-Type" header
  });
};
export function change_password(old_password, new_password, token){

  let data = {
    "old_password": old_password,
    "new_password": new_password,

  }
  // let data = {
  //   login: nickname.current.value,
  //   password: password.current.value
  // }
  return fetch(Authorization, {
    method: "PATCH", // *GET, POST, PUT, DELETE, etc.
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${token}`
      // 'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: JSON.stringify(data), // body data type must match "Content-Type" header
  });
};
