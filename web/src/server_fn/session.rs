use dioxus::prelude::*;


/// Echo the user input on the server.
#[server(SessionEcho)]
pub async fn session_echo(input: String) -> Result<String, ServerFnError> {
    use crate::server::session::UserSessionData;
    match UserSessionData::fetch().await? {
        Some(data) => {
            Ok(format!("{} typed: {}", data.username, input))
        }
        None => {
            Ok(format!("Anonimous typed: {}", input))
        }
    }
}


#[server(GetUserSession)]
pub async fn get_user_session() -> Result<Option<String>, ServerFnError> {
    use crate::server::session::UserSessionData;
    let data = UserSessionData::fetch().await?;
    println!("user session is: {:?}", data);
    Ok(data.map(|data| data.username))
}

#[server(SetUserSession)]
pub async fn set_user_session(username: String) -> Result<String, ServerFnError> {
    use crate::server::session::UserSessionData;
    let data = UserSessionData::new(username.clone());
    data.set().await?;
    println!("set user session: {:?}", data);
    Ok(username)
}

#[server(DeleteUserSession)]
pub async fn delete_user_session() -> Result<(), ServerFnError> {
    let session = crate::server::session::extract_session().await?;
    session.delete().await?;
    println!("session deleted");
    // use crate::server::session::UserSessionData;
    // match UserSessionData::fetch().await? {
    //     Some(data) => {
    //         let username = data.username.clone();
    //         data.delete().await?;
    //         Ok(Some(username))
    //     }
    //     None => Ok(None),
    // }
    Ok(())
}