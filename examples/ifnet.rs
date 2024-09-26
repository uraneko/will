// #[link(name = "ifaddrs")]
// #[link(name = "sys/types")]
// #[link(name = "sys/socket")]

extern "C" {
    fn getifaddrs(ifap: *mut ifaddrs) -> i32;
}

#[repr(C)]
#[derive(Debug)]
struct ifaddrs {
    ifa_next: *mut ifaddrs,
    ifa_name: [u8; 6],
    ifa_flags: u32,
    ifa_addr: sockaddr,
    ifa_netmask: sockaddr,
}

#[repr(C)]
#[derive(Debug)]
struct sockaddr {
    sa_family: sa_family_t,
    sa_data: [u8; 6],
}

type sa_family_t = u16;

fn main() {
    let mut ifa: ifaddrs = ifaddrs {
        ifa_next: std::ptr::null_mut(),
        ifa_name: [0; 6],
        ifa_flags: 0,
        ifa_addr: sockaddr {
            sa_family: 0,
            sa_data: [0; 6],
        },
        ifa_netmask: sockaddr {
            sa_family: 0,
            sa_data: [0; 6],
        },
    };

    unsafe {
        let res = getifaddrs(&mut ifa);
        println!("{}", res);
    }

    println!("{:?}", ifa);
    while !ifa.ifa_next.is_null() {
        unsafe {
            println!(
                "{:?}",
                (ifa.ifa_next).is_aligned() && !(ifa.ifa_next).is_null()
            );
            ifa = std::ptr::read(ifa.ifa_next);
            println!("{:?}", ifa);
        }
    }
}
