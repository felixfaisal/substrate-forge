#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime_interface::runtime_interface;

#[runtime_interface]
pub trait LogExt {
    fn log_to_cli()  {
        println!("Logging to CLI");
    }
}