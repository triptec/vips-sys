extern crate pkg_config;

fn main() {
    std::env::set_var("PKG_CONFIG_PATH", "/var/task/lib/pkgconfig");
    pkg_config::Config::new().statik(false).probe("vips").unwrap();
    //pkg_config::probe_library("vips").unwrap();
}
