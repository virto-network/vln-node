// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use crate::service::{new_partial, Block, KreivoRuntimeExecutor, VirtoRuntimeExecutor};
use crate::{
	chain_spec,
	cli::{Cli, RelayChainCli, Subcommand},
};

use codec::Encode;
use cumulus_client_cli::generate_genesis_block;
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use log;
use log::{info, warn};
use parachains_common::AuraId;
use sc_cli::{
	ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams, NetworkParams, Result,
	RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::traits::{AccountIdConversion, Block as BlockT};
use std::{net::SocketAddr, path::PathBuf};

/// Helper enum that is used for better distinction of different
/// parachain/runtime configuration (it is based/calculated on ChainSpec's ID
/// attribute)
#[derive(Debug, PartialEq, Default)]
enum Runtime {
	/// This is the default runtime (actually based on rococo)
	#[default]
	Default,
	Kreivo,
	Virto,
	Seedling,
}

trait RuntimeResolver {
	fn runtime(&self) -> Runtime;
}

impl RuntimeResolver for dyn ChainSpec {
	fn runtime(&self) -> Runtime {
		runtime(self.id())
	}
}

/// Implementation, that can resolve [`Runtime`] from any json configuration
/// file
impl RuntimeResolver for PathBuf {
	fn runtime(&self) -> Runtime {
		#[derive(Debug, serde::Deserialize)]
		struct EmptyChainSpecWithId {
			id: String,
		}

		let file = std::fs::File::open(self).expect("Failed to open file");
		let reader = std::io::BufReader::new(file);
		let chain_spec: EmptyChainSpecWithId =
			serde_json::from_reader(reader).expect("Failed to read 'json' file with ChainSpec configuration");

		runtime(&chain_spec.id)
	}
}

fn runtime(id: &str) -> Runtime {
	let id = id.replace("_", "-");
	if id.starts_with("seedling") {
		Runtime::Seedling
	} else if id.starts_with("kreivo") {
		Runtime::Kreivo
	} else if id.starts_with("virto") {
		Runtime::Virto
	} else {
		log::warn!(
			"No specific runtime was recognized for ChainSpec's id: '{}', so Runtime::default() will be used",
			id
		);
		Runtime::default()
	}
}

fn load_spec(id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
	Ok(match id {
		// - Defaul-like
		"kreivo" => Box::new(chain_spec::kreivo::kreivo_kusama_chain_spec()),
		"kreivo-local" => Box::new(chain_spec::kreivo::kreivo_kusama_chain_spec_local()),
		"kreivo-rococo-local" => Box::new(chain_spec::kreivo::kreivo_rococo_chain_spec_local()),
		"virto" => Box::new(chain_spec::virto::virto_polkadot_chain_spec()),
		"virto-local" => Box::new(chain_spec::virto::virto_polkadot_chain_spec_local()),
		"seedling-rococo" => Box::new(chain_spec::seedling::seedling_rococo_chain_spec()),
		"seedling-rococo-local" => Box::new(chain_spec::seedling::seedling_rococo_chain_spec_local()),
		"seedling-kusama" => Box::new(chain_spec::seedling::seedling_kusama_chain_spec()),
		"seedling-kusama-local" => Box::new(chain_spec::seedling::seedling_kusama_chain_spec_local()),
		// -- Fallback (generic chainspec)
		"" => {
			log::warn!("No ChainSpec.id specified, so using default one, based on rococo-parachain runtime");
			Box::new(chain_spec::kreivo::kreivo_kusama_chain_spec_local())
		}
		// -- Loading a specific spec from disk
		path => {
			let path: PathBuf = path.into();
			match path.runtime() {
				Runtime::Kreivo => Box::new(chain_spec::kreivo::ChainSpec::from_json_file(path)?),
				Runtime::Default | Runtime::Virto => Box::new(chain_spec::virto::ChainSpec::from_json_file(path)?),
				Runtime::Seedling => Box::new(chain_spec::seedling::ChainSpec::from_json_file(path)?),
			}
		}
	})
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Virto parachain".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Virto parachain\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relaychain node.\n\n\
		{} [parachain-args] -- [relaychain-args]",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/paritytech/cumulus/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
		load_spec(id)
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		match chain_spec.runtime() {
			Runtime::Kreivo => &kreivo_runtime::VERSION,
			Runtime::Default | Runtime::Virto => &virto_runtime::VERSION,
			Runtime::Seedling => &seedling_runtime::VERSION,
		}
	}
}

