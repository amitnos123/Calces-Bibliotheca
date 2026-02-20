/* Account  */
// post /auth/account/create
// post /auth/account/reverify
// put /auth/account/delete
// post /auth/account/delete
// get /auth/account/
// post /auth/account/disable
// patch /auth/account/change/password
// patch /auth/account/change/email
// post /auth/account/verify/{code}
// post /auth/account/reset_password
// patch /auth/account/reset_password

/* Session */
// post /auth/session/login
// post /auth/session/logout
// get /auth/session/all
// delete /auth/session/all
// delete /auth/session/{id}
// patch /auth/session/{id}

/* Onboarding */
// get /onboard/hello
// post /onboard/complete

/* MFA */
// put /auth/mfa/ticket
// get /auth/mfa/
// post /auth/mfa/recovery
// patch /auth/mfa/recovery
// get /auth/mfa/methods
// put /auth/mfa/totp
// post /auth/mfa/totp
// delete /auth/mfa/totp