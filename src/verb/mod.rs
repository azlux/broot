mod builtin;
mod execution_builder;
mod external_execution;
mod external_execution_mode;
mod internal;
mod internal_execution;
pub mod internal_focus;
mod invocation_parser;
mod verb;
mod verb_conf;
mod verb_description;
mod verb_execution;
mod verb_invocation;
mod verb_store;

pub use {
    execution_builder::ExecutionStringBuilder,
    external_execution::ExternalExecution,
    external_execution_mode::ExternalExecutionMode,
    internal::Internal,
    internal_execution::InternalExecution,
    invocation_parser::InvocationParser,
    verb::Verb,
    verb_conf::VerbConf,
    verb_description::VerbDescription,
    verb_execution::VerbExecution,
    verb_invocation::VerbInvocation,
    verb_store::{PrefixSearchResult, VerbStore},
};


// the group you find in invocation patterns and execution patterns
lazy_static! {
    pub static ref GROUP: regex::Regex = regex::Regex::new(r"\{([^{}:]+)(?::([^{}:]+))?\}").unwrap();
}