impl SubstrateCli for RelayChainCli {
	fn impl_name() -> String {
		"Virto parachain".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Virto parachain\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		{} [parachain-args] -- [relay_chain-args]",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/paritytech/cumulus/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
		polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		polkadot_cli::Cli::native_runtime_version(chain_spec)
	}
}

/// Creates partial components for the runtimes that are supported by the
/// benchmarks.
macro_rules! construct_benchmark_partials {
	($config:expr, |$partials:ident| $code:expr) => {
		match $config.chain_spec.runtime() {
			Runtime::Seedling => {
				let $partials = new_partial::<seedling_runtime::RuntimeApi, _>(
					&$config,
					crate::service::aura_build_import_queue::<_, AuraId>,
				)?;
				$code
			}
			Runtime::Kreivo => {
				let $partials = new_partial::<kreivo_runtime::RuntimeApi, _>(
					&$config,
					crate::service::aura_build_import_queue::<_, AuraId>,
				)?;
				$code
			}
			Runtime::Default | Runtime::Virto => {
				let $partials = new_partial::<virto_runtime::RuntimeApi, _>(
					&$config,
					crate::service::aura_build_import_queue::<_, AuraId>,
				)?;
				$code
			}
		}
	};
}

macro_rules! construct_async_run {
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
		let runner = $cli.create_runner($cmd)?;
		match runner.config().chain_spec.runtime() {
			Runtime::Seedling => {
				runner.async_run(|$config| {
					let $components = new_partial::<seedling_runtime::RuntimeApi, _>(
						&$config,
						crate::service::aura_build_import_queue::<_, AuraId>,
					)?;
					let task_manager = $components.task_manager;
					{ $( $code )* }.map(|v| (v, task_manager))
				})
			},
			Runtime::Kreivo => {
				runner.async_run(|$config| {
					let $components = new_partial::<kreivo_runtime::RuntimeApi, _>(
						&$config,
						crate::service::aura_build_import_queue::<_, AuraId>,
					)?;
					let task_manager = $components.task_manager;
					{ $( $code )* }.map(|v| (v, task_manager))
				})
			},
			Runtime::Default | Runtime::Virto => {
				runner.async_run(|$config| {
					let $components = new_partial::<virto_runtime::RuntimeApi, _>(
						&$config,
						crate::service::aura_build_import_queue::<_, AuraId>,
					)?;
					let task_manager = $components.task_manager;
					{ $( $code )* }.map(|v| (v, task_manager))
				})
			}
		}
	}}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		}
		Some(Subcommand::CheckBlock(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		}
		Some(Subcommand::ExportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| Ok(cmd.run(components.client, config.database)))
		}
		Some(Subcommand::ExportState(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| Ok(cmd.run(components.client, config.chain_spec)))
		}
		Some(Subcommand::ImportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| Ok(cmd.run(components.client, components.import_queue)))
		}
		Some(Subcommand::Revert(cmd)) => construct_async_run!(|components, cli, cmd, config| Ok(cmd.run(
			components.client,
			components.backend,
			None
		))),
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()]
						.iter()
						.chain(cli.relaychain_args.iter()),
				);

				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, config.tokio_handle.clone())
						.map_err(|err| format!("Relay chain argument error: {}", err))?;

				cmd.run(config, polkadot_config)
			})
		}
		Some(Subcommand::ExportGenesisState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				let state_version = Cli::native_runtime_version(&spec).state_version();
				cmd.run::<crate::service::Block>(&*spec, state_version)
			})
		}
		Some(Subcommand::ExportGenesisWasm(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				cmd.run(&*spec)
			})
		}
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			// Switch on the concrete benchmark sub-command-
			match cmd {
				BenchmarkCmd::Pallet(cmd) => {
					if cfg!(feature = "runtime-benchmarks") {
						runner.sync_run(|config| match config.chain_spec.runtime() {
							Runtime::Kreivo => cmd.run::<Block, KreivoRuntimeExecutor>(config),
							Runtime::Virto => cmd.run::<Block, VirtoRuntimeExecutor>(config),
							_ => Err(
								format!("Chain '{:?}' doesn't support benchmarking", config.chain_spec.runtime())
									.into(),
							),
						})
					} else {
						Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
							.into())
					}
				}
				BenchmarkCmd::Block(cmd) => {
					runner.sync_run(|config| construct_benchmark_partials!(config, |partials| cmd.run(partials.client)))
				}
				#[cfg(not(feature = "runtime-benchmarks"))]
				BenchmarkCmd::Storage(_) => {
					return Err(sc_cli::Error::Input(
						"Compile with --features=runtime-benchmarks \
						to enable storage benchmarks."
							.into(),
					)
					.into())
				}
				#[cfg(feature = "runtime-benchmarks")]
				BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
					construct_benchmark_partials!(config, |partials| {
						let db = partials.backend.expose_db();
						let storage = partials.backend.expose_storage();

						cmd.run(config, partials.client.clone(), db, storage)
					})
				}),
				BenchmarkCmd::Machine(cmd) => {
					runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
				}
				// NOTE: this allows the Client to leniently implement
				// new benchmark commands without requiring a companion MR.
				#[allow(unreachable_patterns)]
				_ => Err("Benchmarking sub-command unsupported".into()),
			}
		}
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
			use try_runtime_cli::block_building_info::timestamp_with_aura_info;

			// grab the task manager.
			let runner = cli.create_runner(cmd)?;
			let registry = &runner.config().prometheus_config.as_ref().map(|cfg| &cfg.registry);
			let task_manager = sc_service::TaskManager::new(runner.config().tokio_handle.clone(), *registry)
				.map_err(|e| format!("Error: {:?}", e))?;
			type HostFunctionsOf<E> = ExtendedHostFunctions<
				sp_io::SubstrateHostFunctions,
				<E as NativeExecutionDispatch>::ExtendHostFunctions,
			>;

			let info_provider = timestamp_with_aura_info(6000);

			match runner.config().chain_spec.runtime() {
				Runtime::Kreivo => runner.async_run(|_| {
					Ok((
						cmd.run::<Block, HostFunctionsOf<KreivoRuntimeExecutor>, _>(Some(info_provider)),
						task_manager,
					))
				}),
				Runtime::Virto => runner.async_run(|_| {
					Ok((
						cmd.run::<Block, HostFunctionsOf<VirtoRuntimeExecutor>, _>(Some(info_provider)),
						task_manager,
					))
				}),
				Runtime::Seedling => runner.async_run(|_| {
					Ok((
						cmd.run::<Block, HostFunctionsOf<crate::service::SeedlingRuntimeExecutor>, _>(Some(
							info_provider,
						)),
						task_manager,
					))
				}),
				_ => Err("Chain doesn't support try-runtime".into()),
			}
		}
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("Try-runtime was not enabled when building the node. \
			You can enable it with `--features try-runtime`."
			.into()),
		Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),
		None => {
			let runner = cli.create_runner(&cli.run.normalize())?;
			let collator_options = cli.run.collator_options();

			runner.run_node_until_exit(|config| async move {
				let hwbench = (!cli.no_hardware_benchmarks).then_some(
					config.database.path().map(|database_path| {
						let _ = std::fs::create_dir_all(&database_path);
						sc_sysinfo::gather_hwbench(Some(database_path))
					})).flatten();


				let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
					.map(|e| e.para_id)
					.ok_or_else(|| "Could not find parachain extension in chain-spec.")?;

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relaychain_args.iter()),
				);

				let id = ParaId::from(para_id);

				let parachain_account =
					AccountIdConversion::<polkadot_primitives::AccountId>::into_account_truncating(&id);

				let state_version = Cli::native_runtime_version(&config.chain_spec).state_version();

				let block: crate::service::Block =
					generate_genesis_block(&*config.chain_spec, state_version)
						.map_err(|e| format!("{:?}", e))?;
				let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));

				let tokio_handle = config.tokio_handle.clone();
				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
						.map_err(|err| format!("Relay chain argument error: {}", err))?;

				info!("Parachain id: {:?}", id);
				info!("Parachain Account: {}", parachain_account);
				info!("Parachain genesis state: {}", genesis_state);
				info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

				if !collator_options.relay_chain_rpc_urls.is_empty() && cli.relaychain_args.len() > 0 {
					warn!("Detected relay chain node arguments together with --relay-chain-rpc-url. This command starts a minimal Polkadot node that only uses a network-related subset of all relay chain CLI options.");
				}

				match config.chain_spec.runtime() {
					Runtime::Kreivo => crate::service::start_generic_aura_node::<
						kreivo_runtime::RuntimeApi,
						AuraId,
					>(config, polkadot_config, collator_options, id, hwbench)
					.await
					.map(|r| r.0)
					.map_err(Into::into),
					Runtime::Default | Runtime::Virto => crate::service::start_generic_aura_node::<
						virto_runtime::RuntimeApi,
						AuraId,
					>(config, polkadot_config, collator_options, id, hwbench)
					.await
					.map(|r| r.0)
					.map_err(Into::into),
					Runtime::Seedling => crate::service::start_generic_aura_node::<
						seedling_runtime::RuntimeApi,
						AuraId,
					>(config, polkadot_config, collator_options, id, hwbench)
						.await
						.map(|r| r.0)
						.map_err(Into::into)
				}
			})
		}
	}
}

