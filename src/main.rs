#[tokio::main]
async fn main() {
    use authios_sdk::requests::LoggedUserGetInfoRequest as Request;
    use authios_sdk::responses::LoggedUserGetInfoResponse as Response;
    use authios_sdk::AuthiosClient;
    use serde_json::to_string_pretty;

    let token = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc2MTIzODUyMX0.FAPqP5p_06SbAjtYFwQa_rcUq6V5idSqRVU0DJ-xpvs"); 
    let client = AuthiosClient::new(String::from("http://localhost:3001/"))
        .unwrap();
    let client = std::sync::Arc::from(client);

    let result = client.query()
        .user()
        .logged(token)
        .get_info(Request { get_id: None, get_login: None, get_password_hash: None })
        .await;
    
    //println!("{}", to_string_pretty(&result).unwrap());
    match result {
        Response::Ok { user } => println!("{}", to_string_pretty(&user).unwrap()),
        _ => panic!()
    };
}
