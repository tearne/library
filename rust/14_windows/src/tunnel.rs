use tokio::runtime::Runtime;
use ssh_jumper::SshJumper;
use ssh_jumper::model::{HostAddress, HostSocketParams, JumpHostAuthParams, SshTunnelParams};
use std::borrow::Cow;
use std::cell::RefCell;

pub struct Tunnel {
    rt: RefCell<Runtime>,
}

impl Tunnel {
    pub fn start(&self) {
        let  (local_socket_addr, ssh_forwarder_end_rx) = self.rt.borrow_mut().block_on(async{
            // ssh -i ~/.ssh/id_rsa -L 1234:target_host:8080 my_user@bastion.com
            let jump_host = HostAddress::HostName(Cow::Borrowed("bastion.com"));
            let jump_host_auth_params = JumpHostAuthParams::password(
                Cow::Borrowed("my_user"),
                Cow::Borrowed("my_password"),
            );
            let target_socket = HostSocketParams {
                address: HostAddress::HostName(Cow::Borrowed("target_host")),
                port: 8080,
            };
            let ssh_params =
                SshTunnelParams::new(jump_host, jump_host_auth_params, target_socket)
                    // Optional: OS will allocate a port if this is left out
                    .with_local_port(1234);

            println!("About to open with params {:?}", ssh_params);
            SshJumper::open_tunnel(&ssh_params).await
        }).unwrap();

        println!("{} -> {:?}", local_socket_addr, ssh_forwarder_end_rx)
    }
}

impl Default for Tunnel {
    fn default() -> Self {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        Tunnel{ rt: RefCell::new(rt) }
    }
}
