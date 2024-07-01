import React from 'react'
import Member from './Member'
import classes from './Member.module.css'

export default function MemberList({members, chat, userId, refresh}) {
  function IsOwner(ownerId, memberId){
    return ownerId === memberId;
  }
  return (
    <div className={classes.MemberList}>
      {
        members.filter((member) => member.id !== userId).map((member) => {
          return <Member key={member.id} 
          member={member} 
          chatId={chat.id} 
          
          refresh={refresh}
          isOwner={IsOwner(chat.ownerId, member.id)} ></Member>
        })
      }
    </div>
  )
}
