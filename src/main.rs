use std::io;
use std::env;
use std::path;
use std::error;
use git2;

fn check_ip() -> Result<(), Box<dyn error::Error>> {
    let remote_ip = env::var("REMOTE_ADDR")?;
    let whitelist_ip = env::var("WHITELIST_IP")?;
    if remote_ip == whitelist_ip {
        Ok(())
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            format!("Blocked connection from {}", remote_ip),
        )))
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    match check_ip() {
        Ok(()) => {
            let mut repo_name = String::new();
            io::stdin().read_line(&mut repo_name)?;
            // remove trailing newline
            repo_name.pop();

            let mut opts = git2::RepositoryInitOptions::new();
            git2::RepositoryInitOptions::bare(&mut opts, true)
                .mode(git2::RepositoryInitMode::SHARED_GROUP)
                .no_reinit(true)
                .template_path(path::Path::new("./templates"));

            let repo = git2::Repository::init_opts(format!("{}.git", repo_name), &opts)?;

            let public_user = env::var("USER").unwrap();
            let public_name = env::var("PUBLIC").unwrap();
            let public_path = env::var("PATH").unwrap();
            repo.remote(
                &public_name, 
                &format!(
                    "{user}@{public}:{path}/{repo}.git", 
                    user = public_user, 
                    public = public_name, 
                    path = public_path, 
                    repo = repo_name
                    )
                )?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}
