use std::io::{Write, stdin, stdout};

fn main() {
    print!("login: ");
    stdout().flush().unwrap();
    let mut login = String::new();
    stdin().read_line(&mut login).unwrap();
    login.pop(); // remove the trailing '\n'

    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    let mut auth = pam::Authenticator::with_password("myservice").unwrap();
    auth.get_handler().set_credentials(login, pass);
    if auth.authenticate().is_ok() && auth.open_session().is_ok() {
        println!("Successfully opened a session!");
    }
    else {
        println!("Authentication failed =/");
    }
}
