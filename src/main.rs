use anyhow::{anyhow, Result};
use colored::*;
use probe_rs::{
    architecture::arm::{
        ap::{get_ap_by_idr, APClass, APType, IDR},
        ArmCommunicationInterface, ArmCommunicationInterfaceState,
    },
    DebugProbeType, Probe, WireProtocol,
};
use std::{
    io::{stdin, stdout, Write},
    process,
    time::Instant,
};
use structopt::StructOpt;

mod custom_ap;
use custom_ap::{CtrlAP, ERASEALL, ERASEALLSTATUS, RESET};

#[derive(Debug, StructOpt)]
struct Opt {
    /// Skips confirmation
    #[structopt(long = "yes", short = "y")]
    skip_confirmation: bool,
    #[structopt(name = "Debug probe index", long = "probe-index")]
    probe_index: Option<usize>,
    /// Uses st-link for mass erase, it will not work if the chip is indeed locked
    #[structopt(long = "st-link", short = "s")]
    st_link: bool,
}

fn main() {
    pretty_env_logger::init();
    match main_try() {
        Ok(()) => println!("{}", "Mass erase completed, chip unlocked".green().bold()),
        Err(e) => {
            let mut stderr = std::io::stderr();
            let _ = writeln!(stderr, "       {} {}", "Error".red().bold(), e);
            let _ = stderr.flush();

            process::exit(1);
        }
    }
}

fn main_try() -> Result<()> {
    let opt = Opt::from_args();

    if !opt.skip_confirmation {
        let mut input_buffer = String::new();
        println!(
            "This process will erase the entire code flash and UICR area of the device, \
        \nin addition to the entire RAM. (You can skip this message with the -y flag)"
        );
        print!("Do you want to continue [y/N]: ");
        stdout().flush()?;
        stdin().read_line(&mut input_buffer)?;
        match input_buffer.to_lowercase().chars().next() {
            Some(c) if c == 'y' => {}
            _ => process::exit(0),
        }
    }

    let list = Probe::list_all();

    let device = match opt.probe_index {
        Some(index) => list
            .get(index)
            .ok_or_else(|| anyhow!("Unable to open probe with index {}: Probe not found", index))?,
        None => {
            if list.len() > 1 {
                return Err(anyhow!(
                    "More than a single probe detected. Use the --probe-index 
                argument to select which probe to use."
                ));
            }

            list.first()
                .ok_or_else(|| anyhow!("No supported probe was found"))?
        }
    };

    if device.probe_type == DebugProbeType::STLink && !opt.st_link {
        return Err(anyhow!("It isn't possible to recover with a ST-Link"));
    }

    let mut probe = Probe::open(device)?;
    probe.select_protocol(WireProtocol::Swd)?;
    probe.attach_to_unspecified()?;
    let mut interface_state = ArmCommunicationInterfaceState::new();
    let mut interface = ArmCommunicationInterface::new(&mut probe, &mut interface_state)?
        .ok_or_else(|| anyhow!("Failed to create arm communication interface"))?;
    nrf_recover(&mut interface)?;
    Ok(())
}

const CTRL_AP_IDR: IDR = IDR {
    REVISION: 0,
    DESIGNER: 0x0144,
    CLASS: APClass::Undefined,
    _RES0: 0,
    VARIANT: 0,
    TYPE: APType::JTAG_COM_AP,
};
const UNLOCK_TIMEOUT: u64 = 15;

/// Tries to mass erase a locked nRF52 chip, this process may timeout, if it does, the chip
/// might be unlocked or not, it is advised to try again if flashing fails
fn nrf_recover(probe: &mut ArmCommunicationInterface) -> Result<()> {
    let ctrl_port = match get_ap_by_idr(probe, |idr| idr == CTRL_AP_IDR) {
        Some(port) => CtrlAP::from(port),
        None => {
            return Err(anyhow!("Could not find Nordic's CtrlAP"));
        }
    };
    println!("Starting mass erase...");
    let mut erase_reg = ERASEALL::from(1);
    let status_reg = ERASEALLSTATUS::from(0);
    let mut reset_reg = RESET::from(1);

    // Reset first
    probe.write_ap_register(ctrl_port, reset_reg)?;
    reset_reg.RESET = false;
    probe.write_ap_register(ctrl_port, reset_reg)?;

    probe.write_ap_register(ctrl_port, erase_reg)?;

    // Prepare timeout
    let now = Instant::now();
    let status = probe.read_ap_register(ctrl_port, status_reg)?;
    log::info!("Erase status: {:?}", status.ERASEALLSTATUS);
    let timeout = loop {
        let status = probe.read_ap_register(ctrl_port, status_reg)?;
        if !status.ERASEALLSTATUS {
            break false;
        }
        if now.elapsed().as_secs() >= UNLOCK_TIMEOUT {
            break true;
        }
    };
    reset_reg.RESET = true;
    probe.write_ap_register(ctrl_port, reset_reg)?;
    reset_reg.RESET = false;
    probe.write_ap_register(ctrl_port, reset_reg)?;
    erase_reg.ERASEALL = false;
    probe.write_ap_register(ctrl_port, erase_reg)?;
    if timeout {
        Err(anyhow!(
            "    {} Mass erase process timed out, the chip might still be locked.",
            "Error".red().bold()
        ))
    } else {
        Ok(())
    }
}
