use {
    std::{
        process::{Command, Stdio},
        ffi::OsStr,
        thread::{sleep, spawn},
        time::Duration,
        io::{self, BufRead, BufReader, Write},
    },
};

pub fn da_start() {
    cel_key_gen("cel-key-mocha".as_ref(), "mocha".as_ref());
}

fn cel_key_gen(key_name: &OsStr, network_name: &OsStr) {
    let executable_name = "./target/release/cel-key";
    let mut key_gen_cmd = Command::new(executable_name);

    // Set the arguments for celestia
    key_gen_cmd
        .arg("add").arg(key_name)
        .arg("--keyring-backend").arg("test")
        .arg("--node.type").arg("light")
        .arg("--p2p.network").arg(network_name);

    let output = key_gen_cmd.output().expect("Failed to execute command");
    io::stdout().write_all(&output.stdout).expect("Failed to write to stdout");
    io::stderr().write_all(&output.stderr).expect("Failed to write to stderr");

    if output.status.success() {
        println!();
    } else {
        eprintln!("Command failed with exit code: {:?}", output.status);
    }

    sleep(Duration::from_secs(3));
    light_node_init(key_name, network_name);
}


fn light_node_init(key_name: &OsStr, network_name: &OsStr) {
    let executable_name = "./target/release/celestia";
    let mut da_cmd = Command::new(executable_name);

    // Set the arguments for celestia
    da_cmd
        .arg("light").arg("init")
        .arg("--p2p.network").arg(network_name);

    let output = da_cmd.output().expect("Failed to execute command");

    let output_string = String::from_utf8_lossy(&output.stderr);

    let lines: Vec<&str> = output_string.lines().collect();

    let min_spaces = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|&c| c == ' ').count())
        .min()
        .unwrap_or(0);

    for line in lines {
        let trimmed_line = line.trim_start_matches('\t').trim_start_matches(' ');

        println!("{:indent$}{}", "", trimmed_line, indent = min_spaces);
    }

    light_node_auth(key_name, network_name);
}

fn light_node_auth(key_name: &OsStr, network_name: &OsStr) {
    let executable_name = "./target/release/celestia";
    let mut da_cmd = Command::new(executable_name);

    // Set the arguments for celestia
    da_cmd
        .arg("light")
        .arg("auth").arg("admin")
        .arg("--p2p.network").arg(network_name);

    let output = da_cmd.output().expect("Failed to execute command");


    let output_string = String::from_utf8_lossy(&output.stdout);

    println!("CELESTIA DA AUTH : {:?}", output_string);

    sleep(Duration::from_secs(3));
    light_node_run(key_name, network_name);
}


fn light_node_run(key_name: &OsStr, network_name: &OsStr) {
    let executable_name = "./target/release/celestia";
    let mut da_cmd = Command::new(executable_name);

    // Set the arguments for celestia
    da_cmd
        .arg("light").arg("start")
        .arg("--keyring.accname").arg(key_name)
        .arg("--core.ip").arg("rpc-mocha.pops.one")
        .arg("--p2p.network").arg(network_name)
        .stdout(Stdio::piped());

    println!("DA NODE RUN SUCCESSFULLY !!!!!!!");


    let output = da_cmd.output().expect("Failed to execute command");

    let output_string = String::from_utf8_lossy(&output.stderr);

    let lines: Vec<&str> = output_string.lines().collect();

    let min_spaces = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|&c| c == ' ').count())
        .min()
        .unwrap_or(0);

    for line in lines {
        let trimmed_line = line.trim_start_matches('\t').trim_start_matches(' ');

        println!("{:indent$}{}", "", trimmed_line, indent = min_spaces);
    }


    // let mut child = da_cmd.spawn().expect("Failed to execute command");

    // Wait for the child process to finish
    // let status = child.wait().expect("Failed to wait for command to finish");

    // if !status.success() {
    //     eprintln!("Command failed with exit code: {}", status);
    // }
}



