// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
use std::{cmp::min, env};

use crate::{
    cluster::Cluster,
    experiments::{
        CpuFlamegraphParams, Experiment, ExperimentParam, PerformanceBenchmarkParams,
        PerformanceBenchmarkThreeRegionSimulationParams, RebootRandomValidatorsParams,
        RecoveryTimeParams,
    },
};
use anyhow::{format_err, Result};

pub struct ExperimentSuite {
    pub experiments: Vec<Box<dyn Experiment>>,
}

impl ExperimentSuite {
    fn new_pre_release(cluster: &Cluster) -> Self {
        let mut experiments: Vec<Box<dyn Experiment>> = vec![];
        if env::var("RECOVERY_EXP").is_ok() {
            experiments.push(Box::new(
                RecoveryTimeParams {
                    num_accounts_to_mint: 100_000,
                }
                .build(cluster),
            ));
        }
        let count = min(3, cluster.validator_instances().len() / 3);
        // Reboot different sets of 3 validators *100 times
        for _ in 0..10 {
            let b = Box::new(RebootRandomValidatorsParams { count }.build(cluster));
            experiments.push(b);
        }
        experiments.push(Box::new(
            PerformanceBenchmarkParams::new_nodes_down(0).build(cluster),
        ));
        experiments.push(Box::new(
            PerformanceBenchmarkParams::new_nodes_down(10).build(cluster),
        ));
        experiments.push(Box::new(
            PerformanceBenchmarkParams::new_fixed_tps(0, 10).build(cluster),
        ));
        experiments.push(Box::new(
            PerformanceBenchmarkThreeRegionSimulationParams {}.build(cluster),
        ));
        experiments.push(Box::new(
            CpuFlamegraphParams { duration_secs: 60 }.build(cluster),
        ));
        Self { experiments }
    }

    fn new_perf_suite(cluster: &Cluster) -> Self {
        let mut experiments: Vec<Box<dyn Experiment>> = vec![];
        experiments.push(Box::new(
            PerformanceBenchmarkParams::new_nodes_down(0).build(cluster),
        ));
        experiments.push(Box::new(
            PerformanceBenchmarkParams::new_nodes_down(10).build(cluster),
        ));
        experiments.push(Box::new(
            PerformanceBenchmarkParams::new_fixed_tps(0, 10).build(cluster),
        ));
        experiments.push(Box::new(
            PerformanceBenchmarkThreeRegionSimulationParams {}.build(cluster),
        ));
        Self { experiments }
    }

    fn new_land_blocking_suite(cluster: &Cluster) -> Self {
        let mut experiments: Vec<Box<dyn Experiment>> = vec![];
        experiments.push(Box::new(
            PerformanceBenchmarkParams::new_nodes_down(0).build(cluster),
        ));
        Self { experiments }
    }

    pub fn new_by_name(cluster: &Cluster, name: &str) -> Result<Self> {
        match name {
            "perf" => Ok(Self::new_perf_suite(cluster)),
            "pre_release" => Ok(Self::new_pre_release(cluster)),
            "land_blocking" => Ok(Self::new_land_blocking_suite(cluster)),
            other => Err(format_err!("Unknown suite: {}", other)),
        }
    }
}
