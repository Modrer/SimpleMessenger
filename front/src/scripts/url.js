export const url = "https://simplemessenger.pp.ua/api";
export const webSocketUrl = "wss://simplemessenger.pp.ua/ws";
export function GetImageURL(image){
    return `${url}/images/${image}`
}