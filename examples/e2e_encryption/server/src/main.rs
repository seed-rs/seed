use shared::{
    decrypt, encrypt,
    opaque_ke::{
        ciphersuite::CipherSuite,
        keypair::{Key, KeyPair, SizedBytes},
        opaque::{ServerLogin, ServerRegistration},
    },
    rand_core::OsRng,
    DefaultCipherSuite,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::hyper::body::Bytes;
use warp::Filter;

#[allow(clippy::rc_buffer)]
#[tokio::main]
async fn main() {
    let mut rng = OsRng;
    let server_kp = DefaultCipherSuite::generate_random_keypair(&mut rng).expect("server keypair");

    let private_key = Arc::new(server_kp.private().clone());
    let private_key = warp::any().map(move || Arc::clone(&private_key));

    let public_key_bytes = Arc::new(server_kp.public().to_arr().to_vec());
    let public_key_bytes = warp::any().map(move || Arc::clone(&public_key_bytes));

    let rng = Arc::new(Mutex::new(rng));
    let rng = warp::any().map(move || Arc::clone(&rng));

    let registration_state = Arc::new(Mutex::new(None));
    let registration_state = warp::any().map(move || Arc::clone(&registration_state));

    let password_file_bytes = Arc::new(Mutex::new(None));
    let password_file_bytes = warp::any().map(move || Arc::clone(&password_file_bytes));

    let login_state = Arc::new(Mutex::new(None));
    let login_state = warp::any().map(move || Arc::clone(&login_state));

    let shared_secret = Arc::new(Mutex::new(None));
    let shared_secret = warp::any().map(move || Arc::clone(&shared_secret));

    // ------ Routes ------

    let status = warp::path!("api" / "status").map(|| "Server is running.");

    let api = warp::post().and(warp::path("api"));

    let public_key = api
        .and(warp::path("public-key"))
        .and(public_key_bytes)
        .map(|public_key_bytes: Arc<Vec<u8>>| (*public_key_bytes).clone());

    let registration_step_1 = api
        .and(warp::path!("registration" / "step-1"))
        .and(warp::body::bytes())
        .and(registration_state.clone())
        .and(rng.clone())
        .and_then(registration_step_1_handler);

    let registration_step_2 = api
        .and(warp::path!("registration" / "step-2"))
        .and(warp::body::bytes())
        .and(registration_state)
        .and(password_file_bytes.clone())
        .and_then(registration_step_2_handler);

    let login_step_1 = api
        .and(warp::path!("login" / "step-1"))
        .and(warp::body::bytes())
        .and(password_file_bytes)
        .and(rng)
        .and(private_key)
        .and(login_state.clone())
        .and_then(login_step_1_handler);

    let login_step_2 = api
        .and(warp::path!("login" / "step-2"))
        .and(warp::body::bytes())
        .and(login_state)
        .and(shared_secret.clone())
        .and_then(login_step_2_handler);

    let echo = api
        .and(warp::path("echo"))
        .and(warp::body::bytes())
        .and(shared_secret)
        .and_then(echo_handler);

    let files = warp::path("pkg").and(warp::fs::dir("./client/pkg/"));

    let index = warp::get().and(warp::fs::file("./client/index.html"));

    let routes = status
        .or(public_key)
        .or(registration_step_1)
        .or(registration_step_2)
        .or(login_step_1)
        .or(login_step_2)
        .or(echo)
        .or(files)
        .or(index);

    // ------ Start ------

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

async fn registration_step_1_handler(
    r1: Bytes,
    registration_state: Arc<Mutex<Option<ServerRegistration<DefaultCipherSuite>>>>,
    rng: Arc<Mutex<OsRng>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let (r2, state) = ServerRegistration::<DefaultCipherSuite>::start(
        r1.to_vec().as_slice().try_into().expect("r1"),
        &mut *rng.lock().await,
    )
    .expect("reg step 1");

    *registration_state.lock().await = Some(state);
    Ok(r2.to_bytes())
}

async fn registration_step_2_handler(
    r3: Bytes,
    registration_state: Arc<Mutex<Option<ServerRegistration<DefaultCipherSuite>>>>,
    password_file_bytes: Arc<Mutex<Option<Vec<u8>>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let password_file = (*registration_state.lock().await)
        .take()
        .expect("registration state")
        .finish(r3.to_vec().as_slice().try_into().expect("r3"))
        .expect("reg step 2");
    *password_file_bytes.lock().await = Some(password_file.to_bytes());
    Ok("Registration complete")
}

async fn login_step_1_handler(
    l1: Bytes,
    password_file_bytes: Arc<Mutex<Option<Vec<u8>>>>,
    rng: Arc<Mutex<OsRng>>,
    private_key: Arc<Key>,
    login_state: Arc<Mutex<Option<ServerLogin<DefaultCipherSuite>>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let password_file_bytes = &(*password_file_bytes.lock().await);
    let password_file = ServerRegistration::<DefaultCipherSuite>::try_from(
        password_file_bytes
            .as_ref()
            .expect("password_file_bytes")
            .as_slice(),
    )
    .expect("password file");

    let (l2, state) = ServerLogin::start(
        password_file,
        &private_key,
        l1.to_vec().as_slice().try_into().expect("l1"),
        &mut *rng.lock().await,
    )
    .expect("login step 1");

    *login_state.lock().await = Some(state);
    Ok(l2.to_bytes())
}

async fn login_step_2_handler(
    l3: Bytes,
    login_state: Arc<Mutex<Option<ServerLogin<DefaultCipherSuite>>>>,
    shared_secret: Arc<Mutex<Option<Vec<u8>>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let secret = (*login_state.lock().await)
        .take()
        .expect("login state")
        .finish(l3.to_vec().as_slice().try_into().expect("l3"))
        .expect("login step 2");
    println!("Shared secret: {:?}", secret);
    *shared_secret.lock().await = Some(secret);
    Ok("Login complete")
}

async fn echo_handler(
    message: Bytes,
    shared_secret: Arc<Mutex<Option<Vec<u8>>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let shared_secret_option = &(*shared_secret.lock().await);
    let shared_secret = shared_secret_option.as_ref().expect("shared secret");

    let message = decrypt(&message.to_vec(), shared_secret);
    let message = String::from_utf8(message).expect("message");

    println!("Message: {}", message);

    let message = encrypt(message.as_bytes(), shared_secret);
    Ok(message)
}
