import {url as main_url} from "./url.js";

const url = `${main_url}/Chat`;
export function GetChats(){

  const token = JSON.parse(window.localStorage.getItem("userInfo"))?.token;

    return fetch(url, {
      method: "GET", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Content-Type": "application/json",
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      }
       // body data type must match "Content-Type" header
    });
}
export function MakeChat(name, image, token){
  const formData = new FormData();
  formData.append("name", name);
  formData.append("image", image);

    return fetch(url, {
      method: "PUT", // *GET, POST, PUT, DELETE, etc.
      headers: {
        "Authorization": `Bearer ${token}`
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: formData, // body data type must match "Content-Type" header
    });
}
export function UpdateChat(id, name, image,  token){
  const formData = new FormData();
  console.log(name)
  formData.append("id", id);
  formData.append("name", name);

  if(image){
    formData.append("image", image);
  }


  return fetch(url, {
    method: "PATCH", // *GET, POST, PUT, DELETE, etc.
    headers: {
      "Authorization": `Bearer ${token}`
      // 'Content-Type': 'application/x-www-form-urlencoded',
    },
    body: formData, // body data type must match "Content-Type" header
  });
}
