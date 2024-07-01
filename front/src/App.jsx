import React, { useState } from 'react'
import './App.css'
import {Route, useNavigate} from "react-router-dom";
import SignInUp from "./SignInUp/SignInUp.jsx";
import {Routes} from "react-router";
import Contacts from "./Contacts/Contacts.jsx";
import {GetContacts} from "./scripts/Contacts.js";
import SearchUser from "./SearchUser/SearchUser.jsx";
import useWebSocket from 'react-use-websocket';
import {GetChats} from "./scripts/Chats.js";
import Main from "./Main/Main.jsx";
import {GetUserById} from "./scripts/User.js";
import {GetMessages} from "./scripts/Messages.js";
import {GetMembers, SetReadMessage} from "./scripts/Members.js";
import Group from "./Group/Group.jsx";
import Menu from "./Menu/Menu.jsx";
import Settings from "./Setings/Settings.jsx";
import CreateGroup from "./CreateGroup/CreateGroup.jsx";
import GroupSettings from "./GroupSettings/GroupSettings.jsx";
import {webSocketUrl} from "./scripts/url.js";

function App() {
    const [count, setCount] = useState(0)
    const [contacts, setContacts] = useState([])
    const [activeChatId, setActiveChatId] = React.useState(-1);

    const [chats, setChats] = React.useState([]);

    const [current_user, setCurrentUser] = React.useState(
        {
            id: -1,
            name: "",
            email: "",
            image: "default.png",
            token: ""
        }
    )

    const navigate = useNavigate()

    React.useEffect(() => {
        console.log("use effect")
        let user;
        try {
            user = JSON.parse(window.localStorage.getItem("userInfo"))
        } catch (e) {
            // Oh well, but whatever...
            user = undefined
        }

        console.log("User",user);
        if(!user){
            console.log("navigate");

            navigate('/logIn/login');
        }

        setCurrentUser(user)

        GetContacts().then((data) => {
            if(data.ok){
                return data.json()
            }
            return []
        }).then((data) => {
            setContacts(data)
            console.log(data)
        })
        GetChats().then((data) => {
            if(data.ok){
                return data.json()
            }
            return []
        }).then((data) => {
            let new_data = data.map(async (item) => {
                let messages = GetMessages(item.id).then((messages) => messages.json())
                let members = GetMembers(item.id).then((members) => members.json())



                await Promise.all([messages, members])
                    .then(([messages, members]) => {
                        console.log("messages",messages)
                        item.messages =  messages;
                        item.members =  members;
                    })

                return item
            })

            Promise.all(new_data).then((data) => {
                setChats(data)
                console.log("data", data)
            })



        })
    },[])

    function RemoveContact(id){
        setContacts(contacts.filter((contact) => contact.user_id !==id))
    }
    function OnMessage(message_prew){
        console.log("onMessage", message_prew);

        let message = message_prew


        let newChats = chats;
        newChats = newChats.map((chat) => {
            //console.log(chat.id + "  " +message.chatId );

            if(chat.id === message.chat_id){
                console.log("pre",chat);
                chat.messages.push(message);
                console.log("post",chat);
            }
            return chat;
        })
        setChats(newChats);
    }

    function OnRemoveChat(id){
        console.log("OnRemoveChat", id);

        let newChats = chats;
        newChats = newChats.filter((chat) => {
            return chat.id !== id;
        });

        if(activeChatId === id){
            navigate("/")
        }

        setChats(newChats);
    }
    function OnAddChat(chat){
        console.log("OnAddChat", chat);
        console.log("todo OnAddChat", chat);
        let parsed_chat = chat;
        let messages = GetMessages(parsed_chat.id).then((messages) => messages.json())
        let members = GetMembers(parsed_chat.id).then((members) => members.json())

        Promise.all([messages, members])
            .then(([messages, members]) => {
                parsed_chat.messages =  messages;
                parsed_chat.members =  members;
                setChats([...chats, parsed_chat])
            })



    }
    async function OnAddMember(chatId, memberId){
        let user;
        console.log(chatId, memberId);
        await GetUserById(memberId).then((user) => {
            return user.json();
        }).then((data) => {
            user = data;
        })
        Promise.all(chats.map( async(chat) => {
            if(chat.id === chatId){

                chat.members.push(user);
            }
            return chat;

        })).then((newChats) => {
            // console.log("newChats",newChats);
            setChats(newChats);
        })


    }
    function OnRemoveMember(chatId, memberId){
        console.log("Remove", chatId,memberId);
        let newChats = chats.map( (chat) => {
            if(chat.id === chatId){
                chat.members = chat.members.filter((member) => {
                    return member.user_id !== memberId;
                })
            }
            return chat;

        })
        console.log("newChats",newChats);
        setChats(newChats);

    }

    function OnUpdateChat(updated_chat){
        setChats(chats.map(chat => {
            if(chat.id === updated_chat.id ){
                chat.name = updated_chat.name
                chat.image = updated_chat.image
            }
            return chat
        }))
    }
    function OnReceive(messageEvent){
        let data = JSON.parse(messageEvent.data)
        let type = data.message_type;

        console.log(data);
        console.log(data.message);
        let message = JSON.parse(data.message);
        switch(type){
            case 'message':
                console.log("before", data.message);
                OnMessage(message); break;
            case 'remove chat':
                OnRemoveChat(message);
                break;
            case 'add chat' :
                OnAddChat(message);
                break;
            case 'remove member' :
                OnRemoveMember(message.chat_id,message.user_id);
                break;
            case 'add member' :
                console.log(data);
                OnAddMember(message.chat_id,message.user_id);
                break;
            case 'update chat' :
                console.log(data);
                OnUpdateChat(message)
                break;
            default: break;
        }
    }

    React.useEffect(()=> {

    }, [current_user])

    const {
        sendMessage,
        sendJsonMessage,
        lastMessage,
        lastJsonMessage,
        readyState,
        getWebSocket,
    }  = useWebSocket(webSocketUrl, {
        shouldReconnect: (closeEvent) => true,


        queryParams : {
            claims : current_user?.token
        },

        onMessage : (messageEvent) => {
            OnReceive(messageEvent);
            console.log('This has type "message": ', JSON.parse(messageEvent.data));

        },
        onClose : (event) => {
            console.log('This close: ', event);
        },
        onError: (event) => {
            console.log('This error: ', event);
        },
        onOpen: (event) => {
            console.log("Connected");
        }


    });

    if(!current_user){
        return <Routes>
            <Route path='/logIn/*' element={<SignInUp></SignInUp>}></Route>
        </Routes>
    }


  return (
    <>
        <Routes>
            {/*<Route path='/' element={<MainPage></MainPage>}></Route>*/}
            <Route path='/logIn/*' element={<SignInUp></SignInUp>}></Route>
            <Route path='/contacts' element={<Contacts contacts={contacts} removeContact={RemoveContact}></Contacts>}></Route>
            <Route path='/' element={
                <Main chats={chats} setActiveChatId={setActiveChatId} current_user_id={current_user.id}></Main>
            }></Route>
            <Route path='/new_group' element={<CreateGroup user={current_user}></CreateGroup>}></Route>
            <Route path='/menu' element={
                current_user?
                <Menu user={current_user} setCurrentUser={setCurrentUser}></Menu>
                    :
                    <></>
            }></Route>

            <Route path='/group' element={
                chats.findLast((chat) => {
                    if(chat.id === activeChatId){
                        console.log(chat)
                        return true
                    }

                    return false
                })?
                <Group
                    chats={chats}
                    chat_id={activeChatId}
                    user_id={current_user?.id}
                    setLastRead={(chat_id, message_id) => {
                        SetReadMessage(chat_id, message_id).then(response => {
                            console.log(response.status)
                            if(response.ok){
                                let updated_chats = chats.map(chat => {
                                    if(chat.id === chat_id ){
                                        chat.last_read_message_id = message_id
                                    }
                                    return chat
                                })

                                setChats(updated_chats)

                            }
                        })
                    }}
                ></Group>
                :
                <>
                </>

            }></Route>
            <Route path='/group_settings' element={
                chats.findLast((chat) => {
                    if(chat.id === activeChatId){
                        console.log(chat)
                        return true
                    }
                    return false
                })?
                    <GroupSettings
                        chats={chats}
                        chat_id={activeChatId}
                        user={current_user}
                        contacts={contacts}
                    ></GroupSettings>
                    :
                    <></>
            }></Route>
            <Route path='/searchuser'
                   element={
                <SearchUser contacts={contacts}
                            addContact={(contact) => setContacts([contact,...contacts])}
                            user_id={current_user.id}
                ></SearchUser>
            }
            ></Route>
            <Route path='/settings'
                   element={
                       <Settings user={current_user} setCurrentUser={setCurrentUser} ></Settings>
                   }
            ></Route>
        </Routes>
    </>
  )
}

export default App
