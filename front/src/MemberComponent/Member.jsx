import React from 'react'
import { RemoveMember } from '../scripts/Members.js'
import classes from './Member.module.css'

export default function Member({member, chatId,isOwner, refresh }) {
  return (
    <div className={classes.Member}> 
      <div>{member.name}</div>
      {
        isOwner ?
        <button onClick={ () => {
            RemoveMember(chatId, member.id).then(r => {}); refresh()}
        }> Remove</button>
        :
        null
      }
    </div>
  )
}
