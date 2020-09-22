use {
    super::*,
    crate::errors::ConfError,
};

/// A verb execution definition based on an internal
#[derive(Debug, Clone)]
pub struct InternalExecution {

    /// the internal to use
    pub internal: Internal,

    /// whether to open the resulting state in a new panel
    /// instead of the current ones
    pub bang: bool,

    /// arguments
    /// (for example `"~"` when a verb execution is `:!focus ~`)
    pub arg: Option<String>,
}

impl InternalExecution {
    pub fn from_internal(internal: Internal) -> Self {
        Self {
            internal,
            bang: false,
            arg: None,
        }
    }
    pub fn from_internal_bang(internal: Internal, bang: bool) -> Self {
        Self {
            internal,
            bang,
            arg: None,
        }
    }
    pub fn try_from(invocation_str: &str) -> Result<Self, ConfError> {
        let invocation = VerbInvocation::from(invocation_str);
        let internal = Internal::try_from(&invocation.name)?;
        Ok(Self {
            internal,
            bang: invocation.bang,
            arg: invocation.args,
        })
    }
    pub fn as_desc_code(&self) -> Option<String> {
        self.arg.as_ref().map(|arg| {
            format!(
                ":{}{} {}",
                self.internal.name(),
                if self.bang { "!" } else { "" },
                arg
            )
        })
    }
}