impl DefaultConfigurationValues for RelayChainCli {
	fn p2p_listen_port() -> u16 {
		30334
	}

	fn rpc_listen_port() -> u16 {
		9945
	}

	fn prometheus_listen_port() -> u16 {
		9616
	}
}

impl CliConfiguration<Self> for RelayChainCli {
	fn shared_params(&self) -> &SharedParams {
		self.base.base.shared_params()
	}

	fn import_params(&self) -> Option<&ImportParams> {
		self.base.base.import_params()
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		self.base.base.network_params()
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		self.base.base.keystore_params()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(self
			.shared_params()
			.base_path()?
			.or_else(|| self.base_path.clone().map(Into::into)))
	}

	fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_addr(default_listen_port)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base.base.prometheus_config(default_listen_port, chain_spec)
	}

	fn init<F>(
		&self,
		_support_url: &String,
		_impl_version: &String,
		_logger_hook: F,
		_config: &sc_service::Configuration,
	) -> Result<()>
	where
		F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
	{
		unreachable!("PolkadotCli is never initialized; qed");
	}

	fn chain_id(&self, is_dev: bool) -> Result<String> {
		let chain_id = self.base.base.chain_id(is_dev)?;

		Ok(if chain_id.is_empty() {
			self.chain_id.clone().unwrap_or_default()
		} else {
			chain_id
		})
	}

	fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
		self.base.base.role(is_dev)
	}

	fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
		self.base.base.transaction_pool(is_dev)
	}

	fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
		self.base.base.trie_cache_maximum_size()
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.base.rpc_methods()
	}

	fn rpc_max_connections(&self) -> Result<u32> {
		self.base.base.rpc_max_connections()
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		self.base.base.rpc_cors(is_dev)
	}

	fn default_heap_pages(&self) -> Result<Option<u64>> {
		self.base.base.default_heap_pages()
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.base.force_authoring()
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.base.disable_grandpa()
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.base.max_runtime_instances()
	}

	fn announce_block(&self) -> Result<bool> {
		self.base.base.announce_block()
	}

	fn telemetry_endpoints(&self, chain_spec: &Box<dyn ChainSpec>) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
		self.base.base.telemetry_endpoints(chain_spec)
	}

	fn node_name(&self) -> Result<String> {
		self.base.base.node_name()
	}
}

// TODO - Bring tests back if useful
