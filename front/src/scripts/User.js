import {url as main_url} from "./url.js";
const url = `${main_url}/User`;

export function ChangeUser(new_name, new_image, token){
  const formData = new FormData();
  formData.append("name", new_name);

  if(new_image){
    formData.append("image", new_image);
  }


  return fetch(url, {
    method: "PATCH",
    headers: {

      "Authorization": `Bearer ${token}`
      // 'Content-Type': 'application/x-www-form-urlencoded',
    },
    // Set the FormData instance as the request body
    body: formData,
  })
}

export function GetUserById(id){

  const token = JSON.parse(window.localStorage.getItem("userInfo")).token;
    // let data = {
    //   login: nickname.current.value,
    //   password: password.current.value
    // }
    return fetch(`${url}/${id}`, {
      method: "GET", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
    });
}
export function SearchUser(name){

  let data = {
    Name:name
  };
  const token = JSON.parse(window.localStorage.getItem("userInfo")).token;
    // let data = {
    //   login: nickname.current.value,
    //   password: password.current.value
    // }
    return fetch(`${url}/${name}`, {
      method: "POST", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify(data), // body data type must match "Content-Type" header
    });
}