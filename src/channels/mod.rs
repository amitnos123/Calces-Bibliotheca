/* Channel Information  */
// get /channels/{target}
// delete /channels/{target}
// patch /channels/{target}

/* Channel Invites */
// post /channels/{target}/invites

/* Channel Permissions */ 
// put /channels/{target}/permissions/{role_id}
// put /channels/{target}/permissions/default

/* Messaging */
// put /channels/{target}/ack/{message}
// get /channels/{target}/messages
// post /channels/{target}/messages
// post /channels/{target}/search
// post /channels/{target}/messages/{msg}/pin
// delete /channels/{target}/messages/{msg}/pin
// get /channels/{target}/messages/{msg}
// delete /channels/{target}/messages/{msg}
// patch /channels/{target}/messages/{msg}
// delete /channels/{target}/messages/bulk

/* Interactions */
// put /channels/{target}/messages/{msg}/reactions/{emoji}
// delete /channels/{target}/messages/{msg}/reactions/{emoji}
// delete /channels/{target}/messages/{msg}/reactions

/* Groups */
// get /channels/{target}/members
// post /channels/create
// put /channels/{group_id}/recipients/{member_id}
// delete /channels/{target}/recipients/{member}

/* Voice */
// post /channels/{target}/join_call
// put /channels/{target}/end_ring/{target_user}

/* Webhooks */
// post /channels/{target}/webhooks
// get /channels/{channel_id}/webhooks
// get /webhooks/{webhook_id}/{token}
// post /webhooks/{webhook_id}/{token}
// delete /webhooks/{webhook_id}/{token}
// patch /webhooks/{webhook_id}/{token}
// get /webhooks/{webhook_id}
// delete /webhooks/{webhook_id}
// patch /webhooks/{webhook_id}
// post /webhooks/{webhook_id}/{token}/github