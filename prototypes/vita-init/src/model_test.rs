//! Fixed, opt-in QEMU-only setup. No Agent-controlled paths or network arguments.
use std::{fs, io, net::{IpAddr, Ipv4Addr}, time::Duration};
use futures_util::TryStreamExt;
use nix::mount::{mount, MsFlags};
use rtnetlink::{new_connection, LinkUnspec, RouteMessageBuilder};

pub fn prepare() -> io::Result<()> {
    // Caller has already required root PID1 and the compiled feature + boot token.
    // Check the one known VM NIC rather than scanning or configuring arbitrary devices.
    if fs::read_to_string("/sys/class/net/eth0/address")?.trim() != "52:54:00:76:69:74" {
        return Err(io::Error::other("model-test requires the fixed QEMU NIC identity"));
    }
    fs::create_dir_all("/run/vita-model")?;
    mount(Some("vita-model"), "/run/vita-model", Some("virtiofs"),
        MsFlags::MS_RDONLY | MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOEXEC,
        None::<&str>).map_err(io::Error::other)?;
    if !super::has_readonly_mount(&fs::read_to_string("/proc/self/mountinfo")?, "/run/vita-model") {
        return Err(io::Error::other("model configuration share is not readonly"));
    }
    match fs::OpenOptions::new().create_new(true).write(true).open("/run/vita-model/.write-probe") {
        Err(e) if e.raw_os_error() == Some(nix::errno::Errno::EROFS as i32) => {},
        _ => return Err(io::Error::other("configuration share must reject writes with EROFS")),
    }
    // The immutable image contains a symlink from /etc/resolv.conf to this tmpfs file.
    fs::write("/run/vita-resolv.conf", "nameserver 10.0.2.3\noptions timeout:2 attempts:2\n")?;
    tokio::runtime::Builder::new_current_thread().enable_all().build()?.block_on(async {
        tokio::time::timeout(Duration::from_secs(10), configure_network()).await
            .map_err(|_| io::Error::other("guest network setup timeout"))?
    })?;
    super::serial("VITA_I010_CONFIG_READONLY_OK");
    super::serial("VITA_I010_GUEST_NETWORK_READY");
    Ok(())
}

async fn configure_network() -> io::Result<()> {
    let (connection, handle, _) = new_connection().map_err(io::Error::other)?;
    let task = tokio::spawn(connection);
    let result = async {
        for name in ["lo", "eth0"] {
            let mut links = handle.link().get().match_name(name.to_owned()).execute();
            let link = links.try_next().await.map_err(io::Error::other)?
                .ok_or_else(|| io::Error::other("fixed guest interface missing"))?;
            let index = link.header.index;
            handle.link().set(LinkUnspec::new_with_index(index).up().build())
                .execute().await.map_err(io::Error::other)?;
            if name == "eth0" {
                handle.address().add(index, IpAddr::V4(Ipv4Addr::new(10,0,2,15)), 24)
                    .execute().await.map_err(io::Error::other)?;
                let route = RouteMessageBuilder::<Ipv4Addr>::new()
                    .destination_prefix(Ipv4Addr::UNSPECIFIED, 0)
                    .gateway(Ipv4Addr::new(10,0,2,2)).output_interface(index).build();
                handle.route().add(route).execute().await.map_err(io::Error::other)?;
            }
        }
        Ok(())
    }.await;
    task.abort();
    result
}
