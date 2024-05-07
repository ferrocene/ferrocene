use std::ffi::OsString;

use rustc_data_structures::fx::FxHashMap;

use crate::*;
use shims::{unix::UnixEnvVars, windows::WindowsEnvVars};

#[derive(Default)]
pub enum EnvVars<'tcx> {
    #[default]
    Uninit,
    Unix(UnixEnvVars<'tcx>),
    Windows(WindowsEnvVars),
}

impl VisitProvenance for EnvVars<'_> {
    fn visit_provenance(&self, visit: &mut VisitWith<'_>) {
        match self {
            EnvVars::Uninit => {}
            EnvVars::Unix(env) => env.visit_provenance(visit),
            EnvVars::Windows(env) => env.visit_provenance(visit),
        }
    }
}

impl<'tcx> EnvVars<'tcx> {
    pub(crate) fn init<'mir>(
        ecx: &mut InterpCx<'mir, 'tcx, MiriMachine<'mir, 'tcx>>,
        config: &MiriConfig,
    ) -> InterpResult<'tcx> {
        // Initialize the `env_vars` map.
        // Skip the loop entirely if we don't want to forward anything.
        let mut env_vars = FxHashMap::default();
        if ecx.machine.communicate() || !config.forwarded_env_vars.is_empty() {
            for (name, value) in &config.env {
                let forward = ecx.machine.communicate()
                    || config.forwarded_env_vars.iter().any(|v| **v == *name);
                if forward {
                    env_vars.insert(OsString::from(name), OsString::from(value));
                }
            }
        }

        for (name, value) in &config.set_env_vars {
            env_vars.insert(OsString::from(name), OsString::from(value));
        }

        let env_vars = if ecx.target_os_is_unix() {
            EnvVars::Unix(UnixEnvVars::new(ecx, env_vars)?)
        } else if ecx.tcx.sess.target.os == "windows" {
            EnvVars::Windows(WindowsEnvVars::new(ecx, env_vars)?)
        } else {
            // Used e.g. for wasi
            EnvVars::Uninit
        };
        ecx.machine.env_vars = env_vars;

        Ok(())
    }

    pub(crate) fn cleanup<'mir>(
        ecx: &mut InterpCx<'mir, 'tcx, MiriMachine<'mir, 'tcx>>,
    ) -> InterpResult<'tcx> {
        let this = ecx.eval_context_mut();
        match this.machine.env_vars {
            EnvVars::Unix(_) => UnixEnvVars::cleanup(this),
            EnvVars::Windows(_) => Ok(()), // no cleanup needed
            EnvVars::Uninit => Ok(()),
        }
    }

    pub(crate) fn unix(&self) -> &UnixEnvVars<'tcx> {
        match self {
            EnvVars::Unix(env) => env,
            _ => unreachable!(),
        }
    }

    pub(crate) fn unix_mut(&mut self) -> &mut UnixEnvVars<'tcx> {
        match self {
            EnvVars::Unix(env) => env,
            _ => unreachable!(),
        }
    }

    pub(crate) fn windows(&self) -> &WindowsEnvVars {
        match self {
            EnvVars::Windows(env) => env,
            _ => unreachable!(),
        }
    }

    pub(crate) fn windows_mut(&mut self) -> &mut WindowsEnvVars {
        match self {
            EnvVars::Windows(env) => env,
            _ => unreachable!(),
        }
    }
}

impl<'mir, 'tcx: 'mir> EvalContextExt<'mir, 'tcx> for crate::MiriInterpCx<'mir, 'tcx> {}
pub trait EvalContextExt<'mir, 'tcx: 'mir>: crate::MiriInterpCxExt<'mir, 'tcx> {}
