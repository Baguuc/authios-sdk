#[tokio::main]
async fn main() {
    use authios_sdk::LoggedUserCheckServicePermissionRequest as Request;

    let token = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjIsImV4cCI6MTc2MTIzODUyMX0.FAPqP5p_06SbAjtYFwQa_rcUq6V5idSqRVU0DJ-xpvs"); 
    
    let result = authios_sdk::QueryBuilder::new(reqwest::Url::parse("http://localhost:3001/").unwrap())
        .user()
        .logged(token)
        .permissions()
        .service()
        .check(Request {
            service_id: String::from("taskios")
        })
        .await;
    
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
